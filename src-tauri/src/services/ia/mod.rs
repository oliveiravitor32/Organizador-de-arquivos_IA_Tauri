//! Abstração do Serviço de IA (ADR-003, ADR-021).
//!
//! O pipeline de conhecimento depende deste trait, não do Ollama diretamente.
//! Isso permite trocar o runtime e usar `MockServicoIa` nos testes.

use async_trait::async_trait;

use crate::domain::conhecimento::{EntidadeExtraida, RelacaoInferida};
use crate::error::AppResult;

pub mod ollama;

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait ServicoIa: Send + Sync {
    /// Extrai entidades nomeadas do texto via LLM.
    async fn extrair_entidades(&self, texto: &str) -> AppResult<Vec<EntidadeExtraida>>;

    /// Gera vetor de embedding para o texto via modelo dedicado (ADR-018).
    async fn gerar_embedding(&self, texto: &str) -> AppResult<Vec<f32>>;

    /// Infere relações entre entidades a partir do texto de contexto.
    async fn inferir_relacoes(
        &self,
        texto: &str,
        entidades: &[String],
    ) -> AppResult<Vec<RelacaoInferida>>;

    /// Gera um nome curto (≤ 5 palavras) para um cluster a partir dos nomes dos arquivos membros.
    /// Recebe até MAX_FILES_LLM_NAMING nomes (os mais representativos do cluster).
    async fn gerar_nome_cluster(&self, nomes_arquivos: Vec<String>) -> AppResult<String>;

    /// Verifica se o Ollama está rodando e se os dois modelos estão disponíveis.
    async fn verificar_saude(&self) -> AppResult<()>;
}
