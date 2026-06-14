//! Endpoint /api/embed do Ollama + serialização BLOB f32 LE (ADR-020).

use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

/// Limite de caracteres por chunk antes de truncar para o modelo de embeddings.
/// nomic-embed-text suporta até ~8.192 tokens; 1 token ≈ 4 chars → ~32.768 chars.
/// Usamos 30.000 para uma margem segura.
const MAX_CHARS_POR_CHUNK: usize = 30_000;

#[derive(Serialize)]
struct EmbedRequest<'a> {
    model: &'a str,
    input: &'a str,
}

#[derive(Deserialize)]
struct EmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

/// Gera embedding para o texto. Se o texto exceder o limite, trunca ao primeiro chunk (M6: multi-vector).
pub async fn gerar_embedding(
    client: &reqwest::Client,
    base_url: &str,
    model: &str,
    texto: &str,
) -> AppResult<Vec<f32>> {
    let chunk = truncar_chunk(texto);

    let body = EmbedRequest {
        model,
        input: chunk,
    };

    let resp = client
        .post(format!("{base_url}/api/embed"))
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("erro ao gerar embedding: {e}")))?;

    if !resp.status().is_success() {
        return Err(AppError::Internal(format!(
            "Ollama /api/embed retornou {}",
            resp.status()
        )));
    }

    let embed: EmbedResponse = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("resposta de embedding inválida: {e}")))?;

    embed
        .embeddings
        .into_iter()
        .next()
        .ok_or_else(|| AppError::Internal("Ollama retornou embeddings vazios".into()))
}

/// Serializa Vec<f32> como BLOB little-endian (ADR-020).
pub fn serializar_vetor(vetor: &[f32]) -> Vec<u8> {
    bytemuck::cast_slice(vetor).to_vec()
}

/// Deserializa BLOB little-endian para Vec<f32>.
pub fn deserializar_vetor(blob: &[u8]) -> Vec<f32> {
    bytemuck::cast_slice(blob).to_vec()
}

/// Trunca o texto ao primeiro chunk dentro do limite de tokens do modelo.
fn truncar_chunk(texto: &str) -> &str {
    if texto.len() <= MAX_CHARS_POR_CHUNK {
        return texto;
    }
    // Trunca no limite de char (não corta no meio de UTF-8)
    let mut idx = MAX_CHARS_POR_CHUNK;
    while !texto.is_char_boundary(idx) {
        idx -= 1;
    }
    &texto[..idx]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializa_e_deserializa_vetor_f32() {
        let original = vec![0.1f32, -0.5, 1.0, 0.0, f32::MAX];
        let blob = serializar_vetor(&original);
        let recuperado = deserializar_vetor(&blob);
        assert_eq!(original.len(), recuperado.len());
        for (a, b) in original.iter().zip(recuperado.iter()) {
            assert!((a - b).abs() < f32::EPSILON, "divergência: {a} != {b}");
        }
    }

    #[test]
    fn vetor_identico_serializa_como_blob_determinista() {
        let v = vec![1.0f32, 2.0, 3.0];
        assert_eq!(serializar_vetor(&v), serializar_vetor(&v));
    }

    #[test]
    fn truncar_chunk_texto_curto_retorna_inteiro() {
        let texto = "olá mundo";
        assert_eq!(truncar_chunk(texto), texto);
    }

    #[test]
    fn truncar_chunk_texto_longo_trunca_no_limite() {
        let texto = "a".repeat(MAX_CHARS_POR_CHUNK + 1000);
        let chunk = truncar_chunk(&texto);
        assert_eq!(chunk.len(), MAX_CHARS_POR_CHUNK);
    }

    #[test]
    fn truncar_chunk_respeita_fronteira_utf8() {
        // String com chars multi-byte na região do limite
        let base = "á".repeat(MAX_CHARS_POR_CHUNK / 2 + 1);
        let chunk = truncar_chunk(&base);
        // Verifica que o resultado é UTF-8 válido (não panic em chars())
        assert!(chunk.chars().count() > 0);
    }

    #[test]
    fn embed_response_deserializa() {
        let json = r#"{"embeddings":[[0.1,-0.2,0.3]]}"#;
        let resp: EmbedResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.embeddings.len(), 1);
        assert_eq!(resp.embeddings[0].len(), 3);
    }
}
