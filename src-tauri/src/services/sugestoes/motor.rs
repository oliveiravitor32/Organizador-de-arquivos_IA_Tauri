//! Motor de sugestões de organização (UC-005, ADR-025).
//!
//! Pipeline: carregar clusters → filtrar oportunidades → nomear via LLM (cap 15) → persistir.

use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use bytemuck::cast_slice;
use sqlx::SqlitePool;
use tauri::{AppHandle, Emitter};

use crate::db::repositories::clusters::ClustersRepository;
use crate::db::repositories::embeddings::EmbeddingsRepository;
use crate::db::repositories::files::FileRepository;
use crate::db::repositories::sugestoes::SugestoesRepository;
use crate::domain::sugestoes::{SugestaoStats, SugestaoTipo};
use crate::error::AppResult;
use crate::events;
use crate::services::ia::ServicoIa;

/// Cap de arquivos enviados ao LLM para nomeação (ADR-025).
pub const MAX_FILES_LLM_NAMING: usize = 15;

/// Limiar mínimo de confiança para gerar sugestão (ADR-024).
pub const MIN_SUGGESTION_CONFIDENCE: f64 = 0.50;

/// Modelo de embedding padrão (ADR-018).
const EMBED_MODEL: &str = "nomic-embed-text";

pub struct SugestaoMotorService {
    pool: SqlitePool,
    ia: Arc<dyn ServicoIa>,
}

impl SugestaoMotorService {
    pub fn new(pool: SqlitePool, ia: Arc<dyn ServicoIa>) -> Self {
        Self { pool, ia }
    }

    /// Executa o pipeline e emite eventos via `AppHandle`.
    pub async fn processar(
        &self,
        app: &AppHandle,
        generation_id: &str,
    ) -> AppResult<SugestaoStats> {
        let clusters_repo = ClustersRepository::new(&self.pool);
        let total = clusters_repo.find_clusters().await?.len();

        let _ = app.emit(
            events::SUGGESTION_STARTED,
            serde_json::json!({ "suggestionGenerationId": generation_id, "total": total }),
        );

        let (stats, criadas) = self.executar_pipeline().await?;

        for (sid, titulo, confianca) in &criadas {
            let _ = app.emit(
                events::SUGGESTION_CREATED,
                serde_json::json!({
                    "suggestionId": sid,
                    "titulo": titulo,
                    "confianca": confianca,
                }),
            );
        }

        let _ = app.emit(
            events::SUGGESTION_COMPLETED,
            serde_json::json!({ "suggestionGenerationId": generation_id, "stats": stats }),
        );

        Ok(stats)
    }

