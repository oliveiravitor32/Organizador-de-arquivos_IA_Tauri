//! Adaptador Ollama — implementa ServicoIa via reqwest (ADR-021).

pub mod embed;
pub mod generate;
pub mod health;

use async_trait::async_trait;

use crate::domain::conhecimento::{EntidadeExtraida, RelacaoInferida};
use crate::error::AppResult;
use crate::services::ia::ServicoIa;

pub const DEFAULT_BASE_URL: &str = "http://localhost:11434";
pub const DEFAULT_LLM_MODEL: &str = "qwen3:0.6b";
pub const DEFAULT_EMBED_MODEL: &str = "nomic-embed-text";

pub struct OllamaService {
    client: reqwest::Client,
    base_url: String,
    llm_model: String,
    embed_model: String,
}

impl OllamaService {
    pub fn new(
        base_url: impl Into<String>,
        llm_model: impl Into<String>,
        embed_model: impl Into<String>,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .unwrap_or_default();

        Self {
            client,
            base_url: base_url.into(),
            llm_model: llm_model.into(),
            embed_model: embed_model.into(),
        }
    }

    pub fn default() -> Self {
        Self::new(DEFAULT_BASE_URL, DEFAULT_LLM_MODEL, DEFAULT_EMBED_MODEL)
    }
}

#[async_trait]
impl ServicoIa for OllamaService {
    async fn extrair_entidades(&self, texto: &str) -> AppResult<Vec<EntidadeExtraida>> {
        generate::extrair_entidades(&self.client, &self.base_url, &self.llm_model, texto).await
    }

    async fn gerar_embedding(&self, texto: &str) -> AppResult<Vec<f32>> {
        embed::gerar_embedding(&self.client, &self.base_url, &self.embed_model, texto).await
    }

    async fn inferir_relacoes(
        &self,
        texto: &str,
        entidades: &[String],
    ) -> AppResult<Vec<RelacaoInferida>> {
        generate::inferir_relacoes(
            &self.client,
            &self.base_url,
            &self.llm_model,
            texto,
            entidades,
        )
        .await
    }

    async fn gerar_nome_cluster(&self, nomes_arquivos: Vec<String>) -> AppResult<String> {
        generate::gerar_nome_cluster(
            &self.client,
            &self.base_url,
            &self.llm_model,
            nomes_arquivos,
        )
        .await
    }

    async fn verificar_saude(&self) -> AppResult<()> {
        health::verificar(
            &self.client,
            &self.base_url,
            &self.llm_model,
            &self.embed_model,
        )
        .await
    }
}
