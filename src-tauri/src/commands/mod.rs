//! Commands expostos ao frontend (ver docs/arquitetura/contratos-tauri.md).
//!
//! No Marco 0 há apenas o command de teste `ping`, que valida a fronteira
//! React <-> Rust. Os commands de domínio chegam nos marcos seguintes.

use crate::error::AppResult;

/// Retorna a versão da aplicação. Command de teste do Marco 0.
#[tauri::command]
pub fn ping() -> AppResult<String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}
