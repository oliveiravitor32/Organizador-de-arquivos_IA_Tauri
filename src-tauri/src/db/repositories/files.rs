//! Repositório de arquivos — único ponto de acesso à tabela `files` e `file_contents`.

use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::domain::arquivo::{FileContent, FileRecord, FileStatus, NewFile};
use crate::error::AppResult;

pub struct FileRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> FileRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Insere um arquivo com status DISCOVERED. Idempotente por `path`.
    pub async fn upsert_discovered(&self, file: &NewFile) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let status = FileStatus::Discovered.as_str();

        sqlx::query(
            r#"
            INSERT INTO files (id, scan_id, path, relative_path, name, extension, status, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(path) DO UPDATE SET
                scan_id    = excluded.scan_id,
                status     = excluded.status,
                created_at = excluded.created_at
            "#,
        )
        .bind(&id)
        .bind(&file.scan_id)
        .bind(&file.path)
        .bind(&file.relative_path)
        .bind(&file.name)
        .bind(&file.extension)
        .bind(status)
        .bind(&now)
        .execute(self.pool)
        .await?;

        // Retorna o id real (pode ser o existente após o ON CONFLICT).
        let real_id: String = sqlx::query_scalar("SELECT id FROM files WHERE path = ?")
            .bind(&file.path)
            .fetch_one(self.pool)
            .await?;

        Ok(real_id)
    }

    /// Retorna todos os arquivos com um determinado scan_id e status DISCOVERED.
    pub async fn find_discovered_by_scan(&self, scan_id: &str) -> AppResult<Vec<FileRecord>> {
        let records = sqlx::query_as::<_, FileRecord>(
            "SELECT * FROM files WHERE scan_id = ? AND status = 'discovered'",
        )
        .bind(scan_id)
        .fetch_all(self.pool)
        .await?;
        Ok(records)
    }

    /// Atualiza o status de um arquivo.
    pub async fn update_status(&self, file_id: &str, status: FileStatus) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();
        let indexed_at = if status == FileStatus::Indexed || status == FileStatus::PendingAnalysis {
            Some(now.clone())
        } else {
            None
        };

        sqlx::query(
            "UPDATE files SET status = ?, indexed_at = COALESCE(?, indexed_at), modified_at = ? WHERE id = ?",
        )
        .bind(status.as_str())
        .bind(&indexed_at)
        .bind(&now)
        .bind(file_id)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    /// Atualiza metadados coletados durante a indexação.
    pub async fn update_metadata(
        &self,
        file_id: &str,
        size: i64,
        hash: &str,
        mime_type: &str,
    ) -> AppResult<()> {
        sqlx::query("UPDATE files SET size = ?, hash = ?, mime_type = ? WHERE id = ?")
            .bind(size)
            .bind(hash)
            .bind(mime_type)
            .bind(file_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Insere o conteúdo extraído de um arquivo (upsert por file_id).
    pub async fn upsert_content(&self, content: &FileContent) -> AppResult<()> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO file_contents (id, file_id, content, language, content_length, extracted_at)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(file_id) DO UPDATE SET
                content        = excluded.content,
                language       = excluded.language,
                content_length = excluded.content_length,
                extracted_at   = excluded.extracted_at
            "#,
        )
        .bind(&id)
        .bind(&content.file_id)
        .bind(&content.content)
        .bind(&content.language)
        .bind(content.content_length)
        .bind(&now)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    /// Retorna arquivos com status pending_analysis, opcionalmente filtrados por ids.
    pub async fn find_pending_analysis(
        &self,
        file_ids: Option<&[String]>,
    ) -> AppResult<Vec<FileRecord>> {
        let rows = match file_ids {
            None => {
                sqlx::query_as::<_, FileRecord>(
                    "SELECT id, path, relative_path, name, extension, size, hash, mime_type, scan_id, status, created_at, modified_at, indexed_at FROM files WHERE status = 'pending_analysis'",
                )
                .fetch_all(self.pool)
                .await?
            }
            Some(ids) => {
                // SQLite não suporta IN com parâmetro de lista; filtra em memória
                let all = sqlx::query_as::<_, FileRecord>(
                    "SELECT id, path, relative_path, name, extension, size, hash, mime_type, scan_id, status, created_at, modified_at, indexed_at FROM files WHERE status = 'pending_analysis'",
                )
                .fetch_all(self.pool)
                .await?;
                all.into_iter().filter(|r| ids.contains(&r.id)).collect()
            }
        };
        Ok(rows)
    }

    /// Retorna o conteúdo textual de um arquivo.
    pub async fn get_content(&self, file_id: &str) -> AppResult<Option<String>> {
        let content: Option<String> =
            sqlx::query_scalar("SELECT content FROM file_contents WHERE file_id = ?")
                .bind(file_id)
                .fetch_optional(self.pool)
                .await?;
        Ok(content)
    }

    /// Conta arquivos com status DISCOVERED para um scan.
    pub async fn count_by_scan_and_status(
        &self,
        scan_id: &str,
        status: FileStatus,
    ) -> AppResult<i64> {
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM files WHERE scan_id = ? AND status = ?")
                .bind(scan_id)
                .bind(status.as_str())
                .fetch_one(self.pool)
                .await?;
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn pool_com_migracoes() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        crate::db::MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn upsert_discovered_insere_e_retorna_id() {
        let pool = pool_com_migracoes().await;
        let repo = FileRepository::new(&pool);

        let file = NewFile {
            scan_id: "scan-1".to_string(),
            path: "/tmp/teste.txt".to_string(),
            relative_path: "teste.txt".to_string(),
            name: "teste.txt".to_string(),
            extension: Some("txt".to_string()),
        };

        let id = repo.upsert_discovered(&file).await.unwrap();
        assert!(!id.is_empty());
    }

    #[tokio::test]
    async fn upsert_discovered_e_idempotente() {
        let pool = pool_com_migracoes().await;
        let repo = FileRepository::new(&pool);

        let file = NewFile {
            scan_id: "scan-1".to_string(),
            path: "/tmp/teste.txt".to_string(),
            relative_path: "teste.txt".to_string(),
            name: "teste.txt".to_string(),
            extension: Some("txt".to_string()),
        };

        let id1 = repo.upsert_discovered(&file).await.unwrap();
        let id2 = repo.upsert_discovered(&file).await.unwrap();
        assert_eq!(id1, id2);

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM files WHERE path = '/tmp/teste.txt'")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn update_status_altera_status_do_arquivo() {
        let pool = pool_com_migracoes().await;
        let repo = FileRepository::new(&pool);

        let file = NewFile {
            scan_id: "scan-1".to_string(),
            path: "/tmp/a.txt".to_string(),
            relative_path: "a.txt".to_string(),
            name: "a.txt".to_string(),
            extension: Some("txt".to_string()),
        };
        let id = repo.upsert_discovered(&file).await.unwrap();
        repo.update_status(&id, FileStatus::Indexed).await.unwrap();

        let status: String = sqlx::query_scalar("SELECT status FROM files WHERE id = ?")
            .bind(&id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(status, "indexed");
    }

    /// UC-002 CA-001: update_metadata persiste size, hash e mime_type.
    #[tokio::test]
    async fn update_metadata_persiste_dados() {
        let pool = pool_com_migracoes().await;
        let repo = FileRepository::new(&pool);

        let file = NewFile {
            scan_id: "scan-meta".to_string(),
            path: "/tmp/meta.txt".to_string(),
            relative_path: "meta.txt".to_string(),
            name: "meta.txt".to_string(),
            extension: Some("txt".to_string()),
        };
        let id = repo.upsert_discovered(&file).await.unwrap();
        repo.update_metadata(&id, 2048, "deadbeef", "text/plain")
            .await
            .unwrap();

        let row: (i64, String, String) =
            sqlx::query_as("SELECT size, hash, mime_type FROM files WHERE id = ?")
                .bind(&id)
                .fetch_one(&pool)
                .await
                .unwrap();

        assert_eq!(row.0, 2048);
        assert_eq!(row.1, "deadbeef");
        assert_eq!(row.2, "text/plain");
    }

    /// UC-002 CA-002: upsert_content persiste conteúdo e é idempotente.
    #[tokio::test]
    async fn upsert_content_persiste_e_idempotente() {
        let pool = pool_com_migracoes().await;
        let repo = FileRepository::new(&pool);

        let file = NewFile {
            scan_id: "scan-content".to_string(),
            path: "/tmp/content.txt".to_string(),
            relative_path: "content.txt".to_string(),
            name: "content.txt".to_string(),
            extension: Some("txt".to_string()),
        };
        let id = repo.upsert_discovered(&file).await.unwrap();

        let conteudo = FileContent {
            file_id: id.clone(),
            content: "texto extraído".to_string(),
            language: None,
            content_length: 14,
        };
        repo.upsert_content(&conteudo).await.unwrap();
        // Segunda inserção (idempotência via ON CONFLICT).
        repo.upsert_content(&FileContent {
            content: "texto atualizado".to_string(),
            content_length: 16,
            ..conteudo.clone()
        })
        .await
        .unwrap();

        let (content, count): (String, i64) = sqlx::query_as(
            "SELECT content, (SELECT COUNT(*) FROM file_contents WHERE file_id = ?) FROM file_contents WHERE file_id = ?"
        )
        .bind(&id)
        .bind(&id)
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(count, 1, "deve existir exatamente um registro");
        assert_eq!(content, "texto atualizado", "deve conter o valor mais recente");
    }

    /// UC-001 CA-004 / UC-002: find_discovered_by_scan retorna só arquivos do scan correto.
    #[tokio::test]
    async fn find_discovered_by_scan_retorna_arquivos_do_scan() {
        let pool = pool_com_migracoes().await;
        let repo = FileRepository::new(&pool);

        for i in 0..3 {
            let f = NewFile {
                scan_id: "scan-a".to_string(),
                path: format!("/tmp/scan-a-{i}.txt"),
                relative_path: format!("scan-a-{i}.txt"),
                name: format!("scan-a-{i}.txt"),
                extension: Some("txt".to_string()),
            };
            repo.upsert_discovered(&f).await.unwrap();
        }
        // Arquivo de outro scan — não deve aparecer.
        let outro = NewFile {
            scan_id: "scan-b".to_string(),
            path: "/tmp/outro.txt".to_string(),
            relative_path: "outro.txt".to_string(),
            name: "outro.txt".to_string(),
            extension: Some("txt".to_string()),
        };
        repo.upsert_discovered(&outro).await.unwrap();

        let resultado = repo.find_discovered_by_scan("scan-a").await.unwrap();
        assert_eq!(resultado.len(), 3);
        assert!(resultado.iter().all(|r| r.scan_id.as_deref() == Some("scan-a")));
    }

    /// UC-002: find_discovered_by_scan exclui arquivos com status diferente de discovered.
    #[tokio::test]
    async fn find_discovered_by_scan_exclui_nao_discovered() {
        let pool = pool_com_migracoes().await;
        let repo = FileRepository::new(&pool);

        let f = NewFile {
            scan_id: "scan-filtro".to_string(),
            path: "/tmp/filtro.txt".to_string(),
            relative_path: "filtro.txt".to_string(),
            name: "filtro.txt".to_string(),
            extension: Some("txt".to_string()),
        };
        let id = repo.upsert_discovered(&f).await.unwrap();
        repo.update_status(&id, FileStatus::Indexed).await.unwrap();

        let resultado = repo.find_discovered_by_scan("scan-filtro").await.unwrap();
        assert_eq!(resultado.len(), 0, "indexado não deve aparecer em discovered");
    }

    /// count_by_scan_and_status retorna contagem correta por status.
    #[tokio::test]
    async fn count_by_scan_and_status_retorna_contagem() {
        let pool = pool_com_migracoes().await;
        let repo = FileRepository::new(&pool);

        let scan = "scan-count";
        for i in 0..4 {
            let f = NewFile {
                scan_id: scan.to_string(),
                path: format!("/tmp/count-{i}.txt"),
                relative_path: format!("count-{i}.txt"),
                name: format!("count-{i}.txt"),
                extension: Some("txt".to_string()),
            };
            repo.upsert_discovered(&f).await.unwrap();
        }
        // Marca 2 como indexed.
        let todos = repo.find_discovered_by_scan(scan).await.unwrap();
        for arquivo in todos.iter().take(2) {
            repo.update_status(&arquivo.id, FileStatus::Indexed)
                .await
                .unwrap();
        }

        let discovered = repo
            .count_by_scan_and_status(scan, FileStatus::Discovered)
            .await
            .unwrap();
        let indexed = repo
            .count_by_scan_and_status(scan, FileStatus::Indexed)
            .await
            .unwrap();

        assert_eq!(discovered, 2);
        assert_eq!(indexed, 2);
    }
}
