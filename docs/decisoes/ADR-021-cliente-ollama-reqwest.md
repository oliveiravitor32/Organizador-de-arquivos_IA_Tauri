# ADR-021 — Cliente Ollama: reqwest com Structs Próprias

**Status:** Aceita  
**Data:** 2026-06-13  
**Marco:** M2 — Conhecimento

---

## Contexto

O pipeline de IA (UC-003, UC-008, UC-009) precisa se comunicar com a API HTTP do Ollama para inferência e geração de embeddings.

Dois endpoints principais são usados:
- `POST /api/generate` — geração de texto (extração de entidades, relações)
- `POST /api/embed` — geração de embeddings
- `GET /api/tags` — verificação de modelos disponíveis

Duas abordagens foram consideradas:
- **reqwest direto** com structs Rust próprias para serializar/deserializar as chamadas.
- **ollama-rs** (crate da comunidade que encapsula a API do Ollama).

---

## Decisão

Usar **reqwest** com structs Rust próprias, sem crate de abstração de terceiros.

---

## Motivações

- **API simples:** o Ollama expõe 3 endpoints que usaremos. Não justifica uma crate de abstração.
- **reqwest já é dependência:** o Tauri e outras dependências já puxam reqwest transitivamente; torná-la explícita tem custo zero.
- **Controle total:** structs próprias permitem mapear exatamente os campos necessários, sem depender de atualização da crate quando o Ollama muda a API.
- **ollama-rs:** historicamente com lag de semanas/meses para refletir mudanças de API do Ollama; sem garantia de manutenção de longo prazo.
- **Substituível:** a abstração do Serviço de IA (`pipeline-ia.md`) isola o adaptador — trocar reqwest por outra coisa no futuro não afeta o domínio.

---

## Consequências

### Positivas

- Sem dependência extra; zero risco de abandono de crate.
- Structs serializadas com `serde_json` (já dependência).
- Fácil de testar com mock do servidor HTTP.

### Negativas

- Mais código boilerplate para mapear os payloads da API.
- Mudanças na API do Ollama exigem atualização manual das structs.

---

## Structs Principais

```rust
// POST /api/generate
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    format: Option<String>, // "json" para structured output
}

struct OllamaGenerateResponse {
    response: String,
    done: bool,
}

// POST /api/embed
struct OllamaEmbedRequest {
    model: String,
    input: String,
}

struct OllamaEmbedResponse {
    embeddings: Vec<Vec<f32>>,
}
```

---

## Alternativa Rejeitada

**ollama-rs:** lag de atualização, abstração desnecessária para 3 endpoints, risco de abandono.
