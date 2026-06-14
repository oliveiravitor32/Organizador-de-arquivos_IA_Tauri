//! Orquestrador do pipeline de análise semântica (UC-003).
//!
//! Marco 2: apenas embeddings + clusterização (ADR-023).
//! O LLM será usado no Marco 3 para nomear clusters e gerar sugestões,
//! com uma chamada por cluster — não por arquivo.

use std::sync::Arc;

use sqlx::SqlitePool;
use tauri::{AppHandle, Emitter};

use crate::db::repositories::files::FileRepository;
use crate::domain::arquivo::FileStatus;
use crate::domain::conhecimento::AnaliseStats;
use crate::error::AppResult;
use crate::events;
use crate::services::ia::ServicoIa;
use crate::services::conhecimento::{
    clusters::ClusterService,
    embeddings::EmbeddingService,
};

pub struct AnaliseService {
    pool: SqlitePool,
    ia: Arc<dyn ServicoIa>,
    embed_model: String,
}

impl AnaliseService {
    pub fn new(pool: SqlitePool, ia: Arc<dyn ServicoIa>, embed_model: impl Into<String>) -> Self {
        Self { pool, ia, embed_model: embed_model.into() }
    }

    /// Executa o pipeline de embeddings + clusterização para os arquivos pendentes.
    /// Emite eventos de progresso pelo AppHandle.
    pub async fn analisar(
        &self,
        app: &AppHandle,
        file_ids: Option<Vec<String>>,
        analysis_id: &str,
    ) -> AppResult<AnaliseStats> {
        let file_repo = FileRepository::new(&self.pool);
        let arquivos = file_repo
            .find_pending_analysis(file_ids.as_deref())
            .await?;

        if arquivos.is_empty() {
            return Ok(AnaliseStats::default());
        }

        let total = arquivos.len();
        let started = std::time::Instant::now();

        let _ = app.emit(events::ANALYSIS_STARTED, serde_json::json!({
            "analysisId": analysis_id,
            "total": total
        }));

        let mut processados = 0u64;
        let mut sem_conteudo = 0u64;
        let mut falhos = 0u64;

        let embed_svc = EmbeddingService::new(&self.pool, self.ia.as_ref(), &self.embed_model);

        for (i, arquivo) in arquivos.iter().enumerate() {
            let _ = app.emit(events::ANALYSIS_PROGRESS, serde_json::json!({
                "analysisId": analysis_id,
                "processed": i,
                "total": total,
                "currentFile": arquivo.name
            }));

            // Arquivos sem texto extraível são ignorados — não é um erro.
            let texto = match file_repo.get_content(&arquivo.id).await? {
                Some(t) if !t.is_empty() => t,
                _ => {
                    sem_conteudo += 1;
                    continue;
                }
            };

            let _ = app.emit(events::ANALYSIS_EMBEDDING_GENERATION_STARTED, serde_json::json!({
                "fileId": arquivo.id
            }));

            match embed_svc.processar(&arquivo.id, &texto).await {
                Ok(_) => {
                    let _ = file_repo
                        .update_status(&arquivo.id, FileStatus::Analyzed)
                        .await;
                    processados += 1;
                }
                Err(_) => {
                    falhos += 1;
                    let _ = file_repo
                        .update_status(&arquivo.id, FileStatus::Failed)
                        .await;
                }
            }
        }

        let cluster_svc = ClusterService::new(&self.pool, &self.embed_model);
        let clusters_criados = cluster_svc.recalcular().await.unwrap_or(0) as u64;

        let _ = app.emit(events::ANALYSIS_GRAPH_UPDATED, serde_json::json!({
            "analysisId": analysis_id
        }));

        let duration_ms = started.elapsed().as_millis() as u64;
        let stats = AnaliseStats {
            processados,
            sem_conteudo,
            falhos,
            clusters_criados,
            duration_ms,
        };

        let _ = app.emit(events::ANALYSIS_COMPLETED, serde_json::json!({
            "analysisId": analysis_id,
            "stats": {
                "processados": stats.processados,
                "semConteudo": stats.sem_conteudo,
                "falhos": stats.falhos,
                "clustersCriados": stats.clusters_criados,
                "durationMs": stats.duration_ms
            }
        }));

        Ok(stats)
    }
}
