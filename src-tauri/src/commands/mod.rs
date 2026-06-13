//! Commands expostos ao frontend (ver docs/arquitetura/contratos-tauri.md).
//!
//! No Marco 0 há apenas commands de teste, que validam a fronteira
//! React <-> Rust. Os commands de domínio chegam nos marcos seguintes.

use tauri::{AppHandle, Emitter};

use crate::error::{AppError, AppResult};
use crate::events;

/// Retorna a versão da aplicação. Command de teste do Marco 0.
#[tauri::command]
pub fn ping() -> AppResult<String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}

/// Emite o evento de prontidão para o frontend.
///
/// Chamado pelo frontend logo após registrar o listener, garantindo a
/// entrega do evento (evita a corrida de emitir no `setup`, antes de o
/// frontend estar escutando).
#[tauri::command]
pub fn announce_ready(app: AppHandle) -> AppResult<()> {
    app.emit(
        events::READY,
        events::ReadyPayload {
            message: "Backend pronto".to_string(),
        },
    )
    .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(())
}
