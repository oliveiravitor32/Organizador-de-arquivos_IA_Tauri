# 2-planejamento — Marco 2: Conhecimento

## Abordagem

O M2 é verticalmente dividido em 6 camadas que se constroem em sequência:

```
T1 — Dependências + migração 0003
T2 — Cliente Ollama (reqwest) + trait ServicoIa
T3 — Repositórios (entities, embeddings, relationships, clusters)
T4 — Serviços de IA (extração, embedding, relações, clusters)
T5 — AnaliseService (orquestrador UC-003) + command analisar_arquivos
T6 — Frontend (store + componente Analise + eventos analysis://)
```

Cada camada pode ser testada de forma independente antes de avançar para a próxima.

---

## Tarefas

### T1 — Dependências e Migração

- [ ] Adicionar ao `Cargo.toml`:
  - `ndarray = "0.16"` — operações matriciais para similaridade
  - `bytemuck = { version = "1", features = ["derive"] }` — serialização BLOB
  - `serde_json` (já existente, confirmar)
- [ ] Criar `src-tauri/migrations/0003_marco2_conhecimento.sql`:
  - Nenhuma nova tabela (esquema já existe desde M0)
  - Adicionar índices faltantes se necessário (ex: `idx_embeddings_file_id`)

### T2 — Cliente Ollama + Trait ServicoIa

- [ ] `src-tauri/src/services/ia/mod.rs` — trait `ServicoIa`:
  ```rust
  #[async_trait]
  #[cfg_attr(test, automock)]
  pub trait ServicoIa: Send + Sync {
      async fn extrair_entidades(&self, texto: &str) -> AppResult<Vec<EntidadeExtraida>>;
      async fn gerar_embedding(&self, texto: &str) -> AppResult<Vec<f32>>;
      async fn inferir_relacoes(&self, contexto: &str) -> AppResult<Vec<RelacaoInferida>>;
      async fn verificar_saude(&self) -> AppResult<bool>;
  }
  ```
- [ ] `src-tauri/src/services/ia/ollama/mod.rs` — struct `OllamaService { base_url, llm_model, embed_model }`
- [ ] `src-tauri/src/services/ia/ollama/health.rs` — GET /api/tags, verifica presença dos dois modelos
- [ ] `src-tauri/src/services/ia/ollama/generate.rs` — POST /api/generate com JSON mode; parse de entidades e relações
- [ ] `src-tauri/src/services/ia/ollama/embed.rs` — POST /api/embed; deserializa Vec<f32>; serializa como BLOB (bytemuck)
- [ ] Testes unitários com mock HTTP (wiremock ou respostas hardcoded)

### T3 — Repositórios

- [ ] `src-tauri/src/db/repositories/entities.rs`:
  - `upsert_entity(nome, tipo, confianca)` → String (id)
  - `link_file_entity(file_id, entity_id, confianca)` → upsert em file_entities
  - `find_entities_by_file(file_id)` → Vec<Entity>
- [ ] `src-tauri/src/db/repositories/embeddings.rs`:
  - `upsert_embedding(file_id, model, vector_blob, dims)` → AppResult
  - `find_all_embeddings()` → Vec<(file_id, Vec<f32>)> (para clusterização)
  - `find_embedding_by_file(file_id)` → Option<Vec<f32>>
- [ ] `src-tauri/src/db/repositories/relationships.rs`:
  - `upsert_relationship(source_id, target_id, tipo, confianca, evidencia)` → AppResult
  - `find_relationships_by_file(file_id)` → Vec<Relationship>
- [ ] `src-tauri/src/db/repositories/clusters.rs`:
  - `upsert_cluster(nome, descricao, confianca)` → String (id)
  - `add_cluster_member(cluster_id, member_id, member_type)` → AppResult
  - `find_clusters()` → Vec<Cluster>
- [ ] Testes de integração para todos os métodos (SQLite em memória)

### T4 — Serviços de IA

- [ ] `src-tauri/src/services/conhecimento/entidades.rs` — `ExtracaoEntidadesService`:
  - Recebe texto → chama `ServicoIa::extrair_entidades` → persiste via EntidadesRepo
  - Upsert de entidade por nome+tipo (deduplicação)
- [ ] `src-tauri/src/services/conhecimento/embeddings.rs` — `EmbeddingService`:
  - Recebe texto → chunking se > 8.192 tokens → chama `ServicoIa::gerar_embedding` → serializa BLOB → persiste
- [ ] `src-tauri/src/services/conhecimento/relacoes.rs` — `RelacoesService`:
  - Carrega entidades do arquivo → chama `ServicoIa::inferir_relacoes` → filtra confiança < 0.50 → persiste
- [ ] `src-tauri/src/services/conhecimento/clusters.rs` — `ClusterService`:
  - Carrega todos os embeddings → calcula matriz de cosseno (ndarray) → aplica threshold 0.75 → persiste clusters
- [ ] Todos os serviços recebem `&dyn ServicoIa` — testáveis com `MockServicoIa`

### T5 — AnaliseService + Command

- [ ] `src-tauri/src/services/conhecimento/analise.rs` — `AnaliseService`:
  - Orquestra UC-003: busca pending_analysis → para cada arquivo: extração → embedding → relações → atualiza status
  - Clusters rodados em batch ao final (UC-011 depende de todos os embeddings)
  - Cancela via `watch::Receiver<bool>` (mesmo padrão do M1)
  - Emite todos os eventos `analysis://`
- [ ] `src-tauri/src/commands/conhecimento.rs`:
  - `analisar_arquivos(app, state, scan_id)` → valida Ollama disponível → spawn AnaliseService → retorna `{ analysisId }`
