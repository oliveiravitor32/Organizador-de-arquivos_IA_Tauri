# Arquitetura: Catálogo de Eventos

## Objetivo

Este documento consolida os **events** emitidos pelo backend para o frontend através da camada Tauri (ver `tauri.md`).

Os eventos comunicam progresso e mudanças de estado de operações assíncronas.

Os comandos que disparam esses eventos estão em `contratos-tauri.md`.

---

# Princípios

## Progresso sem Bloqueio

Operações longas não bloqueiam a interface; comunicam-se por eventos.

---

## Correlação por Identificador

Todo evento de uma operação carrega o identificador retornado pelo comando que a iniciou.

---

## Ciclo de Vida Previsível

Toda operação assíncrona segue o ciclo:

```text
Started → (Progress | eventos intermediários) → Completed | Failed | Cancelled
```

---

# Envelope Comum

Todo evento compartilha uma estrutura base:

```json
{
  "operationId": "Uuid",
  "timestamp": "Timestamp",
  "payload": "object"
}
```

O campo `payload` varia conforme o evento.

---

# Convenção de Progresso

Eventos de progresso seguem o formato:

```json
{
  "operationId": "Uuid",
  "processados": "number",
  "total": "number",
  "atual": "string | null"
}
```

---

# Descoberta (UC-001)

## ScanStarted

Escaneamento iniciado.

---

## DirectoryDiscovered

Diretório encontrado.

`payload`: `{ "path": "Path" }`

---

## FileDiscovered

Arquivo encontrado.

`payload`: `{ "path": "Path" }`

---

## ScanProgress

Progresso do escaneamento.

---

## ScanCompleted

Escaneamento concluído.

`payload`: `{ "totalArquivos": "number", "totalDiretorios": "number" }`

---

## ScanFailed

Falha no escaneamento.

`payload`: `{ "scanId": "Uuid", "error": "string" }`

---

## ScanCancelled

Escaneamento cancelado.

---

# Indexação (UC-002)

## IndexingStarted

Indexação iniciada.

---

## FileIndexingStarted

Indexação de um arquivo iniciada.

`payload`: `{ "fileId": "Uuid" }`

---

## IndexingProgress

Progresso da indexação.

---

## IndexingCompleted

Indexação concluída.

---

## IndexingFailed

Falha na indexação.

`payload`: `{ "indexingId": "Uuid", "error": "string" }`

---

## IndexingCancelled

Indexação cancelada.

---

# Análise (UC-003)

## AnalysisStarted

Análise iniciada.

---

## EntityExtractionStarted

Extração de entidades iniciada (UC-008).

---

## EmbeddingGenerationStarted

Geração de embeddings iniciada (UC-009).

---

## RelationsDiscovered

Relações identificadas (UC-010).

---

## GraphUpdated

Grafo atualizado.

---

## AnalysisCompleted

Análise concluída.

---

## AnalysisFailed

Falha na análise.

---

# Subprocessos de Conhecimento

Eventos emitidos pelos subprocessos quando executados de forma observável.

## Extrair Entidades (UC-008)

- EntityExtractionStarted
- EntityIdentified
- EntityReused
- EntityExtractionCompleted
- EntityExtractionFailed

---

## Gerar Embeddings (UC-009)

- EmbeddingGenerationStarted
- EmbeddingGenerated
- EmbeddingGenerationFailed

---

## Descobrir Relações (UC-010)

- RelationDiscoveryStarted
- RelationDiscovered
- RelationReinforced
- RelationDiscoveryCompleted
- RelationDiscoveryFailed

---

## Construir Clusters (UC-011)

- ClusteringStarted
- ClusterCreated
- ClusterUpdated
- ClusteringCompleted
- ClusteringFailed

---

# Construção do Grafo (UC-004)

## GraphBuildStarted

Construção iniciada.

---

## NodeCreated

Nó criado.

---

## NodeMerged

Nós consolidados.

---

## RelationCreated

Relação criada.

---

