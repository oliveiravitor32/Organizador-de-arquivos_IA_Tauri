# ADR-023 — LLM usado somente na geração de sugestões (Marco 3)

**Status:** Aceito  
**Data:** 2026-06-14

---

## Contexto

Durante os testes do Marco 2, o uso do LLM (Qwen 3 4B) para extração de
entidades por arquivo mostrou-se inviável para uso diário:

- Cada arquivo exigia 1–2 chamadas de geração (~5–30 s cada)
- 48 arquivos = dezenas de minutos de processamento
- Alto consumo de CPU/GPU durante toda a análise

O pipeline de embeddings (`nomic-embed-text`) processou os mesmos 48 arquivos
em segundos e produziu 4 clusters semânticos — resultado já útil para
organização.

Testou-se também o `qwen3:0.6b` como alternativa mais leve, mas a latência
ainda tornou o fluxo pesado demais para uso cotidiano.

---

## Decisão

**O LLM não é chamado durante o pipeline de análise por arquivo (Marco 2).**

O Marco 2 executa apenas:
1. Geração de embeddings por arquivo (`nomic-embed-text`)
2. Clusterização semântica por similaridade cosseno (threshold 0.75)

**O LLM será usado somente no Marco 3**, para nomear clusters e gerar
sugestões de organização legíveis pelo usuário — com uma chamada por cluster
(não por arquivo), recebendo apenas metadados (nomes de arquivos, tamanhos,
datas), sem ler o conteúdo dos arquivos novamente.

---

## Alternativas consideradas

| Alternativa | Motivo da rejeição |
|---|---|
| LLM por arquivo (original) | Inviável — latência de minutos para dezenas de arquivos |
| Modelo menor (`qwen3:0.6b`) | Mais rápido, mas ainda lento o suficiente para ser impeditivo |
| Embeddings apenas (sem LLM nunca) | Viável, mas perde nomes legíveis para clusters e sugestões textuais |
| API externa (Claude/OpenAI) | Quebra ADR-003 (local-first); pode ser opt-in futuro |

---

## Consequências

- O `ServicoIa` e toda a infraestrutura Ollama permanecem no código — serão
  usados no Marco 3.
- `AnaliseStats` não expõe `entidades_criadas` como dado relevante no Marco 2
  (sempre zero).
- Se no futuro o LLM for removido do Marco 3 também, basta implementar uma
  heurística de nomeação por frequência de palavras nos nomes dos arquivos —
  a abstração `ServicoIa` isola essa troca.

---

## Referências

- ADR-006 — Indexação independente de IA  
- ADR-008 — Ollama como runtime de IA local  
- ADR-018 — Modelo de embeddings nomic-embed-text  
- ADR-022 — Clusterização por threshold cosseno
