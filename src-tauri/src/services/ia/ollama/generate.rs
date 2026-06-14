//! Chamadas ao endpoint /api/generate do Ollama (entidades e relações).

use serde::{Deserialize, Serialize};

use crate::domain::conhecimento::{EntidadeExtraida, EntityType, RelacaoInferida};
use crate::error::{AppError, AppResult};

// ─── Request/Response ────────────────────────────────────────────────────────

#[derive(Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: String,
    stream: bool,
    format: &'static str,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
}

// ─── Entidades ────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct EntidadesJson {
    #[serde(default)]
    entities: Vec<EntidadeJson>,
}

#[derive(Deserialize)]
struct EntidadeJson {
    name: String,
    #[serde(rename = "type")]
    entity_type: String,
    #[serde(default = "confianca_padrao")]
    confidence: f64,
}

fn confianca_padrao() -> f64 {
    0.75
}

pub async fn extrair_entidades(
    client: &reqwest::Client,
    base_url: &str,
    model: &str,
    texto: &str,
) -> AppResult<Vec<EntidadeExtraida>> {
    let prompt = format!(
        "Extract named entities from the text below. \
         Return ONLY a JSON object with this exact structure: \
         {{\"entities\": [{{\"name\": \"...\", \"type\": \"person|organization|project|topic|document\", \"confidence\": 0.0-1.0}}]}} \
         \nText:\n{texto}"
    );

    let body = GenerateRequest {
        model,
        prompt,
        stream: false,
        format: "json",
    };

    let resp = client
        .post(format!("{base_url}/api/generate"))
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("erro ao chamar Ollama: {e}")))?;

    let gen: GenerateResponse = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("resposta inválida do Ollama: {e}")))?;

    let parsed: EntidadesJson = serde_json::from_str(&gen.response).unwrap_or(EntidadesJson {
        entities: vec![],
    });

    let entidades = parsed
        .entities
        .into_iter()
        .filter_map(|e| {
            let entity_type = EntityType::try_from(e.entity_type.as_str()).ok()?;
            // Ignora entidades com nome vazio ou muito curto
            if e.name.trim().len() < 2 {
                return None;
            }
            Some(EntidadeExtraida {
                name: e.name.trim().to_string(),
                entity_type,
                confidence: e.confidence.clamp(0.0, 1.0),
            })
        })
        .collect();

    Ok(entidades)
}

// ─── Relações ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct RelacoesJson {
    #[serde(default)]
    relations: Vec<RelacaoJson>,
}

#[derive(Deserialize)]
struct RelacaoJson {
    source: String,
    target: String,
    #[serde(rename = "type")]
    relation_type: String,
    #[serde(default = "confianca_padrao")]
    confidence: f64,
}

const TIPOS_RELACAO_VALIDOS: &[&str] = &["related_to", "parent_of", "derived_from"];

pub async fn inferir_relacoes(
    client: &reqwest::Client,
    base_url: &str,
    model: &str,
    texto: &str,
    entidades: &[String],
) -> AppResult<Vec<RelacaoInferida>> {
    if entidades.len() < 2 {
        return Ok(vec![]);
    }

    let lista = entidades.join(", ");
    let prompt = format!(
        "Given these entities found in a document, identify relationships between them. \
         Return ONLY a JSON object: \
         {{\"relations\": [{{\"source\": \"entity name\", \"target\": \"entity name\", \
         \"type\": \"related_to|parent_of|derived_from\", \"confidence\": 0.0-1.0}}]}} \
         \nEntities: {lista}\nContext excerpt:\n{texto}"
    );

    let body = GenerateRequest {
        model,
        prompt,
        stream: false,
        format: "json",
    };

    let resp = client
        .post(format!("{base_url}/api/generate"))
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("erro ao chamar Ollama (relações): {e}")))?;

    let gen: GenerateResponse = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("resposta inválida do Ollama: {e}")))?;

    let parsed: RelacoesJson = serde_json::from_str(&gen.response).unwrap_or(RelacoesJson {
        relations: vec![],
    });

    let relacoes = parsed
        .relations
        .into_iter()
        .filter(|r| {
            !r.source.is_empty()
                && !r.target.is_empty()
                && TIPOS_RELACAO_VALIDOS.contains(&r.relation_type.as_str())
                && r.confidence >= 0.50
        })
        .map(|r| RelacaoInferida {
            source: r.source,
            target: r.target,
            relation_type: r.relation_type,
            confidence: r.confidence.clamp(0.0, 1.0),
        })
        .collect();

    Ok(relacoes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_entidades_json_valido() {
        let json = r#"{"entities":[{"name":"João Silva","type":"person","confidence":0.9}]}"#;
        let parsed: EntidadesJson = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.entities.len(), 1);
        assert_eq!(parsed.entities[0].name, "João Silva");
    }

    #[test]
    fn parse_entidades_json_invalido_retorna_vazio() {
        let parsed: EntidadesJson =
            serde_json::from_str("não é json").unwrap_or(EntidadesJson { entities: vec![] });
        assert!(parsed.entities.is_empty());
    }

    #[test]
    fn tipo_invalido_e_filtrado() {
        let json = r#"{"entities":[{"name":"X","type":"desconhecido","confidence":0.8}]}"#;
        let parsed: EntidadesJson = serde_json::from_str(json).unwrap();
        let result: Vec<EntidadeExtraida> = parsed
            .entities
            .into_iter()
            .filter_map(|e| {
                let t = EntityType::try_from(e.entity_type.as_str()).ok()?;
                Some(EntidadeExtraida {
                    name: e.name,
                    entity_type: t,
                    confidence: e.confidence,
                })
            })
            .collect();
        assert!(result.is_empty());
    }

    #[test]
    fn relacao_abaixo_de_050_e_filtrada() {
        let rel = RelacaoJson {
            source: "A".into(),
            target: "B".into(),
            relation_type: "related_to".into(),
            confidence: 0.3,
        };
        assert!(rel.confidence < 0.50);
    }

    #[test]
    fn tipo_relacao_invalido_e_rejeitado() {
        assert!(!TIPOS_RELACAO_VALIDOS.contains(&"similar_to"));
        assert!(TIPOS_RELACAO_VALIDOS.contains(&"related_to"));
    }
}
