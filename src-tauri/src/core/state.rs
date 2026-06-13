//! Estado compartilhado da aplicação, gerenciado pelo Tauri.

use sqlx::SqlitePool;

/// Estado global injetado nos commands. Nos próximos marcos receberá filas e
/// o serviço de IA.
pub struct AppState {
    pub db: SqlitePool,
}
