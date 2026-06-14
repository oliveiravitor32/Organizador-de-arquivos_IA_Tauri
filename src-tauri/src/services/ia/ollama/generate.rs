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
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GenerateOptions>,
}

#[derive(Serialize)]
struct GenerateOptions {
    temperature: f32,
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
        format: Some("json"),
        system: None,
        options: None,
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

    let parsed: EntidadesJson =
        serde_json::from_str(&gen.response).unwrap_or(EntidadesJson { entities: vec![] });

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
        format: Some("json"),
        system: None,
        options: None,
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

    let parsed: RelacoesJson =
        serde_json::from_str(&gen.response).unwrap_or(RelacoesJson { relations: vec![] });

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

// ─── Nome de Cluster ──────────────────────────────────────────────────────────

pub async fn gerar_nome_cluster(
    client: &reqwest::Client,
    base_url: &str,
    model: &str,
    nomes_arquivos: Vec<String>,
) -> AppResult<String> {
    let total = nomes_arquivos.len();
    let lista = nomes_arquivos
        .iter()
        .map(|n| format!("- {n}"))
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        "Analise os nomes dos arquivos abaixo e descubra o TEMA GERAL do grupo \
         (não foque em um arquivo específico). Crie um TÍTULO curto em PORTUGUÊS DO BRASIL \
         com no máximo 4 palavras, no estilo \"Documentos de X\" ou \"Grupo X\".\n\n\
         Exemplos de bons títulos:\n\
         - Documentos de TCC\n\
         - Notas fiscais 2024\n\
         - Artigos de Pesquisa\n\
         - Contratos do Projeto Alpha\n\n\
         REGRAS OBRIGATÓRIAS:\n\
         - O título DEVE estar em português do Brasil.\n\
         - NÃO use inglês em hipótese alguma.\n\
         - NÃO escreva explicações, JSON ou aspas.\n\
         - Responda APENAS com o título.\n\n\
         Arquivos do grupo (mostrando até 15 de {total} total):\n{lista}\n\n\
         Título em português:"
    );

    let body = GenerateRequest {
        model,
        prompt,
        stream: false,
        format: None, // texto puro — não forçar JSON
        system: Some(
            "Você é um assistente que SEMPRE responde em português do Brasil. \
             Sua tarefa é nomear grupos de arquivos de forma curta e descritiva. \
             Você NUNCA responde em inglês. Você NUNCA dá explicações. \
             Você responde apenas com o título solicitado, em uma única linha.",
        ),
        options: Some(GenerateOptions { temperature: 0.2 }),
    };

    let resp = client
        .post(format!("{base_url}/api/generate"))
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("erro ao chamar Ollama (nome cluster): {e}")))?;

    let gen: GenerateResponse = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("resposta inválida do Ollama: {e}")))?;

    let nome = sanitizar_nome_cluster(&gen.response);
    if nome.is_empty() {
        return Err(AppError::Internal("Ollama retornou nome vazio".into()));
    }

    Ok(nome)
}

/// Sanitiza a resposta do LLM: remove blocos <think>, JSON acidental, aspas
/// e limita a 5 palavras / 80 chars.
fn sanitizar_nome_cluster(raw: &str) -> String {
    let mut texto = raw.trim().to_string();

    // Remove blocos <think>...</think> (qwen3 às vezes inclui)
    while let (Some(ini), Some(fim)) = (texto.find("<think>"), texto.find("</think>")) {
        if fim > ini {
            texto.replace_range(ini..fim + "</think>".len(), "");
        } else {
            break;
        }
    }

    let texto = texto.trim();

    // Se o modelo devolveu JSON, tentar extrair campo "nome" ou similar
    if texto.starts_with('{') {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(texto) {
            for chave in &["nome", "titulo", "name", "title"] {
                if let Some(s) = val.get(chave).and_then(|v| v.as_str()) {
                    return finalizar_nome(s);
                }
            }
        }
    }

    finalizar_nome(texto)
}

fn finalizar_nome(s: &str) -> String {
    let limpo = s
        .trim()
        .trim_matches(|c: char| c == '"' || c == '\'' || c == '.' || c == ':' || c == '*')
        .trim();

    // Pega só a primeira linha não vazia (alguns modelos quebram com explicação extra)
    let primeira = limpo
        .lines()
        .find(|l| !l.trim().is_empty())
        .unwrap_or(limpo)
        .trim();

    // Limita a 5 palavras, máx 80 chars
    let palavras: Vec<&str> = primeira.split_whitespace().take(5).collect();
    let resultado = palavras.join(" ");
    if resultado.len() > 80 {
        resultado.chars().take(80).collect()
    } else {
        resultado
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitizar_nome_texto_puro() {
        assert_eq!(
            sanitizar_nome_cluster("Documentos Fiscais"),
            "Documentos Fiscais"
        );
    }

    #[test]
    fn sanitizar_nome_com_aspas() {
        assert_eq!(sanitizar_nome_cluster("\"Projeto Alpha\""), "Projeto Alpha");
    }

    #[test]
    fn sanitizar_nome_de_json() {
        let raw = r#"{ "arquivos": [{"arquivo":"x.pdf"}], "nome": "Revisão Bibliográfica TCC" }"#;
        assert_eq!(sanitizar_nome_cluster(raw), "Revisão Bibliográfica TCC");
    }

    #[test]
    fn sanitizar_nome_limita_5_palavras() {
        let raw = "Documentos do projeto alpha de finanças trimestre 2024";
        let r = sanitizar_nome_cluster(raw);
        assert_eq!(r.split_whitespace().count(), 5);
    }

    #[test]
    fn sanitizar_nome_remove_think_block() {
        let raw = "<think>vou pensar...</think>Projeto Alpha";
        assert_eq!(sanitizar_nome_cluster(raw), "Projeto Alpha");
    }

    #[test]
    fn sanitizar_nome_pega_primeira_linha() {
        let raw = "TCC Bibliografia\n\nExplicação extra que deve ser ignorada.";
        assert_eq!(sanitizar_nome_cluster(raw), "TCC Bibliografia");
    }

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
