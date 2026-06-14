# 1-pesquisa — Marco 2: Conhecimento

## Feature

Transformar arquivos indexados em conhecimento estruturado: entidades, embeddings, relações, clusters e grafo de conhecimento.

---

## Casos de uso envolvidos

| UC | Nome | Papel |
| --- | --- | --- |
| UC-003 | Analisar Arquivos | Orquestrador do pipeline completo |
| UC-008 | Extrair Entidades | Etapa 1 do pipeline |
| UC-009 | Gerar Embeddings | Etapa 2 do pipeline |
| UC-010 | Descobrir Relações | Etapa 3 do pipeline |
| UC-011 | Construir Clusters | Etapa 4 do pipeline |
| UC-004 | Construir Grafo | Consolida o conhecimento |

Todos em `docs/casos-de-uso/02-conhecimento/`.

---

## Decisões aplicáveis

| ADR | Relevância |
| --- | --- |
| ADR-003 | IA local — nenhuma chamada externa |
| ADR-004 | Grafo é a fonte da verdade |
| ADR-006 | Pipeline de IA consome arquivos já indexados (status pending_analysis) |
| ADR-008 | Qwen 3 4B via Ollama — modelo de raciocínio (extração de entidades, relações) |
| ADR-018 | nomic-embed-text via Ollama — modelo de embeddings (768 dims) |
| ADR-019 | Cosseno em memória com ndarray — similaridade vetorial |
| ADR-020 | BLOB f32 little-endian — serialização de vetores |
| ADR-021 | reqwest com structs próprias — cliente HTTP do Ollama |
| ADR-022 | Threshold de cosseno 0.75 — algoritmo de clusterização |

---

## Contratos e dados

### Commands (contratos-tauri.md)

- `analisar_arquivos(scan_id)` → retorna `{ analysisId: String }`
- `cancelar_operacao(operation_id)` → já implementado (M1)

### Events (catalogo-de-eventos.md)

- `analysis://started { analysisId, scanId, total }`
- `analysis://file_started { analysisId, fileId }`
- `analysis://file_completed { analysisId, fileId, entidades, embeddings }`
- `analysis://file_failed { analysisId, fileId, reason }`
- `analysis://progress { analysisId, processed, total }`
- `analysis://completed { analysisId, processados, falhos, durationMs }`
- `analysis://cancelled { analysisId }`

### Tabelas (esquema-sql.md)

- `files` — leitura (status = pending_analysis) + update (status → analyzed / failed)
- `file_contents` — leitura (conteúdo para pipeline)
- `entities` — escrita
- `file_entities` — escrita
- `embeddings` — escrita (BLOB f32 LE, ADR-020)
- `relationships` — escrita
- `clusters` — escrita
- `cluster_members` — escrita

---

## Onde mora o código

```
src-tauri/src/
  services/
    ia/
      mod.rs                  # trait ServicoIa
      ollama/
        mod.rs                # adaptador reqwest
        generate.rs           # /api/generate (entidades, relações)
        embed.rs              # /api/embed (embeddings)
        health.rs             # /api/tags (verificação de modelos)
    conhecimento/
      mod.rs
      analise.rs              # AnaliseService (orquestrador, UC-003)
      entidades.rs            # extração de entidades (UC-008)
      embeddings.rs           # geração e persistência (UC-009)
      relacoes.rs             # descoberta de relações (UC-010)
      clusters.rs             # clusterização por threshold (UC-011)
      grafo.rs                # construção do grafo (UC-004)
  db/repositories/
    entities.rs
    embeddings.rs
    relationships.rs
    clusters.rs
  commands/
    conhecimento.rs           # analisar_arquivos
  domain/
    conhecimento.rs           # Entity, Embedding, Relation, Cluster

src/
  features/conhecimento/
    Analise.tsx               # UI de progresso da análise
    Analise.test.tsx
  stores/
    analise.ts                # Zustand store
    analise.test.ts
  ipc/
    commands.ts               # analisarArquivos (adição)
    events.ts                 # listeners analysis:// (adição)
```

---

## Critérios de aceitação por UC

### UC-003 (Analisar Arquivos)
- CA-001: pipeline inicia para arquivos com status `pending_analysis`
- CA-002: falha em um arquivo não interrompe o restante
- CA-003: análise parcial é persistida quando etapa falha
- CA-004: status atualizado para `analyzed` ou `failed` ao final
- CA-005: Ollama ausente emite evento de falha sem travar a UI

### UC-008 (Extrair Entidades)
- CA-001: entidades são extraídas e persistidas com tipo e confiança
- CA-002: entidades duplicadas são reutilizadas (upsert por nome+tipo)
- CA-003: nenhuma entidade é criada para arquivo sem conteúdo

### UC-009 (Gerar Embeddings)
- CA-001: embedding gerado e persistido para arquivo com conteúdo
- CA-002: embedding registra o modelo utilizado (nomic-embed-text)
- CA-003: conteúdo extenso é segmentado (>8.192 tokens)
- CA-004: falha de embedding não interrompe pipeline global
- CA-005: vetor disponível para busca e clusterização

### UC-010 (Descobrir Relações)
- CA-001: relações entre entidades são inferidas com confiança
- CA-002: relações duplicadas são consolidadas
- CA-003: relações abaixo de confiança 0.50 não são persistidas

### UC-011 (Construir Clusters)
- CA-001: arquivos com similaridade ≥ 0.75 são agrupados
- CA-002: cada cluster tem confiança (média de similaridade dos membros)
- CA-003: membros são associados ao cluster
- CA-004: clusters incoerentes (confiança < 0.50) são sinalizados
- CA-005: clusters ficam disponíveis para o grafo

### UC-004 (Construir Grafo)
- CA-001: grafo reflete entidades, relações e clusters persistidos
- CA-002: nós existentes são reutilizados (sem duplicação)
- CA-003: relações MENCIONA, SIMILAR_A, PERTENCE_A geradas corretamente

---

## Riscos e questões em aberto

| # | Questão | Impacto | Mitigação |
| --- | --- | --- | --- |
| 1 | Prompt para extração de entidades precisa ser calibrado — o Qwen 3 4B pode produzir saídas inconsistentes | Alto | Usar JSON mode do Ollama; testar com documentos reais; iterar no M6 |
| 2 | Ollama pode não estar instalado/rodando | Alto | Verificar saúde via /api/tags antes de iniciar; emitir evento de erro claro |
| 3 | Chunking de conteúdo extenso (FA-003 UC-009) precisa de limiar | Médio | Usar 8.192 tokens como limiar inicial; configurável no M6 |
| 4 | Serialização/deserialização de f32 BLOB precisa ser consistente entre plataformas | Médio | Usar `bytemuck` com endianness explícito (LE); testar em Windows |
