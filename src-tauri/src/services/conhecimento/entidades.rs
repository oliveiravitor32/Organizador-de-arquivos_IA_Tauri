//! Serviço de extração de entidades de um arquivo.

use sqlx::SqlitePool;

use crate::db::repositories::entities::EntitiesRepository;
use crate::error::AppResult;
use crate::services::ia::ServicoIa;

pub struct ExtracaoEntidadesService<'a> {
    pool: &'a SqlitePool,
    ia: &'a dyn ServicoIa,
}

impl<'a> ExtracaoEntidadesService<'a> {
    pub fn new(pool: &'a SqlitePool, ia: &'a dyn ServicoIa) -> Self {
        Self { pool, ia }
    }

    /// Extrai entidades do texto de um arquivo e persiste os vínculos.
    /// Retorna a quantidade de entidades novas/atualizadas.
    pub async fn processar(&self, file_id: &str, texto: &str) -> AppResult<usize> {
        let entidades = self.ia.extrair_entidades(texto).await?;
        let total = entidades.len();
        let repo = EntitiesRepository::new(self.pool);

        for e in &entidades {
            let entity_id = repo.upsert_entity(e).await?;
            repo.link_file_entity(file_id, &entity_id, e.confidence)
                .await?;
        }

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::conhecimento::{EntidadeExtraida, EntityType, RelacaoInferida};
    use crate::services::ia::MockServicoIa;

    async fn pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        crate::db::MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn processar_persiste_entidades_e_vinculos() {
        let pool = pool().await;
        sqlx::query(
            "INSERT INTO files (id, path, relative_path, name, status) VALUES ('f1', '/tmp/f.txt', 'f.txt', 'f.txt', 'discovered')"
        ).execute(&pool).await.unwrap();

        let mut mock = MockServicoIa::new();
        mock.expect_extrair_entidades().returning(|_| {
            Box::pin(async {
                Ok(vec![EntidadeExtraida {
                    name: "Empresa X".into(),
                    entity_type: EntityType::Organization,
                    confidence: 0.9,
                }])
            })
        });

        let svc = ExtracaoEntidadesService::new(&pool, &mock);
        let count = svc.processar("f1", "texto sobre Empresa X").await.unwrap();
        assert_eq!(count, 1);

        let count_db: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM entities")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count_db, 1);

        let count_link: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM file_entities WHERE file_id = 'f1'")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(count_link, 1);
    }

    #[tokio::test]
    async fn processar_sem_entidades_retorna_zero() {
        let pool = pool().await;
        sqlx::query(
            "INSERT INTO files (id, path, relative_path, name, status) VALUES ('f2', '/tmp/g.txt', 'g.txt', 'g.txt', 'discovered')"
        ).execute(&pool).await.unwrap();

        let mut mock = MockServicoIa::new();
        mock.expect_extrair_entidades()
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        let svc = ExtracaoEntidadesService::new(&pool, &mock);
        let count = svc.processar("f2", "texto sem entidades").await.unwrap();
        assert_eq!(count, 0);
    }
}
