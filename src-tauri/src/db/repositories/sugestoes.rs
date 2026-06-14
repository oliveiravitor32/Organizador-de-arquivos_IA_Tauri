//! Repositório de sugestões de organização (UC-005, M3).

use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::domain::sugestoes::{Sugestao, SugestaoOperacao};
use crate::error::AppResult;

pub struct SugestoesRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SugestoesRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Insere uma sugestão nova. Idempotente por id.
    pub async fn insert_suggestion(
        &self,
        tipo: &str,
        titulo: Option<&str>,
        descricao: Option<&str>,
        confianca: f64,
        cluster_id: Option<&str>,
        evidencias_json: Option<&str>,
    ) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT OR IGNORE INTO suggestions
                (id, tipo, titulo, descricao, confianca, status, cluster_id, evidencias, criado_em)
            VALUES (?, ?, ?, ?, ?, 'pendente', ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(tipo)
        .bind(titulo)
        .bind(descricao)
        .bind(confianca)
        .bind(cluster_id)
        .bind(evidencias_json)
        .bind(&now)
        .execute(self.pool)
        .await?;

        Ok(id)
    }

    /// Associa uma operação concreta a uma sugestão (usado pelo Marco 4).
    pub async fn insert_suggestion_operation(
        &self,
        suggestion_id: &str,
        tipo_operacao: &str,
        payload_json: &str,
    ) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();

        sqlx::query(
            r#"
            INSERT INTO suggestion_operations (id, suggestion_id, tipo_operacao, payload)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(suggestion_id)
        .bind(tipo_operacao)
        .bind(payload_json)
        .execute(self.pool)
        .await?;

        Ok(id)
    }

    /// Retorna todas as sugestões, opcionalmente filtradas por status.
    pub async fn find_suggestions(
        &self,
        status: Option<&str>,
    ) -> AppResult<Vec<Sugestao>> {
        let rows = match status {
            Some(s) => {
                sqlx::query_as::<_, Sugestao>(
                    "SELECT * FROM suggestions WHERE status = ? ORDER BY confianca DESC",
                )
                .bind(s)
                .fetch_all(self.pool)
                .await?
            }
            None => {
                sqlx::query_as::<_, Sugestao>(
                    "SELECT * FROM suggestions ORDER BY confianca DESC",
                )
                .fetch_all(self.pool)
                .await?
            }
        };

        Ok(rows)
    }

    /// Retorna uma sugestão por id.
    pub async fn find_suggestion_by_id(&self, id: &str) -> AppResult<Option<Sugestao>> {
        let row = sqlx::query_as::<_, Sugestao>(
            "SELECT * FROM suggestions WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;

        Ok(row)
    }

    /// Retorna as operações de uma sugestão.
    pub async fn find_operations_by_suggestion(
        &self,
        suggestion_id: &str,
    ) -> AppResult<Vec<SugestaoOperacao>> {
        let rows = sqlx::query_as::<_, SugestaoOperacao>(
            "SELECT * FROM suggestion_operations WHERE suggestion_id = ?",
        )
        .bind(suggestion_id)
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    /// Total de sugestões no banco.
    pub async fn count(&self) -> AppResult<i64> {
        let n: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM suggestions")
            .fetch_one(self.pool)
            .await?;
        Ok(n)
    }
}

// ─── Testes ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn insert_e_find_suggestion() {
        let pool = setup().await;
        let repo = SugestoesRepository::new(&pool);

        let id = repo
            .insert_suggestion("agrupamento", Some("Grupo A"), Some("Desc"), 0.85, None, None)
            .await
            .unwrap();

        let s = repo.find_suggestion_by_id(&id).await.unwrap().unwrap();
        assert_eq!(s.tipo, "agrupamento");
        assert_eq!(s.titulo.as_deref(), Some("Grupo A"));
        assert_eq!(s.status, "pendente");
    }

    #[tokio::test]
    async fn find_suggestions_por_status() {
        let pool = setup().await;
        let repo = SugestoesRepository::new(&pool);

        repo.insert_suggestion("agrupamento", Some("A"), None, 0.9, None, None)
            .await
            .unwrap();

        let pendentes = repo.find_suggestions(Some("pendente")).await.unwrap();
        assert_eq!(pendentes.len(), 1);

        let aceitas = repo.find_suggestions(Some("aceita")).await.unwrap();
        assert!(aceitas.is_empty());
    }

    #[tokio::test]
    async fn insert_e_find_operation() {
        let pool = setup().await;
        let repo = SugestoesRepository::new(&pool);

        let sid = repo
            .insert_suggestion("agrupamento", Some("B"), None, 0.8, None, None)
            .await
            .unwrap();

        let payload = r#"{"file_ids":["f1","f2"],"destino":"/docs/grupo"}"#;
        repo.insert_suggestion_operation(&sid, "mover_arquivo", payload)
            .await
            .unwrap();

        let ops = repo.find_operations_by_suggestion(&sid).await.unwrap();
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].tipo_operacao, "mover_arquivo");
    }

    #[tokio::test]
    async fn find_suggestions_sem_filtro_retorna_todas() {
        let pool = setup().await;
        let repo = SugestoesRepository::new(&pool);

        repo.insert_suggestion("agrupamento", Some("X"), None, 0.9, None, None)
            .await
            .unwrap();
        repo.insert_suggestion("agrupamento", Some("Y"), None, 0.7, None, None)
            .await
            .unwrap();

        let todas = repo.find_suggestions(None).await.unwrap();
        assert_eq!(todas.len(), 2);
        // Ordenadas por confiança DESC
        assert!(todas[0].confianca >= todas[1].confianca);
    }

    #[tokio::test]
    async fn count_retorna_total() {
        let pool = setup().await;
        let repo = SugestoesRepository::new(&pool);

        assert_eq!(repo.count().await.unwrap(), 0);
        repo.insert_suggestion("agrupamento", None, None, 0.6, None, None)
            .await
            .unwrap();
        assert_eq!(repo.count().await.unwrap(), 1);
    }
}
