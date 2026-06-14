//! Repositório de entidades e vínculos arquivo↔entidade.

use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::domain::conhecimento::{EntidadeExtraida, Entity};
use crate::error::AppResult;

pub struct EntitiesRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> EntitiesRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Insere ou retorna entidade existente por (name, type). Retorna o id real.
    pub async fn upsert_entity(&self, e: &EntidadeExtraida) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let tipo = e.entity_type.as_str();

        sqlx::query(
            r#"
            INSERT INTO entities (id, name, type, confidence, created_at)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(name, type) DO UPDATE SET
                confidence = MAX(confidence, excluded.confidence)
            "#,
        )
        .bind(&id)
        .bind(&e.name)
        .bind(tipo)
        .bind(e.confidence)
        .bind(&now)
        .execute(self.pool)
        .await?;

        let real_id: String =
            sqlx::query_scalar("SELECT id FROM entities WHERE name = ? AND type = ?")
                .bind(&e.name)
                .bind(tipo)
                .fetch_one(self.pool)
                .await?;

        Ok(real_id)
    }

    /// Cria vínculo arquivo↔entidade (MENCIONA). Idempotente por (file_id, entity_id).
    pub async fn link_file_entity(
        &self,
        file_id: &str,
        entity_id: &str,
        confidence: f64,
    ) -> AppResult<()> {
        let id = Uuid::new_v4().to_string();

        sqlx::query(
            r#"
            INSERT INTO file_entities (id, file_id, entity_id, relationship_type, confidence)
            VALUES (?, ?, ?, 'mentions', ?)
            ON CONFLICT(file_id, entity_id) DO UPDATE SET
                confidence = MAX(confidence, excluded.confidence)
            "#,
        )
        .bind(&id)
        .bind(file_id)
        .bind(entity_id)
        .bind(confidence)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    /// Retorna todas as entidades mencionadas em um arquivo.
    pub async fn find_entities_by_file(&self, file_id: &str) -> AppResult<Vec<Entity>> {
        let rows = sqlx::query_as::<_, Entity>(
            r#"
            SELECT e.id, e.name, e.type, e.confidence, e.created_at
            FROM entities e
            JOIN file_entities fe ON fe.entity_id = e.id
            WHERE fe.file_id = ?
            "#,
        )
        .bind(file_id)
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    /// Conta entidades existentes no banco.
    pub async fn count(&self) -> AppResult<i64> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM entities")
            .fetch_one(self.pool)
            .await?;
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::conhecimento::EntityType;

    async fn pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        crate::db::MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    fn entidade(nome: &str, tipo: EntityType, conf: f64) -> EntidadeExtraida {
        EntidadeExtraida {
            name: nome.to_string(),
            entity_type: tipo,
            confidence: conf,
        }
    }

    #[tokio::test]
    async fn upsert_entity_insere_e_retorna_id() {
        let pool = pool().await;
        let repo = EntitiesRepository::new(&pool);
        let id = repo
            .upsert_entity(&entidade("João", EntityType::Person, 0.9))
            .await
            .unwrap();
        assert!(!id.is_empty());
    }

    #[tokio::test]
    async fn upsert_entity_e_idempotente_por_nome_tipo() {
        let pool = pool().await;
        let repo = EntitiesRepository::new(&pool);
        let e = entidade("XPTO", EntityType::Organization, 0.8);
        let id1 = repo.upsert_entity(&e).await.unwrap();
        let id2 = repo.upsert_entity(&e).await.unwrap();
        assert_eq!(id1, id2);
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM entities WHERE name = 'XPTO'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn link_file_entity_persiste_menciona() {
        let pool = pool().await;
        let erepo = EntitiesRepository::new(&pool);
        // Inserir arquivo mínimo
        sqlx::query(
            "INSERT INTO files (id, path, relative_path, name, status) VALUES ('f1', '/tmp/f.txt', 'f.txt', 'f.txt', 'discovered')"
        ).execute(&pool).await.unwrap();
        let eid = erepo
            .upsert_entity(&entidade("Alpha", EntityType::Project, 0.85))
            .await
            .unwrap();
        erepo.link_file_entity("f1", &eid, 0.85).await.unwrap();
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM file_entities WHERE file_id = 'f1'")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn link_file_entity_e_idempotente() {
        let pool = pool().await;
        let erepo = EntitiesRepository::new(&pool);
        sqlx::query(
            "INSERT INTO files (id, path, relative_path, name, status) VALUES ('f2', '/tmp/g.txt', 'g.txt', 'g.txt', 'discovered')"
        ).execute(&pool).await.unwrap();
        let eid = erepo
            .upsert_entity(&entidade("Beta", EntityType::Project, 0.7))
            .await
            .unwrap();
        erepo.link_file_entity("f2", &eid, 0.7).await.unwrap();
        erepo.link_file_entity("f2", &eid, 0.9).await.unwrap();
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM file_entities WHERE file_id = 'f2'")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn find_entities_by_file_retorna_entidades_do_arquivo() {
        let pool = pool().await;
        let erepo = EntitiesRepository::new(&pool);
        sqlx::query(
            "INSERT INTO files (id, path, relative_path, name, status) VALUES ('f3', '/tmp/h.txt', 'h.txt', 'h.txt', 'discovered')"
        ).execute(&pool).await.unwrap();
        let eid = erepo
            .upsert_entity(&entidade("Gamma", EntityType::Topic, 0.75))
            .await
            .unwrap();
        erepo.link_file_entity("f3", &eid, 0.75).await.unwrap();
        let entities = erepo.find_entities_by_file("f3").await.unwrap();
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].name, "Gamma");
    }
}
