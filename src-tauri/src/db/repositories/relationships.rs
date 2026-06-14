//! Repositório de relações entre entidades.

use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::domain::conhecimento::RelacaoInferida;
use crate::error::AppResult;

pub struct RelationshipsRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> RelationshipsRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Insere ou atualiza relação por (source_id, target_id, type).
    pub async fn upsert_relationship(
        &self,
        source_entity_id: &str,
        target_entity_id: &str,
        relacao: &RelacaoInferida,
    ) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO relationships (id, source_entity_id, target_entity_id, relationship_type, confidence, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(source_entity_id, target_entity_id, relationship_type) DO UPDATE SET
                confidence = MAX(confidence, excluded.confidence)
            "#,
        )
        .bind(&id)
        .bind(source_entity_id)
        .bind(target_entity_id)
        .bind(&relacao.relation_type)
        .bind(relacao.confidence)
        .bind(&now)
        .execute(self.pool)
        .await?;

        let real_id: String = sqlx::query_scalar(
            "SELECT id FROM relationships WHERE source_entity_id = ? AND target_entity_id = ? AND relationship_type = ?",
        )
        .bind(source_entity_id)
        .bind(target_entity_id)
        .bind(&relacao.relation_type)
        .fetch_one(self.pool)
        .await?;

        Ok(real_id)
    }

    /// Retorna relações onde a entidade é source ou target.
    pub async fn find_relationships_by_entity(
        &self,
        entity_id: &str,
    ) -> AppResult<Vec<(String, String, String, f64)>> {
        let rows = sqlx::query_as::<_, (String, String, String, f64)>(
            r#"
            SELECT source_entity_id, target_entity_id, relationship_type, confidence
            FROM relationships
            WHERE source_entity_id = ? OR target_entity_id = ?
            "#,
        )
        .bind(entity_id)
        .bind(entity_id)
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
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

    async fn inserir_entidade(pool: &SqlitePool, id: &str, nome: &str) {
        sqlx::query(
            "INSERT INTO entities (id, name, type, confidence, created_at) VALUES (?, ?, 'topic', 0.8, '2024-01-01')"
        )
        .bind(id)
        .bind(nome)
        .execute(pool)
        .await
        .unwrap();
    }

    fn relacao(tipo: &str, conf: f64) -> RelacaoInferida {
        RelacaoInferida {
            source: "A".into(),
            target: "B".into(),
            relation_type: tipo.into(),
            confidence: conf,
        }
    }

    #[tokio::test]
    async fn upsert_relationship_persiste() {
        let pool = pool().await;
        inserir_entidade(&pool, "eid1", "Alpha").await;
        inserir_entidade(&pool, "eid2", "Beta").await;
        let repo = RelationshipsRepository::new(&pool);
        let id = repo.upsert_relationship("eid1", "eid2", &relacao("related_to", 0.8)).await.unwrap();
        assert!(!id.is_empty());
    }

    #[tokio::test]
    async fn upsert_relationship_e_idempotente() {
        let pool = pool().await;
        inserir_entidade(&pool, "eid3", "Gamma").await;
        inserir_entidade(&pool, "eid4", "Delta").await;
        let repo = RelationshipsRepository::new(&pool);
        let r = relacao("parent_of", 0.7);
        let id1 = repo.upsert_relationship("eid3", "eid4", &r).await.unwrap();
        let id2 = repo.upsert_relationship("eid3", "eid4", &r).await.unwrap();
        assert_eq!(id1, id2);
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM relationships")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn find_relationships_by_entity_retorna_relacoes() {
        let pool = pool().await;
        inserir_entidade(&pool, "eid5", "Epsilon").await;
        inserir_entidade(&pool, "eid6", "Zeta").await;
        let repo = RelationshipsRepository::new(&pool);
        repo.upsert_relationship("eid5", "eid6", &relacao("derived_from", 0.9)).await.unwrap();
        let rels = repo.find_relationships_by_entity("eid5").await.unwrap();
        assert_eq!(rels.len(), 1);
        assert_eq!(rels[0].2, "derived_from");
    }
}
