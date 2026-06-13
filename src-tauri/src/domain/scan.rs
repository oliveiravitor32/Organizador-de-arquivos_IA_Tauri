//! Modelos de estatísticas e progresso do escaneamento/indexação.

use serde::{Deserialize, Serialize};

/// Estatísticas finais do escaneamento (UC-001 CA-006).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScanStats {
    pub total_files: u64,
    pub total_dirs: u64,
    pub total_errors: u64,
    pub duration_ms: u64,
}

/// Progresso parcial emitido durante o escaneamento.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub scan_id: String,
    pub files_found: u64,
    pub dirs_found: u64,
    pub current_path: String,
}

/// Estatísticas finais da indexação (UC-002 CA-006).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IndexingStats {
    pub processed: u64,
    pub skipped: u64,
    pub failed: u64,
    pub duration_ms: u64,
}

/// Progresso parcial emitido durante a indexação.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingProgress {
    pub indexing_id: String,
    pub processed: u64,
    pub total: u64,
    pub current_file: String,
}