## ClusterUpdated

Cluster atualizado.

---

## GraphUpdated

Grafo atualizado.

---

## GraphBuildCompleted

Construção concluída.

---

## GraphBuildFailed

Falha na construção.

---

# Sugestões (UC-005)

## SuggestionGenerationStarted

Geração iniciada.

`payload`: `{ "suggestionGenerationId": "Uuid", "total": "number" }`

---

## SuggestionCreated

Sugestão criada.

`payload`: `{ "suggestionId": "Uuid", "titulo": "string", "confianca": "Confidence" }`

---

## SuggestionGenerationCompleted

Geração concluída.

`payload`: `{ "suggestionGenerationId": "Uuid", "stats": { "geradas": "number", "descartadas": "number", "durationMs": "number" } }`

---

## SuggestionGenerationFailed

Falha fatal durante a geração.

`payload`: `{ "suggestionGenerationId": "Uuid", "error": "string" }`

---

# Explicação (UC-012)

## ExplanationRequested

Explicação solicitada.

---

## ExplanationGenerated

Explicação produzida.

---

## ExplanationIncomplete

Explicação parcial gerada.

---

# Revisão (UC-013)

## ReviewStarted

Revisão iniciada.

---

## SuggestionApproved

Sugestão aprovada.

---

## SuggestionRejected

Sugestão rejeitada.

---

## SuggestionAdjusted

Sugestão ajustada.

---

## ReviewCompleted

Revisão concluída.

---

# Execução (UC-006)

## ExecutionStarted

Execução iniciada.

---

## SnapshotCreated

Snapshot concluído.

`payload`: `{ "snapshotId": "Uuid" }`

---

## OperationStarted

Operação individual iniciada.

---

## OperationCompleted

Operação individual concluída.

---

## ExecutionProgress

Progresso da execução.

---

## ExecutionCompleted

Execução concluída.

---

## ExecutionFailed

Falha crítica na execução.

---

## ExecutionCancelled

Execução cancelada.

---

# Rollback (UC-007)

## RollbackStarted

Restauração iniciada.

---

## SnapshotLoaded

Snapshot carregado.

---

## RollbackProgress

Progresso da restauração.

---

## RollbackCompleted

Restauração concluída.

---

## RollbackFailed

Falha na restauração.

---

## RollbackCancelled

Restauração cancelada.

---

# Busca Semântica (UC-014)

## SearchStarted

Busca iniciada.

---

## SearchCompleted

Busca concluída.

---

## SearchEmpty

Nenhum resultado encontrado.

---

# Exploração de Contexto (UC-015)

## ContextExplorationStarted

Exploração iniciada.

---

## NodeExpanded

Contexto expandido.

---

## ContextExplorationCompleted

Exploração concluída.

---

# Tabela Resumo

| Operação | Started | Progresso | Final |
| --- | --- | --- | --- |
| Escaneamento | ScanStarted | ScanProgress | ScanCompleted / Failed / Cancelled |
| Indexação | IndexingStarted | IndexingProgress | IndexingCompleted / Failed / Cancelled |
| Análise | AnalysisStarted | eventos de subprocesso | AnalysisCompleted / Failed |
| Grafo | GraphBuildStarted | NodeCreated / RelationCreated | GraphBuildCompleted / Failed |
| Sugestões | SuggestionGenerationStarted | SuggestionCreated | SuggestionGenerationCompleted / Failed |
| Execução | ExecutionStarted | ExecutionProgress | ExecutionCompleted / Failed / Cancelled |
| Rollback | RollbackStarted | RollbackProgress | RollbackCompleted / Failed / Cancelled |
| Busca | SearchStarted | — | SearchCompleted / Empty |
| Exploração | ContextExplorationStarted | NodeExpanded | ContextExplorationCompleted |

---

# Observação

Este catálogo é a fonte única dos nomes de eventos.

Os casos de uso descrevem os eventos no contexto de cada fluxo; divergências de nome devem ser resolvidas em favor deste documento.
