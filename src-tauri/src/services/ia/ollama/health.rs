//! Verificação de saúde do Ollama: /api/tags.

use serde::Deserialize;

use crate::error::{AppError, AppResult};

#[derive(Deserialize)]
struct TagsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Deserialize)]
struct ModelInfo {
    name: String,
}

/// Verifica se o Ollama está rodando e se os modelos necessários estão puxados.
pub async fn verificar(
    client: &reqwest::Client,
    base_url: &str,
    llm_model: &str,
    embed_model: &str,
) -> AppResult<()> {
    let url = format!("{base_url}/api/tags");

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Ollama indisponível: {e}")))?;

    if !resp.status().is_success() {
        return Err(AppError::Internal(format!(
            "Ollama retornou status {}",
            resp.status()
        )));
    }

    let tags: TagsResponse = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("resposta inválida do Ollama: {e}")))?;

    let nomes: Vec<&str> = tags.models.iter().map(|m| m.name.as_str()).collect();

    // Verifica se o modelo está disponível (prefixo de nome, ex: "qwen3:4b" ou "qwen3")
    let tem_llm = nomes.iter().any(|n| n.starts_with(llm_model));
    let tem_embed = nomes.iter().any(|n| n.starts_with(embed_model));

    if !tem_llm {
        return Err(AppError::Internal(format!(
            "modelo LLM '{llm_model}' não encontrado no Ollama — execute: ollama pull {llm_model}"
        )));
    }
    if !tem_embed {
        return Err(AppError::Internal(format!(
            "modelo de embeddings '{embed_model}' não encontrado — execute: ollama pull {embed_model}"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_info_desserializa() {
        let json = r#"{"models":[{"name":"qwen3:4b"},{"name":"nomic-embed-text"}]}"#;
        let resp: TagsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.models.len(), 2);
        assert_eq!(resp.models[0].name, "qwen3:4b");
    }
}
