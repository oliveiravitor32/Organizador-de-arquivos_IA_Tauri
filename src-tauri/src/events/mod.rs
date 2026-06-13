//! Catálogo de eventos emitidos para o frontend
//! (ver docs/arquitetura/catalogo-de-eventos.md).

use serde::Serialize;

/// Evento de teste emitido na inicialização, para validar a ponte
/// backend -> frontend no Marco 0.
pub const READY: &str = "app://ready";

#[derive(Clone, Serialize)]
pub struct ReadyPayload {
    pub message: String,
}
