# Arquitetura: Contratos Tauri

## Objetivo

Este documento consolida os **commands** expostos pela camada Tauri — a fronteira entre o frontend (React) e o núcleo (Rust), descrita em `tauri.md`.

Cada comando representa uma operação que o frontend pode solicitar ao backend.

Os eventos emitidos durante a execução são detalhados em `catalogo-de-eventos.md`.

---

# Princípios dos Contratos

## Estabilidade

A assinatura de um comando deve permanecer estável.

Mudanças incompatíveis exigem versionamento.

---

## Validação no Backend

Toda entrada é validada no Rust antes de qualquer efeito.

---

## Erros Estruturados

Falhas retornam um erro tipado, nunca um estado inconsistente silencioso.

---

## Operações Longas via Eventos

Comandos demorados retornam imediatamente uma confirmação de início e comunicam o progresso por eventos.

---

# Formato Geral

Cada comando é descrito por:

- **Entrada** — parâmetros recebidos.
- **Saída** — retorno em caso de sucesso.
- **Erros** — falhas previstas.
- **Eventos** — eventos emitidos durante a execução.
- **UC** — caso de uso relacionado.

As assinaturas em bloco são ilustrativas e independem da implementação final.

---

# Tipos Comuns

```text
Uuid        identificador único
Path        caminho no sistema de arquivos
Confidence  número real entre 0 e 1
Timestamp   data e hora em ISO 8601
```

Erro padrão:

```json
{
  "code": "string",
  "message": "string",
  "details": "object | null"
}
```

---

# Descoberta

## escanear_diretorio

Inicia o escaneamento de um diretório raiz.

**Entrada**

```json
{ "rootPath": "Path", "ignore": ["string"] }
```

**Saída**

```json
{ "scanId": "Uuid" }
```

**Erros**

- caminho_invalido
- permissao_negada

**Eventos:** ScanStarted, DirectoryDiscovered, FileDiscovered, ScanProgress, ScanCompleted, ScanFailed, ScanCancelled

**UC:** UC-001

---

## indexar_arquivos

Inicia a indexação dos arquivos descobertos.

**Entrada**

```json
{ "scanId": "Uuid" }
```

**Saída**

```json
{ "indexingId": "Uuid" }
```

**Erros**

- scan_inexistente

**Eventos:** IndexingStarted, FileIndexingStarted, IndexingProgress, IndexingCompleted, IndexingFailed, IndexingCancelled

**UC:** UC-002

---

# Conhecimento

## analisar_arquivos

Inicia o pipeline de análise semântica dos arquivos pendentes.

**Entrada**

```json
{ "fileIds": ["Uuid"] | null }
```

Quando `null`, analisa todos os arquivos com status `pending_analysis`.

**Saída**

```json
{ "analysisId": "Uuid" }
```

**Eventos:** AnalysisStarted, EntityExtractionStarted, EmbeddingGenerationStarted, RelationsDiscovered, GraphUpdated, AnalysisCompleted, AnalysisFailed

**UC:** UC-003 (orquestra UC-008, UC-009, UC-010, UC-011)

---

## construir_grafo

Reconstrói ou atualiza o Grafo de Conhecimento a partir dos dados analisados.

**Entrada**

```json
{}
```

**Saída**

```json
{ "graphBuildId": "Uuid" }
```

**Eventos:** GraphBuildStarted, NodeCreated, NodeMerged, RelationCreated, ClusterUpdated, GraphUpdated, GraphBuildCompleted, GraphBuildFailed

**UC:** UC-004

---

# Inteligência

## gerar_sugestoes

Gera sugestões de organização a partir do grafo.

**Entrada**

```json
{}
```

**Saída**

```json
{ "suggestionGenerationId": "Uuid" }
```

**Eventos:** SuggestionGenerationStarted, SuggestionCreated, SuggestionGenerationCompleted

**UC:** UC-005

---

## explicar_sugestao

Retorna a explicação de uma sugestão.

**Entrada**

```json
{ "suggestionId": "Uuid" }
```

**Saída**

```json
{
  "suggestionId": "Uuid",
  "justificativa": "string",
  "evidencias": ["string"],
  "confianca": "Confidence",
  "desatualizada": "boolean"
}
```

**Eventos:** ExplanationRequested, ExplanationGenerated, ExplanationIncomplete

**UC:** UC-012

---

# Revisão

## listar_sugestoes

Lista sugestões filtradas por estado.

**Entrada**

```json
{ "status": "pending | approved | rejected | executed | null" }
```

**Saída**

```json
{ "sugestoes": [ { "id": "Uuid", "tipo": "string", "titulo": "string", "confianca": "Confidence", "status": "string" } ] }
```

**UC:** UC-013

---

## aprovar_sugestao

Aprova uma sugestão pendente.

**Entrada**

