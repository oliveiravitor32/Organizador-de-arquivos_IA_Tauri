//! Acesso ao SQLite via sqlx (ADR-005, ADR-011).

pub mod repositories;

use std::path::Path;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::SqlitePool;

use crate::error::AppResult;

/// Migrações embutidas em tempo de compilação a partir de `src-tauri/migrations`.
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

/// Cria o pool de conexões, criando o arquivo se necessário e habilitando
/// `foreign_keys` e o journal em modo WAL.
pub async fn create_pool(path: &Path) -> AppResult<SqlitePool> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new().connect_with(options).await?;
    Ok(pool)
}

/// Aplica todas as migrações pendentes.
pub async fn run_migrations(pool: &SqlitePool) -> AppResult<()> {
    MIGRATOR.run(pool).await?;
    Ok(())
}
