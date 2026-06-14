//! Catálogo de eventos emitidos para o frontend
//! (ver docs/arquitetura/catalogo-de-eventos.md).
//!
//! Toda emissão de evento deve usar uma das constantes abaixo, nunca string
//! literal — assim renomeações ficam centralizadas e divergências entre
//! `scan://progres` (typo) e o catálogo são detectadas em compile time.

use serde::Serialize;

/// Evento de teste emitido na inicialização, para validar a ponte
/// backend -> frontend no Marco 0.
pub const READY: &str = "app://ready";

#[derive(Clone, Serialize)]
pub struct ReadyPayload {
    pub message: String,
}

// ─── Marco 1 — Descoberta (scan) ─────────────────────────────────────────────

pub const SCAN_STARTED: &str = "scan://started";
pub const SCAN_PROGRESS: &str = "scan://progress";
pub const SCAN_COMPLETED: &str = "scan://completed";
pub const SCAN_CANCELLED: &str = "scan://cancelled";
pub const SCAN_FAILED: &str = "scan://failed";
pub const SCAN_DIRECTORY_DISCOVERED: &str = "scan://directory_discovered";
pub const SCAN_FILE_DISCOVERED: &str = "scan://file_discovered";

// ─── Marco 1 — Descoberta (indexing) ─────────────────────────────────────────

pub const INDEXING_STARTED: &str = "indexing://started";
pub const INDEXING_PROGRESS: &str = "indexing://progress";
pub const INDEXING_COMPLETED: &str = "indexing://completed";
pub const INDEXING_CANCELLED: &str = "indexing://cancelled";
pub const INDEXING_FAILED: &str = "indexing://failed";
pub const INDEXING_FILE_STARTED: &str = "indexing://file_started";

// ─── Marco 2 — Conhecimento (analysis) ───────────────────────────────────────

pub const ANALYSIS_STARTED: &str = "analysis://started";
pub const ANALYSIS_PROGRESS: &str = "analysis://progress";
pub const ANALYSIS_EMBEDDING_GENERATION_STARTED: &str = "analysis://embedding_generation_started";
pub const ANALYSIS_GRAPH_UPDATED: &str = "analysis://graph_updated";
pub const ANALYSIS_COMPLETED: &str = "analysis://completed";
pub const ANALYSIS_FAILED: &str = "analysis://failed";

// ─── Marco 3 — Inteligência (suggestion) ─────────────────────────────────────

pub const SUGGESTION_STARTED: &str = "suggestion://started";
pub const SUGGESTION_CREATED: &str = "suggestion://created";
pub const SUGGESTION_COMPLETED: &str = "suggestion://completed";
pub const SUGGESTION_FAILED: &str = "suggestion://failed";
