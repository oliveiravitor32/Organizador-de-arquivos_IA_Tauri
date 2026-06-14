//! Serviço de clusterização semântica por similaridade cosseno (ADR-019, ADR-022).

use ndarray::Array1;
use sqlx::SqlitePool;

use crate::db::repositories::clusters::ClustersRepository;
use crate::db::repositories::embeddings::EmbeddingsRepository;
use crate::error::AppResult;
use crate::services::ia::ollama::embed::deserializar_vetor;

/// Threshold de similaridade cosseno para agrupar arquivos no mesmo cluster (ADR-022).
const THRESHOLD: f32 = 0.75;

pub struct ClusterService<'a> {
    pool: &'a SqlitePool,
    embed_model: String,
}

impl<'a> ClusterService<'a> {
    pub fn new(pool: &'a SqlitePool, embed_model: impl Into<String>) -> Self {
        Self { pool, embed_model: embed_model.into() }
    }

    /// Reconstrói clusters a partir de todos os embeddings existentes.
    /// Retorna a quantidade de clusters criados/atualizados.
    pub async fn recalcular(&self) -> AppResult<usize> {
        let emb_repo = EmbeddingsRepository::new(self.pool);
        let cluster_repo = ClustersRepository::new(self.pool);

        let embeddings = emb_repo.find_all_embeddings(&self.embed_model).await?;
        if embeddings.is_empty() {
            return Ok(0);
        }

        // Decodifica vetores
        let vecs: Vec<(String, Array1<f32>)> = embeddings
            .iter()
            .map(|e| {
                let v = deserializar_vetor(&e.vector);
                (e.file_id.clone(), Array1::from_vec(v))
            })
            .collect();

        // Union-Find simples em memória
        let n = vecs.len();
        let mut parent: Vec<usize> = (0..n).collect();

        fn find(parent: &mut Vec<usize>, i: usize) -> usize {
            if parent[i] != i {
                parent[i] = find(parent, parent[i]);
            }
            parent[i]
        }

        for i in 0..n {
            for j in (i + 1)..n {
                let sim = cosine_sim(&vecs[i].1, &vecs[j].1);
                if sim >= THRESHOLD {
                    let ri = find(&mut parent, i);
                    let rj = find(&mut parent, j);
                    if ri != rj {
                        parent[rj] = ri;
                    }
                }
            }
        }

        // Agrupa por raiz
        let mut grupos: std::collections::HashMap<usize, Vec<usize>> =
            std::collections::HashMap::new();
        for i in 0..n {
            let root = find(&mut parent, i);
            grupos.entry(root).or_default().push(i);
        }

        let mut total = 0;
        for (root, members) in &grupos {
            if members.len() < 2 {
                continue; // cluster de um único arquivo não tem valor semântico
            }

            // Confiança = média das similaridades dos pares do grupo
            let confidence = media_similaridades(members, &vecs);
            let name = format!("cluster-{root}");

            let cluster_id = cluster_repo
                .upsert_cluster(&name, None, confidence as f64)
                .await?;

            for &idx in members {
                cluster_repo
                    .add_cluster_member_file(&cluster_id, &vecs[idx].0)
                    .await?;
            }
            total += 1;
        }

        Ok(total)
    }
}

fn cosine_sim(a: &Array1<f32>, b: &Array1<f32>) -> f32 {
    let dot = a.dot(b);
    let norm_a = a.dot(a).sqrt();
    let norm_b = b.dot(b).sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a * norm_b)
}

fn media_similaridades(members: &[usize], vecs: &[(String, Array1<f32>)]) -> f32 {
    let n = members.len();
    if n < 2 {
        return 0.0;
    }
    let mut soma = 0.0f32;
    let mut count = 0u32;
    for i in 0..n {
        for j in (i + 1)..n {
            soma += cosine_sim(&vecs[members[i]].1, &vecs[members[j]].1);
            count += 1;
        }
    }
    if count == 0 { 0.0 } else { soma / count as f32 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cosine_sim_vetores_identicos_retorna_1() {
        let a = Array1::from_vec(vec![1.0f32, 0.0, 0.0]);
        assert!((cosine_sim(&a, &a) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn cosine_sim_vetores_ortogonais_retorna_0() {
        let a = Array1::from_vec(vec![1.0f32, 0.0]);
        let b = Array1::from_vec(vec![0.0f32, 1.0]);
        assert!(cosine_sim(&a, &b).abs() < 1e-6);
    }

    #[test]
    fn cosine_sim_vetor_zero_retorna_0() {
        let a = Array1::from_vec(vec![0.0f32, 0.0]);
        let b = Array1::from_vec(vec![1.0f32, 0.0]);
        assert_eq!(cosine_sim(&a, &b), 0.0);
    }

    #[tokio::test]
    async fn recalcular_sem_embeddings_retorna_0() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        crate::db::MIGRATOR.run(&pool).await.unwrap();
        let svc = ClusterService::new(&pool, "nomic-embed-text");
        let count = svc.recalcular().await.unwrap();
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn recalcular_agrupa_arquivos_similares() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        crate::db::MIGRATOR.run(&pool).await.unwrap();

        // Insere 2 arquivos com vetores quase idênticos e 1 arquivo diferente
        for id in ["fa", "fb", "fc"] {
            sqlx::query(
                "INSERT INTO files (id, path, relative_path, name, status) VALUES (?, ?, ?, ?, 'discovered')"
            ).bind(id).bind(format!("/tmp/{id}.txt")).bind(format!("{id}.txt")).bind(format!("{id}.txt"))
            .execute(&pool).await.unwrap();
        }

        let similar = vec![1.0f32, 0.0, 0.0];
        let diferente = vec![0.0f32, 1.0, 0.0];
        let blob_sim: Vec<u8> = bytemuck::cast_slice(&similar).to_vec();
        let blob_dif: Vec<u8> = bytemuck::cast_slice(&diferente).to_vec();

        for (fid, blob) in [("fa", blob_sim.clone()), ("fb", blob_sim.clone()), ("fc", blob_dif)] {
            let id = uuid::Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO embeddings (id, file_id, model, vector, created_at) VALUES (?, ?, 'nomic-embed-text', ?, '2024-01-01')"
            ).bind(&id).bind(fid).bind(blob).execute(&pool).await.unwrap();
        }

        let svc = ClusterService::new(&pool, "nomic-embed-text");
        let count = svc.recalcular().await.unwrap();
        assert_eq!(count, 1); // apenas fa+fb formam cluster
    }
}