    /// Pipeline puro — sem efeitos colaterais de eventos. Retorna stats + sugestões criadas.
    pub async fn executar_pipeline(
        &self,
    ) -> AppResult<(SugestaoStats, Vec<(String, String, f64)>)> {
        let inicio = Instant::now();

        let clusters_repo = ClustersRepository::new(&self.pool);
        let embeddings_repo = EmbeddingsRepository::new(&self.pool);
        let files_repo = FileRepository::new(&self.pool);
        let sugestoes_repo = SugestoesRepository::new(&self.pool);

        let clusters = clusters_repo.find_clusters().await?;

        let mut geradas: u64 = 0;
        let mut descartadas: u64 = 0;
        let mut criadas: Vec<(String, String, f64)> = Vec::new();

        for cluster in &clusters {
            let confianca = cluster.confidence.unwrap_or(0.0);

            if confianca < MIN_SUGGESTION_CONFIDENCE {
                descartadas += 1;
                continue;
            }

            let file_ids = clusters_repo.find_members_by_cluster(&cluster.id).await?;

            if file_ids.len() < 2 {
                descartadas += 1;
                continue;
            }

            // Verifica se os arquivos estão em diretórios distintos
            let mut dirs: HashSet<String> = HashSet::new();
            let mut nomes: Vec<String> = Vec::new();

            for fid in &file_ids {
                if let Some(record) = files_repo.find_by_id(fid).await? {
                    let dir = Path::new(&record.path)
                        .parent()
                        .and_then(|p| p.to_str())
                        .unwrap_or("")
                        .to_string();
                    dirs.insert(dir);
                    nomes.push(record.name);
                }
            }

            if dirs.len() < 2 {
                // Todos no mesmo diretório — sem oportunidade de agrupamento
                descartadas += 1;
                continue;
            }

            // Seleciona até MAX_FILES_LLM_NAMING arquivos mais próximos do centróide
            let nomes_para_llm = if nomes.len() <= MAX_FILES_LLM_NAMING {
                nomes.clone()
            } else {
                self.selecionar_por_centroide(&embeddings_repo, &file_ids, &nomes)
                    .await
                    .unwrap_or_else(|_| nomes[..MAX_FILES_LLM_NAMING].to_vec())
            };

            // Gera título via LLM (fallback por template se indisponível)
            let titulo = match self.ia.gerar_nome_cluster(nomes_para_llm).await {
                Ok(nome) => nome,
                Err(_) => format!("Grupo semântico com {} arquivos", file_ids.len()),
            };

            let dirs_count = dirs.len();
            let evidencias = serde_json::json!([
                { "tipo": "arquivos_no_cluster", "valor": file_ids.len().to_string() },
                { "tipo": "similaridade_media", "valor": format!("{:.2}", confianca) },
                { "tipo": "diretorios_distintos", "valor": dirs_count.to_string() }
            ]);

            let descricao = format!(
                "{} arquivos com similaridade semântica média de {:.2} foram identificados \
                 no mesmo agrupamento e estão distribuídos em {} diretório(s) distinto(s).",
                file_ids.len(),
                confianca,
                dirs_count
            );

            let sid = sugestoes_repo
                .insert_suggestion(
                    SugestaoTipo::Agrupamento.as_str(),
                    Some(&titulo),
                    Some(&descricao),
                    confianca,
                    Some(&cluster.id),
                    Some(&evidencias.to_string()),
                )
                .await?;

            // Persiste operação de agrupamento para o Marco 4
            let op_payload = serde_json::json!({
                "file_ids": file_ids,
                "cluster_id": cluster.id,
            });
            sugestoes_repo
                .insert_suggestion_operation(&sid, "mover_arquivo", &op_payload.to_string())
                .await?;

            criadas.push((sid, titulo, confianca));
            geradas += 1;
        }

        let duration_ms = inicio.elapsed().as_millis() as u64;
        let stats = SugestaoStats {
            geradas,
            descartadas,
            duration_ms,
        };

        Ok((stats, criadas))
    }

    /// Seleciona os MAX_FILES_LLM_NAMING arquivos mais próximos do centróide semântico.
    async fn selecionar_por_centroide(
        &self,
        embeddings_repo: &EmbeddingsRepository<'_>,
        file_ids: &[String],
        nomes: &[String],
    ) -> AppResult<Vec<String>> {
        let mut vetores: Vec<Vec<f32>> = Vec::new();
        let mut indices_validos: Vec<usize> = Vec::new();

        for (i, fid) in file_ids.iter().enumerate() {
            if let Some(emb) = embeddings_repo
                .find_embedding_by_file(fid, EMBED_MODEL)
                .await?
            {
                let v: Vec<f32> = cast_slice(&emb.vector).to_vec();
                vetores.push(v);
                indices_validos.push(i);
            }
        }

        if vetores.is_empty() {
            return Ok(nomes[..MAX_FILES_LLM_NAMING].to_vec());
        }

        let dim = vetores[0].len();
        let mut centroide = vec![0.0f32; dim];
        for v in &vetores {
            for (j, val) in v.iter().enumerate() {
                centroide[j] += val;
            }
        }
        let n = vetores.len() as f32;
        centroide.iter_mut().for_each(|x| *x /= n);

        // Ordena por similaridade cosseno com o centróide (decrescente)
        let mut scores: Vec<(usize, f32)> = vetores
            .iter()
            .zip(indices_validos.iter())
            .map(|(v, &orig_idx)| (orig_idx, cosine_sim(v, &centroide)))
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let selecionados: Vec<String> = scores
            .into_iter()
            .take(MAX_FILES_LLM_NAMING)
            .map(|(idx, _)| nomes[idx].clone())
            .collect();

        Ok(selecionados)
    }
}

fn cosine_sim(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a * norm_b)
}