- [ ] Adicionar ao `lib.rs`: `.invoke_handler` com `analisar_arquivos`, módulos

### T6 — Frontend

- [ ] `src/stores/analise.ts` — Zustand store (status, analysisId, progress, stats, erro)
- [ ] `src/stores/analise.test.ts` — testes unitários de todos os actions
- [ ] `src/ipc/commands.ts` — adicionar `analisarArquivos(scanId)`
- [ ] `src/ipc/events.ts` — adicionar listeners `analysis://`
- [ ] `src/features/conhecimento/Analise.tsx` — botão "Analisar", progresso, resultado, erro, cancelar
- [ ] `src/features/conhecimento/Analise.test.tsx` — testes de todos os estados visuais
- [ ] `src/i18n/pt-BR.ts` — namespace `analise`
- [ ] `src/App.tsx` — renderizar `<Analise />` após indexação concluída

---

## Pontos de integração

| Ponto | Detalhe |
| --- | --- |
| FileRepository | Busca `pending_analysis`; atualiza status para `analyzed`/`failed` |
| AppState | Cancel senders — mesmo mecanismo do M1 |
| Descoberta.tsx | Estado `indexing_done` deve habilitar botão "Analisar" em Analise.tsx |
| Ollama local | Deve estar rodando com os dois modelos puxados |

---

## Plano de testes

### Backend

| CA | Teste | Nível | Arquivo |
| --- | --- | --- | --- |
| UC-003 CA-001 | `analise_inicia_para_pending_analysis` | Integração | `tests/integracao.rs` |
| UC-003 CA-002 | `falha_em_arquivo_nao_interrompe_restante` | Integração | `tests/integracao.rs` |
| UC-003 CA-004 | `status_atualizado_para_analyzed_ou_failed` | Integração | `tests/integracao.rs` |
| UC-003 CA-005 | `ollama_ausente_retorna_erro_estruturado` | Unitário | `services/conhecimento/analise.rs` |
| UC-008 CA-001 | `entidades_persistidas_com_tipo_e_confianca` | Integração | `tests/integracao.rs` |
| UC-008 CA-002 | `entidade_duplicada_reutiliza_existente` | Unitário | `db/repositories/entities.rs` |
| UC-008 CA-003 | `sem_conteudo_nao_cria_entidade` | Unitário | `services/conhecimento/entidades.rs` |
| UC-009 CA-001 | `embedding_gerado_e_persistido` | Integração | `tests/integracao.rs` |
| UC-009 CA-002 | `embedding_registra_modelo` | Unitário | `db/repositories/embeddings.rs` |
| UC-009 CA-003 | `conteudo_extenso_e_segmentado` | Unitário | `services/conhecimento/embeddings.rs` |
| UC-009 CA-004 | `falha_embedding_nao_interrompe_pipeline` | Unitário | `services/conhecimento/embeddings.rs` |
| UC-010 CA-001 | `relacoes_persistidas_com_confianca` | Integração | `tests/integracao.rs` |
| UC-010 CA-003 | `relacao_abaixo_de_050_nao_e_persistida` | Unitário | `services/conhecimento/relacoes.rs` |
| UC-011 CA-001 | `arquivos_similares_agrupados_acima_threshold` | Unitário | `services/conhecimento/clusters.rs` |
| UC-011 CA-002 | `cluster_tem_confianca_media_dos_membros` | Unitário | `services/conhecimento/clusters.rs` |
| UC-011 CA-004 | `cluster_incoerente_e_sinalizado` | Unitário | `services/conhecimento/clusters.rs` |
| UC-004 CA-002 | `nos_existentes_sao_reutilizados` | Unitário | `db/repositories/entities.rs` |
| Serialização | `blob_f32_serializa_e_deserializa_corretamente` | Unitário | `services/ia/ollama/embed.rs` |
| Similaridade | `cosseno_de_vetores_identicos_e_um` | Unitário | `services/conhecimento/clusters.rs` |
| AppState cancel | já coberto no M1 | — | — |

### Frontend

| CA | Teste | Arquivo |
| --- | --- | --- |
| Estado inicial | `store começa idle` | `stores/analise.test.ts` |
| Todos os actions | `setAnalysisStarted`, `setProgress`, `setCompleted`, `setError`, `setCancelled`, `reset` | `stores/analise.test.ts` |
| UI: idle | `exibe botão analisar quando indexação concluída` | `Analise.test.tsx` |
| UI: progresso | `exibe progresso durante análise` | `Analise.test.tsx` |
| UI: concluído | `exibe estatísticas ao receber analysis://completed` | `Analise.test.tsx` |
| UI: erro | `exibe erro quando analysis://failed` | `Analise.test.tsx` |
| UI: cancelado | `exibe cancelado quando analysis://cancelled` | `Analise.test.tsx` |
| UI: Ollama ausente | `exibe erro de Ollama indisponível` | `Analise.test.tsx` |

---

## Decisões tomadas no planejamento

| Decisão | Escolha | Motivo |
| --- | --- | --- |
| Limite de chunking | 8.192 tokens | Limite conservador; nomic-embed-text suporta até 8.192 |
| Confiança mínima para relações | 0.50 | Definido em pipeline-ia.md |
| Threshold de clusterização | 0.75 | ADR-022; configurável no M6 |
| Clusters em batch | Após todos os arquivos do scan | UC-011 precisa de todos os embeddings para calcular matriz de cosseno |
| Concorrência do pipeline | 1 arquivo por vez (sequencial) | Hardware local modesto (ADR-008); sem perda de qualidade |
| Prompt de extração de entidades | JSON mode do Ollama com schema fixo | Parsing determinístico; evita ambiguidade de texto livre |
