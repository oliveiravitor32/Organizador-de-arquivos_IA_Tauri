# ADR-018 — Modelo de Embeddings: nomic-embed-text via Ollama

**Status:** Aceita  
**Data:** 2026-06-13  
**Marco:** M2 — Conhecimento

---

## Contexto

UC-009 (Gerar Embeddings) requer um modelo de embeddings para vetorizar o conteúdo dos arquivos.

O ADR-008 define o Qwen 3 4B como modelo de raciocínio (extração de entidades, relações). Embeddings servem a um propósito diferente — representação semântica vetorial — e se beneficiam de um modelo dedicado.

Duas opções foram consideradas:
- **nomic-embed-text**: modelo dedicado a embeddings, leve (~274 MB), disponível via Ollama.
- **Qwen 3 4B para tudo**: usar o mesmo modelo de raciocínio para gerar embeddings via `/api/embeddings`.

---

## Decisão

Adotar **nomic-embed-text** como modelo de embeddings, servido pelo mesmo Ollama já decidido em ADR-008.

Dimensão dos vetores: **768**.

O usuário precisará executar `ollama pull nomic-embed-text` além do `ollama pull qwen3:4b`.

---

## Motivações

- Modelos dedicados a embeddings produzem vetores de melhor qualidade semântica do que LLMs de raciocínio.
- nomic-embed-text é leve (~274 MB) e amplamente testado em tarefas de similaridade e busca.
- A separação entre modelo de raciocínio (Qwen) e modelo de embeddings (nomic) segue a arquitetura definida em `pipeline-ia.md`.
- A dimensão 768 é compacta o suficiente para armazenar como BLOB sem overhead excessivo (3 KB por vetor — ver ADR-020).

---

## Consequências

### Positivas

- Qualidade vetorial superior para clusterização e busca semântica.
- Dimensão fixa e documentada facilita o esquema da tabela `embeddings`.
- Separação limpa entre as responsabilidades dos dois modelos.

### Negativas

- O usuário precisa baixar um segundo modelo Ollama (~274 MB).
- A ausência do nomic-embed-text deve ser detectada na inicialização do pipeline e comunicada ao usuário.

---

## Alternativa Rejeitada

**Qwen 3 4B para embeddings:** qualidade inferior para representação semântica, dimensão de vetor dependente de versão do modelo, e acoplamento indesejado entre raciocínio e vetorização.
