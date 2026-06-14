//! Modelos de domínio do pipeline de IA (M2).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntityType {
    Person,
    Organization,
    Project,
    Topic,
    Document,
}

impl EntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityType::Person => "person",
            EntityType::Organization => "organization",
            EntityType::Project => "project",
            EntityType::Topic => "topic",
            EntityType::Document => "document",
        }
    }
}

impl TryFrom<&str> for EntityType {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "person" => Ok(EntityType::Person),
            "organization" => Ok(EntityType::Organization),
            "project" => Ok(EntityType::Project),
            "topic" => Ok(EntityType::Topic),
            "document" => Ok(EntityType::Document),
            _ => Err(()),
        }
    }
}

/// Entidade extraída pelo LLM (antes de persistir).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntidadeExtraida {
    pub name: String,
    pub entity_type: EntityType,
    pub confidence: f64,
}

/// Relação inferida entre entidades pelo LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelacaoInferida {
    pub source: String,
    pub target: String,
    pub relation_type: String,
    pub confidence: f64,
}

/// Registro de entidade na tabela `entities`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Entity {
    pub id: String,
    pub name: String,
    #[sqlx(rename = "type")]
    pub entity_type: String,
    pub confidence: Option<f64>,
    pub created_at: Option<String>,
}

/// Registro de embedding na tabela `embeddings`.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Embedding {
    pub id: String,
    pub file_id: String,
    pub model: String,
    pub vector: Vec<u8>,
    pub created_at: Option<String>,
}

/// Registro de cluster.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Cluster {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub confidence: Option<f64>,
    pub created_at: Option<String>,
}

/// Estatísticas da análise de IA (emitidas no evento analysis://completed).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnaliseStats {
    pub processados: u64,
    /// Arquivos sem conteúdo textual extraível (imagens, binários, PDFs sem texto).
    pub sem_conteudo: u64,
    /// Erros reais durante geração de embeddings.
    pub falhos: u64,
    pub clusters_criados: u64,
    pub duration_ms: u64,
}

/// Progresso parcial durante a análise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnaliseProgress {
    pub analysis_id: String,
    pub processed: u64,
    pub total: u64,
    pub current_file: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_type_as_str_todos_os_valores() {
        assert_eq!(EntityType::Person.as_str(), "person");
        assert_eq!(EntityType::Organization.as_str(), "organization");
        assert_eq!(EntityType::Project.as_str(), "project");
        assert_eq!(EntityType::Topic.as_str(), "topic");
        assert_eq!(EntityType::Document.as_str(), "document");
    }

    #[test]
    fn entity_type_try_from_valido() {
        assert_eq!(EntityType::try_from("person"), Ok(EntityType::Person));
        assert_eq!(EntityType::try_from("topic"), Ok(EntityType::Topic));
    }

    #[test]
    fn entity_type_try_from_invalido() {
        assert!(EntityType::try_from("unknown").is_err());
    }
}
