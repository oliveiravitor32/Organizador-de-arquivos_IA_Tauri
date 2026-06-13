//! Erro estruturado compartilhado entre backend e frontend.
//!
//! Serializa sempre como `{ code, message, details }` (ver
//! docs/arquitetura/contratos-tauri.md).

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("erro de banco de dados: {0}")]
    Database(#[from] sqlx::Error),

    #[error("erro de migração: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("{0}")]
    Internal(String),
}

impl AppError {
    /// Código estável do erro, consumido pelo frontend.
    pub fn code(&self) -> &'static str {
        match self {
            AppError::Database(_) => "database_error",
            AppError::Migration(_) => "migration_error",
            AppError::Internal(_) => "internal_error",
        }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("AppError", 3)?;
        state.serialize_field("code", self.code())?;
        state.serialize_field("message", &self.to_string())?;
        state.serialize_field("details", &Option::<String>::None)?;
        state.end()
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_error_tem_codigo_e_serializa_estruturado() {
        let err = AppError::Internal("algo falhou".to_string());
        assert_eq!(err.code(), "internal_error");

        let value = serde_json::to_value(&err).expect("serializa");
        assert_eq!(value["code"], "internal_error");
        assert_eq!(value["message"], "algo falhou");
        assert!(value["details"].is_null());
    }
}
