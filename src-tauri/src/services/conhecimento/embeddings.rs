//! Serviço de geração e persistência de embeddings.

use sqlx::SqlitePool;

use crate::db::repositories::embeddings::EmbeddingsRepository;
use crate::error::AppResult;
use crate::services::ia::{ollama::embed::serializar_vetor, ServicoIa};

pub struct EmbeddingService<'a> {
    pool: &'a SqlitePool,
    ia: &'a dyn ServicoIa,
    model: String,
}

impl<'a> EmbeddingService<'a> {
    pub fn new(pool: &'a SqlitePool, ia: &'a dyn ServicoIa, model: impl Into<String>) -> Self {
        Self {
            pool,
            ia,
            model: model.into(),
        }
    }

    /// Gera embedding para o texto e persiste no banco. Retorna o blob serializado.
    pub async fn processar(&self, file_id: &str, texto: &str) -> AppResult<Vec<u8>> {
        let vetor = self.ia.gerar_embedding(texto).await?;
        let blob = serializar_vetor(&vetor);

        let repo = EmbeddingsRepository::new(self.pool);
        repo.upsert_embedding(file_id, &self.model, blob.clone())
            .await?;

        Ok(blob)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::ia::ollama::embed::deserializar_vetor;
    use crate::services::ia::MockServicoIa;

    async fn pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        crate::db::MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn processar_persiste_embedding() {
        let pool = pool().await;
        sqlx::query(
            "INSERT INTO files (id, path, relative_path, name, status) VALUES ('f1', '/tmp/f.txt', 'f.txt', 'f.txt', 'discovered')"
        ).execute(&pool).await.unwrap();

        let mut mock = MockServicoIa::new();
        mock.expect_gerar_embedding()
            .returning(|_| Box::pin(async { Ok(vec![0.1f32, 0.2, 0.3]) }));

        let svc = EmbeddingService::new(&pool, &mock, "nomic-embed-text");
        let blob = svc.processar("f1", "texto").await.unwrap();

        let vetor = deserializar_vetor(&blob);
        assert_eq!(vetor.len(), 3);
        assert!((vetor[0] - 0.1f32).abs() < f32::EPSILON);

        // Confirma persistência
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM embeddings WHERE file_id = 'f1'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn processar_idempotente_para_mesmo_arquivo() {
        let pool = pool().await;
        sqlx::query(
            "INSERT INTO files (id, path, relative_path, name, status) VALUES ('f2', '/tmp/g.txt', 'g.txt', 'g.txt', 'discovered')"
        ).execute(&pool).await.unwrap();

        let mut mock = MockServicoIa::new();
        mock.expect_gerar_embedding()
            .times(2)
            .returning(|_| Box::pin(async { Ok(vec![0.5f32]) }));

        let svc = EmbeddingService::new(&pool, &mock, "nomic-embed-text");
        svc.processar("f2", "texto").await.unwrap();
        svc.processar("f2", "texto").await.unwrap();

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM embeddings WHERE file_id = 'f2'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 1);
    }
}
