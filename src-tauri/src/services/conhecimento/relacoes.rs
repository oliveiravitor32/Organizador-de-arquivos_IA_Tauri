//! Serviço de inferência e persistência de relações entre entidades.

use sqlx::SqlitePool;

use crate::db::repositories::entities::EntitiesRepository;
use crate::db::repositories::relationships::RelationshipsRepository;
use crate::error::AppResult;
use crate::services::ia::ServicoIa;

pub struct RelacoesService<'a> {
    pool: &'a SqlitePool,
    ia: &'a dyn ServicoIa,
}

impl<'a> RelacoesService<'a> {
    pub fn new(pool: &'a SqlitePool, ia: &'a dyn ServicoIa) -> Self {
        Self { pool, ia }
    }

    /// Infere relações entre as entidades do arquivo e persiste.
    /// Recebe lista de nomes de entidades já extraídas + texto de contexto.
    pub async fn processar(&self, texto: &str, nomes_entidades: &[String]) -> AppResult<usize> {
        if nomes_entidades.len() < 2 {
            return Ok(0);
        }

        let relacoes = self.ia.inferir_relacoes(texto, nomes_entidades).await?;
        let total = relacoes.len();

        let erepo = EntitiesRepository::new(self.pool);
        let rrepo = RelationshipsRepository::new(self.pool);

        for r in &relacoes {
            // Busca ids das entidades por nome (já devem existir no banco)
            let src_id: Option<String> =
                sqlx::query_scalar("SELECT id FROM entities WHERE name = ?")
                    .bind(&r.source)
                    .fetch_optional(self.pool)
                    .await?;

            let tgt_id: Option<String> =
                sqlx::query_scalar("SELECT id FROM entities WHERE name = ?")
                    .bind(&r.target)
                    .fetch_optional(self.pool)
                    .await?;

            if let (Some(src), Some(tgt)) = (src_id, tgt_id) {
                rrepo.upsert_relationship(&src, &tgt, r).await?;
            }
        }

        // Suppress unused import warning
        let _ = erepo;

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

    async fn inserir_entidade(pool: &SqlitePool, id: &str, nome: &str) {
        sqlx::query(
            "INSERT INTO entities (id, name, type, confidence, created_at) VALUES (?, ?, 'person', 0.8, '2024-01-01')"
        ).bind(id).bind(nome).execute(pool).await.unwrap();
    }

    #[tokio::test]
    async fn processar_persiste_relacoes() {
        let pool = pool().await;
        inserir_entidade(&pool, "e1", "Alice").await;
        inserir_entidade(&pool, "e2", "Bob").await;

        let mut mock = MockServicoIa::new();
        mock.expect_inferir_relacoes().returning(|_, _| {
            Box::pin(async {
                Ok(vec![RelacaoInferida {
                    source: "Alice".into(),
                    target: "Bob".into(),
                    relation_type: "related_to".into(),
                    confidence: 0.8,
                }])
            })
        });

        let svc = RelacoesService::new(&pool, &mock);
        let count = svc
            .processar(
                "Alice e Bob trabalham juntos.",
                &["Alice".into(), "Bob".into()],
            )
            .await
            .unwrap();
        assert_eq!(count, 1);

        let count_db: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM relationships")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count_db, 1);
    }

    #[tokio::test]
    async fn processar_com_menos_de_2_entidades_retorna_zero() {
        let pool = pool().await;
        let mock = MockServicoIa::new();
        let svc = RelacoesService::new(&pool, &mock);
        let count = svc.processar("texto", &["Alice".into()]).await.unwrap();
        assert_eq!(count, 0);
    }
}