```json
{ "suggestionId": "Uuid" }
```

**Saída**

```json
{ "suggestionId": "Uuid", "status": "approved" }
```

**Eventos:** SuggestionApproved

**UC:** UC-013

---

## rejeitar_sugestao

Rejeita uma sugestão pendente.

**Entrada**

```json
{ "suggestionId": "Uuid" }
```

**Saída**

```json
{ "suggestionId": "Uuid", "status": "rejected" }
```

**Eventos:** SuggestionRejected

**UC:** UC-013

---

## ajustar_sugestao

Modifica uma sugestão antes da aprovação, preservando a rastreabilidade da original.

**Entrada**

```json
{ "suggestionId": "Uuid", "ajustes": "object" }
```

**Saída**

```json
{ "suggestionId": "Uuid", "status": "pending" }
```

**Eventos:** SuggestionAdjusted

**UC:** UC-013

---

# Execução

## aplicar_alteracoes

Executa as sugestões aprovadas, criando snapshot antes de qualquer alteração.

**Entrada**

```json
{ "suggestionIds": ["Uuid"] }
```

**Saída**

```json
{ "executionId": "Uuid", "snapshotId": "Uuid" }
```

**Erros**

- nenhuma_sugestao_aprovada
- conflito_de_nome
- permissao_negada

**Eventos:** ExecutionStarted, SnapshotCreated, OperationStarted, OperationCompleted, ExecutionProgress, ExecutionCompleted, ExecutionFailed, ExecutionCancelled

**UC:** UC-006

---

## desfazer_alteracoes

Reverte uma execução a partir do snapshot associado.

**Entrada**

```json
{ "executionId": "Uuid", "modo": "completo | parcial | arquivo | diretorio", "alvo": "Path | null" }
```

**Saída**

```json
{ "rollbackId": "Uuid" }
```

**Erros**

- snapshot_inexistente
- snapshot_corrompido
- conflito_de_caminho

**Eventos:** RollbackStarted, SnapshotLoaded, RollbackProgress, RollbackCompleted, RollbackFailed, RollbackCancelled

**UC:** UC-007

---

## listar_execucoes

Lista o histórico de execuções disponíveis para restauração.

**Entrada**

```json
{}
```

**Saída**

```json
{ "execucoes": [ { "executionId": "Uuid", "snapshotId": "Uuid", "data": "Timestamp", "descricao": "string" } ] }
```

**UC:** UC-007

---

# Exploração

## buscar_semantica

Busca arquivos por significado.

**Entrada**

```json
{ "consulta": "string", "modo": "conteudo | entidade | similaridade", "referenciaId": "Uuid | null" }
```

**Saída**

```json
{ "resultados": [ { "fileId": "Uuid", "relevancia": "Confidence" } ] }
```

**Eventos:** SearchStarted, SearchCompleted, SearchEmpty

**UC:** UC-014

---

## explorar_contexto

Retorna o contexto navegável de um elemento do grafo.

**Entrada**

```json
{ "nodeId": "Uuid", "tipo": "arquivo | entidade | cluster | contexto" }
```

**Saída**

```json
{
  "no": "object",
  "conexoes": [ { "destino": "Uuid", "tipo": "string", "confianca": "Confidence" } ]
}
```

**Eventos:** ContextExplorationStarted, NodeExpanded, ContextExplorationCompleted

**UC:** UC-015

---

# Controle de Operações

## cancelar_operacao

Cancela uma operação assíncrona em andamento.

**Entrada**

```json
{ "operationId": "Uuid" }
```

**Saída**

```json
{ "operationId": "Uuid", "status": "cancelando" }
```

Aplica-se a escaneamento, indexação, análise, execução e rollback.

---

# Configuração

## obter_configuracao

Retorna as configurações atuais do sistema.

**Saída**

```json
{ "modeloIa": "string", "itensIgnorados": ["string"], "ocrHabilitado": "boolean" }
```

---

## atualizar_configuracao

Atualiza configurações do sistema.

**Entrada**

```json
{ "modeloIa": "string | null", "itensIgnorados": ["string"] | null, "ocrHabilitado": "boolean | null" }
```

**Saída**

```json
{ "ok": true }
```

---

# Resumo

```text
Descoberta    escanear_diretorio · indexar_arquivos
Conhecimento  analisar_arquivos · construir_grafo
Inteligência  gerar_sugestoes · explicar_sugestao
Revisão       listar_sugestoes · aprovar/rejeitar/ajustar_sugestao
Execução      aplicar_alteracoes · desfazer_alteracoes · listar_execucoes
Exploração    buscar_semantica · explorar_contexto
Controle      cancelar_operacao
Configuração  obter/atualizar_configuracao
```

Todo comando respeita o escopo do diretório raiz e a autoridade do backend definidos em `tauri.md`.
