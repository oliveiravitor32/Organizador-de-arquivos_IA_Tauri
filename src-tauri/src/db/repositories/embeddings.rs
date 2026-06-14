//! Repositório de embeddings (BLOB f32 LE, ADR-020).

use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::domain::conhecimento::Embedding;
use crate::error::AppResult;

pub struct EmbeddingsRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> EmbeddingsRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Insere ou substitui embedding por (file_id, model).
    pub async fn upsert_embedding(
        &self,
        file_id: &str,
        model: &str,
        vector_blob: Vec<u8>,
    ) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO embeddings (id, file_id, model, vector, created_at)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(file_id, model) DO UPDATE SET
                vector = excluded.vector,
                created_at = excluded.created_at
            "#,
        )
        .bind(&id)
        .bind(file_id)
        .bind(model)
        .bind(&vector_blob)
        .bind(&now)
        .execute(self.pool)
        .await?;

        let real_id: String =
            sqlx::query_scalar("SELECT id FROM embeddings WHERE file_id = ? AND model = ?")
                .bind(file_id)
                .bind(model)
                .fetch_one(self.pool)
                .await?;

        Ok(real_id)
    }

    /// Retorna todos os embeddings de um modelo (para clusterização em memória, ADR-019).
    pub async fn find_all_embeddings(&self, model: &str) -> AppResult<Vec<Embedding>> {
        let rows = sqlx::query_as::<_, Embedding>(
            "SELECT id, file_id, model, vector, created_at FROM embeddings WHERE model = ?",
        )
        .bind(model)
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    /// Retorna embedding de um arquivo específico para um modelo.
    pub async fn find_embedding_by_file(
        &self,
        file_id: &str,
        model: &str,
    ) -> AppResult<Option<Embedding>> {
        let row = sqlx::query_as::<_, Embedding>(
            "SELECT id, file_id, model, vector, created_at FROM embeddings WHERE file_id = ? AND model = ?",
        )
        .bind(file_id)
        .bind(model)
        .fetch_optional(self.pool)
        .await?;

        Ok(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        crate::db::MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    async fn inserir_arquivo(pool: &SqlitePool, id: &str) {
        sqlx::query(
            "INSERT INTO files (id, path, relative_path, name, status) VALUES (?, ?, ?, ?, 'discovered')"
        )
        .bind(id)
        .bind(format!("/tmp/{id}.txt"))
        .bind(format!("{id}.txt"))
        .bind(format!("{id}.txt"))
        .execute(pool)
        .await
        .unwrap();
    }

    fn blob(vals: &[f32]) -> Vec<u8> {
        bytemuck::cast_slice(vals).to_vec()
    }

    #[tokio::test]
    async fn upsert_embedding_persiste_vector() {
        let pool = pool().await;
        inserir_arquivo(&pool, "e1").await;
        let repo = EmbeddingsRepository::new(&pool);
        let b = blob(&[0.1, 0.2, 0.3]);
        repo.upsert_embedding("e1", "nomic-embed-text", b.clone()).await.unwrap();
        let emb = repo.find_embedding_by_file("e1", "nomic-embed-text").await.unwrap().unwrap();
        assert_eq!(emb.vector, b);
    }

    #[tokio::test]
    async fn upsert_embedding_substitui_vector() {
        let pool = pool().await;
        inserir_arquivo(&pool, "e2").await;
        let repo = EmbeddingsRepository::new(&pool);
        repo.upsert_embedding("e2", "nomic-embed-text", blob(&[0.1])).await.unwrap();
        let b2 = blob(&[0.9, 0.8]);
        repo.upsert_embedding("e2", "nomic-embed-text", b2.clone()).await.unwrap();
        let emb = repo.find_embedding_by_file("e2", "nomic-embed-text").await.unwrap().unwrap();
        assert_eq!(emb.vector, b2);
    }

    #[tokio::test]
    async fn find_all_embeddings_retorna_apenas_model_solicitado() {
        let pool = pool().await;
        inserir_arquivo(&pool, "e3").await;
        inserir_arquivo(&pool, "e4").await;
        let repo = EmbeddingsRepository::new(&pool);
        repo.upsert_embedding("e3", "nomic-embed-text", blob(&[0.1])).await.unwrap();
        repo.upsert_embedding("e4", "outro-modelo", blob(&[0.2])).await.unwrap();
        let embs = repo.find_all_embeddings("nomic-embed-text").await.unwrap();
        assert_eq!(embs.len(), 1);
        assert_eq!(embs[0].file_id, "e3");
    }

    #[tokio::test]
    async fn find_embedding_by_file_retorna_none_se_ausente() {
        let pool = pool().await;
        let repo = EmbeddingsRepository::new(&pool);
        let result = repo.find_embedding_by_file("inexistente", "nomic-embed-text").await.unwrap();
        assert!(result.is_none());
    }
}
