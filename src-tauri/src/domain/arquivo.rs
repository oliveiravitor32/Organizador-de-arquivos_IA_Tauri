//! Modelo de arquivo descoberto/indexado.

use serde::{Deserialize, Serialize};

/// Status do ciclo de vida — valores em minúsculas conforme o CHECK da migração.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FileStatus {
    Discovered,
    Indexed,
    PendingAnalysis,
    Analyzed,
    Failed,
}

impl FileStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileStatus::Discovered => "discovered",
            FileStatus::Indexed => "indexed",
            FileStatus::PendingAnalysis => "pending_analysis",
            FileStatus::Analyzed => "analyzed",
            FileStatus::Failed => "failed",
        }
    }
}

/// Registro de arquivo conforme a tabela `files`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FileRecord {
    pub id: String,
    pub path: String,
    pub relative_path: String,
    pub name: String,
    pub extension: Option<String>,
    pub size: Option<i64>,
    pub hash: Option<String>,
    pub mime_type: Option<String>,
    pub scan_id: Option<String>,
    pub status: String,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub indexed_at: Option<String>,
}

/// Conteúdo extraído de um arquivo (tabela `file_contents`).
#[derive(Debug, Clone)]
pub struct FileContent {
    pub file_id: String,
    pub content: String,
    pub language: Option<String>,
    pub content_length: i64,
}

/// Dados para inserir um novo arquivo descoberto.
#[derive(Debug, Clone)]
pub struct NewFile {
    pub scan_id: String,
    pub path: String,
    pub relative_path: String,
    pub name: String,
    pub extension: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_status_as_str_todos_os_valores() {
        assert_eq!(FileStatus::Discovered.as_str(), "discovered");
        assert_eq!(FileStatus::Indexed.as_str(), "indexed");
        assert_eq!(FileStatus::PendingAnalysis.as_str(), "pending_analysis");
        assert_eq!(FileStatus::Analyzed.as_str(), "analyzed");
        assert_eq!(FileStatus::Failed.as_str(), "failed");
    }

    #[test]
    fn file_status_eq_funciona() {
        assert_eq!(FileStatus::Discovered, FileStatus::Discovered);
        assert_ne!(FileStatus::Discovered, FileStatus::Indexed);
    }
}