// ─── Testes ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::repositories::clusters::ClustersRepository;
    use crate::db::repositories::sugestoes::SugestoesRepository;
    use crate::services::ia::MockServicoIa;
    use sqlx::sqlite::SqlitePoolOptions;

    // ─── Helpers ──────────────────────────────────────────────────────────────

    async fn setup_pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    async fn inserir_arquivo(pool: &SqlitePool, id: &str, path: &str, nome: &str) {
        sqlx::query(
            "INSERT INTO files (id, path, relative_path, name, status) VALUES (?, ?, ?, ?, 'discovered')",
        )
        .bind(id)
        .bind(path)
        .bind(nome)
        .bind(nome)
        .execute(pool)
        .await
        .unwrap();
    }

    fn mock_ia_com_titulo(titulo: &'static str) -> Arc<dyn ServicoIa> {
        let mut mock = MockServicoIa::new();
        mock.expect_gerar_nome_cluster()
            .returning(move |_| Box::pin(async move { Ok(titulo.to_string()) }));
        Arc::new(mock)
    }

    fn mock_ia_com_erro() -> Arc<dyn ServicoIa> {
        let mut mock = MockServicoIa::new();
        mock.expect_gerar_nome_cluster().returning(|_| {
            Box::pin(async { Err(crate::error::AppError::Internal("ollama offline".into())) })
        });
        Arc::new(mock)
    }

    // ─── Cosine utility tests ─────────────────────────────────────────────────

    #[test]
    fn cosine_sim_vetores_identicos() {
        let v = vec![1.0f32, 0.0, 0.0];
        assert!((cosine_sim(&v, &v) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn cosine_sim_vetores_ortogonais() {
        let a = vec![1.0f32, 0.0];
        let b = vec![0.0f32, 1.0];
        assert!(cosine_sim(&a, &b).abs() < 1e-5);
    }

    #[test]
    fn cosine_sim_vetor_zero_retorna_zero() {
        let a = vec![0.0f32, 0.0];
        let b = vec![1.0f32, 0.0];
        assert_eq!(cosine_sim(&a, &b), 0.0);
    }

    #[test]
    fn min_confidence_e_050() {
        assert_eq!(MIN_SUGGESTION_CONFIDENCE, 0.50);
    }

    #[test]
    fn max_files_llm_naming_e_15() {
        assert_eq!(MAX_FILES_LLM_NAMING, 15);
    }

    // ─── Pipeline integration tests ───────────────────────────────────────────

    #[tokio::test]
    async fn motor_gera_sugestao_para_cluster_disperso() {
        let pool = setup_pool().await;
        let clusters_repo = ClustersRepository::new(&pool);

        // 2 arquivos em diretórios diferentes
        inserir_arquivo(&pool, "f1", "/docs/contrato.pdf", "contrato.pdf").await;
        inserir_arquivo(&pool, "f2", "/projetos/cronograma.xlsx", "cronograma.xlsx").await;

        let cid = clusters_repo
            .upsert_cluster("Cluster A", None, 0.85)
            .await
            .unwrap();
        clusters_repo
            .add_cluster_member_file(&cid, "f1")
            .await
            .unwrap();
        clusters_repo
            .add_cluster_member_file(&cid, "f2")
            .await
            .unwrap();

        let motor =
            SugestaoMotorService::new(pool.clone(), mock_ia_com_titulo("Documentos do Projeto"));
        let (stats, criadas) = motor.executar_pipeline().await.unwrap();

        assert_eq!(stats.geradas, 1);
        assert_eq!(stats.descartadas, 0);
        assert_eq!(criadas.len(), 1);
        assert_eq!(criadas[0].1, "Documentos do Projeto");
        assert_eq!(criadas[0].2, 0.85);

        // Verifica persistência
        let sugestoes_repo = SugestoesRepository::new(&pool);
        let s = sugestoes_repo.find_suggestions(None).await.unwrap();
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].titulo.as_deref(), Some("Documentos do Projeto"));
        assert_eq!(s[0].cluster_id.as_deref(), Some(cid.as_str()));
    }

    #[tokio::test]
    async fn motor_descarta_cluster_mesmo_diretorio() {
        let pool = setup_pool().await;
        let clusters_repo = ClustersRepository::new(&pool);

        // 2 arquivos no MESMO diretório — sem oportunidade de agrupamento
        inserir_arquivo(&pool, "f1", "/docs/a.pdf", "a.pdf").await;
        inserir_arquivo(&pool, "f2", "/docs/b.pdf", "b.pdf").await;

        let cid = clusters_repo
            .upsert_cluster("Mesmo Dir", None, 0.9)
            .await
            .unwrap();
        clusters_repo
            .add_cluster_member_file(&cid, "f1")
            .await
            .unwrap();
        clusters_repo
            .add_cluster_member_file(&cid, "f2")
            .await
            .unwrap();

        let motor = SugestaoMotorService::new(pool.clone(), mock_ia_com_titulo("X"));
        let (stats, criadas) = motor.executar_pipeline().await.unwrap();

        assert_eq!(stats.geradas, 0);
        assert_eq!(stats.descartadas, 1);
        assert!(criadas.is_empty());
    }

    #[tokio::test]
    async fn motor_descarta_cluster_baixa_confianca() {
        let pool = setup_pool().await;
        let clusters_repo = ClustersRepository::new(&pool);

        inserir_arquivo(&pool, "f1", "/a/x.pdf", "x.pdf").await;
        inserir_arquivo(&pool, "f2", "/b/y.pdf", "y.pdf").await;

        // Confiança 0.30 — abaixo do limiar (ADR-024)
        let cid = clusters_repo
            .upsert_cluster("Fraco", None, 0.30)
            .await
            .unwrap();
        clusters_repo
            .add_cluster_member_file(&cid, "f1")
            .await
            .unwrap();
        clusters_repo
            .add_cluster_member_file(&cid, "f2")
            .await
            .unwrap();

        let motor = SugestaoMotorService::new(pool.clone(), mock_ia_com_titulo("X"));
        let (stats, criadas) = motor.executar_pipeline().await.unwrap();

        assert_eq!(stats.geradas, 0);
        assert_eq!(stats.descartadas, 1);
        assert!(criadas.is_empty());
    }

    #[tokio::test]
    async fn motor_usa_fallback_quando_llm_falha() {
        let pool = setup_pool().await;
        let clusters_repo = ClustersRepository::new(&pool);

        inserir_arquivo(&pool, "f1", "/a/x.pdf", "x.pdf").await;
        inserir_arquivo(&pool, "f2", "/b/y.pdf", "y.pdf").await;
        inserir_arquivo(&pool, "f3", "/c/z.pdf", "z.pdf").await;

        let cid = clusters_repo
            .upsert_cluster("Sem LLM", None, 0.8)
            .await
            .unwrap();
        clusters_repo
            .add_cluster_member_file(&cid, "f1")
            .await
            .unwrap();
        clusters_repo
            .add_cluster_member_file(&cid, "f2")
            .await
            .unwrap();
        clusters_repo
            .add_cluster_member_file(&cid, "f3")
            .await
            .unwrap();

        let motor = SugestaoMotorService::new(pool.clone(), mock_ia_com_erro());
        let (stats, criadas) = motor.executar_pipeline().await.unwrap();

        // Sugestão é gerada mesmo com LLM offline, com título fallback
        assert_eq!(stats.geradas, 1);
        assert_eq!(criadas.len(), 1);
        assert!(criadas[0].1.contains("Grupo semântico"));
        assert!(criadas[0].1.contains("3"));
    }

    #[tokio::test]
    async fn motor_persiste_operacao_com_file_ids() {
        let pool = setup_pool().await;
        let clusters_repo = ClustersRepository::new(&pool);

        inserir_arquivo(&pool, "f1", "/a/x.pdf", "x.pdf").await;
        inserir_arquivo(&pool, "f2", "/b/y.pdf", "y.pdf").await;

        let cid = clusters_repo
            .upsert_cluster("Op Test", None, 0.8)
            .await
            .unwrap();
        clusters_repo
            .add_cluster_member_file(&cid, "f1")
            .await
            .unwrap();
        clusters_repo
            .add_cluster_member_file(&cid, "f2")
            .await
            .unwrap();

        let motor = SugestaoMotorService::new(pool.clone(), mock_ia_com_titulo("T"));
        let (_, criadas) = motor.executar_pipeline().await.unwrap();

        let sid = &criadas[0].0;
        let sugestoes_repo = SugestoesRepository::new(&pool);
        let ops = sugestoes_repo
            .find_operations_by_suggestion(sid)
            .await
            .unwrap();

        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].tipo_operacao, "mover_arquivo");
        let payload: serde_json::Value = serde_json::from_str(&ops[0].payload).unwrap();
        let fids = payload["file_ids"].as_array().unwrap();
        assert_eq!(fids.len(), 2);
    }

    #[tokio::test]
    async fn motor_sem_clusters_retorna_zero() {
        let pool = setup_pool().await;
        let motor = SugestaoMotorService::new(pool, mock_ia_com_titulo("X"));
        let (stats, criadas) = motor.executar_pipeline().await.unwrap();
        assert_eq!(stats.geradas, 0);
        assert_eq!(stats.descartadas, 0);
        assert!(criadas.is_empty());
    }
}
