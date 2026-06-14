//! Modelos de domínio do motor de sugestões (M3, UC-005).

use serde::{Deserialize, Serialize};

// ─── Enums ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SugestaoTipo {
    Agrupamento,
    MoverArquivo,
    RenomearArquivo,
    CriarPasta,
}

impl SugestaoTipo {
    pub fn as_str(&self) -> &'static str {
        match self {
            SugestaoTipo::Agrupamento => "agrupamento",
            SugestaoTipo::MoverArquivo => "mover_arquivo",
            SugestaoTipo::RenomearArquivo => "renomear_arquivo",
            SugestaoTipo::CriarPasta => "criar_pasta",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SugestaoStatus {
    Pendente,
    Aceita,
    Rejeitada,
    Executada,
}

impl SugestaoStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SugestaoStatus::Pendente => "pendente",
            SugestaoStatus::Aceita => "aceita",
            SugestaoStatus::Rejeitada => "rejeitada",
            SugestaoStatus::Executada => "executada",
        }
    }
}

// ─── Structs ──────────────────────────────────────────────────────────────────

/// Registro de sugestão na tabela `suggestions`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Sugestao {
    pub id: String,
    pub tipo: String,
    pub titulo: Option<String>,
    pub descricao: Option<String>,
    pub confianca: Option<f64>,
    pub status: String,
    pub cluster_id: Option<String>,
    pub evidencias: Option<String>, // JSON array serializado
    pub criado_em: String,
}

/// Operação concreta associada a uma sugestão (executada no Marco 4).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SugestaoOperacao {
    pub id: String,
    pub suggestion_id: String,
    pub tipo_operacao: String,
    pub payload: String, // JSON
}

/// Estatísticas da geração de sugestões (emitidas no evento suggestion://completed).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SugestaoStats {
    pub geradas: u64,
    pub descartadas: u64,
    pub duration_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sugestao_tipo_as_str() {
        assert_eq!(SugestaoTipo::Agrupamento.as_str(), "agrupamento");
        assert_eq!(SugestaoTipo::MoverArquivo.as_str(), "mover_arquivo");
        assert_eq!(SugestaoTipo::RenomearArquivo.as_str(), "renomear_arquivo");
        assert_eq!(SugestaoTipo::CriarPasta.as_str(), "criar_pasta");
    }

    #[test]
    fn sugestao_status_as_str() {
        assert_eq!(SugestaoStatus::Pendente.as_str(), "pendente");
        assert_eq!(SugestaoStatus::Aceita.as_str(), "aceita");
        assert_eq!(SugestaoStatus::Rejeitada.as_str(), "rejeitada");
        assert_eq!(SugestaoStatus::Executada.as_str(), "executada");
    }
}
