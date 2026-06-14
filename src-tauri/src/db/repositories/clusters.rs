//! Repositório de clusters semânticos.

use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::domain::conhecimento::Cluster;
use crate::error::AppResult;

pub struct ClustersRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> ClustersRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Cria ou atualiza cluster por nome.
    pub async fn upsert_cluster(
        &self,
        name: &str,
        description: Option<&str>,
        confidence: f64,
    ) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO clusters (id, name, description, confidence, created_at)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(name) DO UPDATE SET
                confidence = excluded.confidence,
                description = excluded.description
            "#,
        )
        .bind(&id)
        .bind(name)
        .bind(description)
        .bind(confidence)
        .bind(&now)
        .execute(self.pool)
        .await?;

        let real_id: String =
            sqlx::query_scalar("SELECT id FROM clusters WHERE name = ?")
                .bind(name)
                .fetch_one(self.pool)
                .await?;

        Ok(real_id)
    }

    /// Vincula arquivo a cluster.
    pub async fn add_cluster_member_file(&self, cluster_id: &str, file_id: &str) -> AppResult<()> {
        let id = Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT OR IGNORE INTO cluster_members (id, cluster_id, file_id) VALUES (?, ?, ?)",
        )
        .bind(&id)
        .bind(cluster_id)
        .bind(file_id)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    /// Retorna todos os clusters.
    pub async fn find_clusters(&self) -> AppResult<Vec<Cluster>> {
        let rows = sqlx::query_as::<_, Cluster>(
            "SELECT id, name, description, confidence, created_at FROM clusters ORDER BY confidence DESC",
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    /// Retorna ids dos arquivos membros de um cluster.
    pub async fn find_members_by_cluster(&self, cluster_id: &str) -> AppResult<Vec<String>> {
        let ids = sqlx::query_scalar::<_, String>(
            "SELECT file_id FROM cluster_members WHERE file_id IS NOT NULL AND cluster_id = ?",
        )
        .bind(cluster_id)
        .fetch_all(self.pool)
        .await?;

        Ok(ids)
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

    #[tokio::test]
    async fn upsert_cluster_cria_e_retorna_id() {
        let pool = pool().await;
        let repo = ClustersRepository::new(&pool);
        let id = repo.upsert_cluster("Finanças", Some("Docs financeiros"), 0.85).await.unwrap();
        assert!(!id.is_empty());
    }

    #[tokio::test]
    async fn upsert_cluster_e_idempotente_por_nome() {
        let pool = pool().await;
        let repo = ClustersRepository::new(&pool);
        let id1 = repo.upsert_cluster("Tech", None, 0.7).await.unwrap();
        let id2 = repo.upsert_cluster("Tech", Some("updated"), 0.9).await.unwrap();
        assert_eq!(id1, id2);
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM clusters WHERE name = 'Tech'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn add_cluster_member_file_persiste() {
        let pool = pool().await;
        inserir_arquivo(&pool, "cm1").await;
        let repo = ClustersRepository::new(&pool);
        let cid = repo.upsert_cluster("RH", None, 0.75).await.unwrap();
        repo.add_cluster_member_file(&cid, "cm1").await.unwrap();
        let members = repo.find_members_by_cluster(&cid).await.unwrap();
        assert_eq!(members, vec!["cm1".to_string()]);
    }

    #[tokio::test]
    async fn add_cluster_member_e_idempotente() {
        let pool = pool().await;
        inserir_arquivo(&pool, "cm2").await;
        let repo = ClustersRepository::new(&pool);
        let cid = repo.upsert_cluster("Legal", None, 0.8).await.unwrap();
        repo.add_cluster_member_file(&cid, "cm2").await.unwrap();
        repo.add_cluster_member_file(&cid, "cm2").await.unwrap();
        let members = repo.find_members_by_cluster(&cid).await.unwrap();
        assert_eq!(members.len(), 1);
    }

    #[tokio::test]
    async fn find_clusters_retorna_todos() {
        let pool = pool().await;
        let repo = ClustersRepository::new(&pool);
        repo.upsert_cluster("A", None, 0.9).await.unwrap();
        repo.upsert_cluster("B", None, 0.7).await.unwrap();
        let clusters = repo.find_clusters().await.unwrap();
        assert_eq!(clusters.len(), 2);
        // Ordenado por confiança decrescente
        assert!(clusters[0].confidence >= clusters[1].confidence);
    }
}
