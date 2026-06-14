# 2-planejamento — Marco 3: Inteligência

## Abordagem

Pipeline sequencial em 4 fases executadas pelo `SugestaoMotorService`:

1. **Carregar clusters** — lê todos os clusters com `confiança ≥ 0,50` e seus arquivos membros.
2. **Filtrar oportunidades** — descarta clusters cujos arquivos estão todos no mesmo diretório (sem dispersão).
3. **Nomear via LLM** — para cada cluster oportunidade: 1 chamada `gerar_nome_cluster(nomes_arquivos)` → `titulo`. Se LLM indisponível → template.
4. **Persistir sugestões** — insere na tabela `suggestions` + `suggestion_files`. Emite eventos por sugestão criada.

A explicação (UC-012) é síncrona: `explicar_sugestao` lê a sugestão + cluster do banco e retorna justificativa montada por template + evidências (file paths, similaridade média).

---

## Tarefas

### T1 — Migration 0004 (banco)

Criar `src-tauri/migrations/0004_marco3_sugestoes.sql`:
- Tabela `suggestions`
- Tabela `suggestion_files`

### T2 — Domain types

Criar `src-tauri/src/domain/sugestoes.rs`:
- `struct Sugestao { id, tipo, titulo, descricao, confianca, status, cluster_id, evidencias, criado_em }`
- `struct SugestaoStats { geradas, descartadas, duration_ms }`
- `enum SugestaoStatus { Pendente, Aceita, Rejeitada }`
- `enum SugestaoTipo { Agrupamento }`

### T3 — Repositório de sugestões

Criar `src-tauri/src/db/repositories/sugestoes.rs`:
- `insert_suggestion(pool, sugestao)` — INSERT OR IGNORE
- `insert_suggestion_operation(pool, suggestion_id, tipo_operacao, payload_json)`
- `find_suggestions(pool, status: Option<&str>) -> Vec<Sugestao>`
- `find_suggestion_by_id(pool, id) -> Option<Sugestao>`
- `find_operations_by_suggestion(pool, suggestion_id) -> Vec<SugestaoOperacao>`

Registrar em `db/repositories/mod.rs`.

### T4 — Estender ServicoIa

Adicionar ao trait `ServicoIa` (`services/ia/mod.rs`):
```rust
async fn gerar_nome_cluster(&self, nomes_arquivos: Vec<String>) -> AppResult<String>;
```

Implementar em `OllamaService` com prompt e cap de 15 arquivos:
```
Você recebe nomes de arquivos de um grupo semântico (mostrando até 15 de N total).
Gere um nome curto (máximo 5 palavras) que descreva o tema comum deste grupo.
Responda apenas com o nome, sem explicações.

Arquivos:
{lista_truncada_em_15}
```

- Se cluster tiver > 15 membros: calcular o centróide semântico (média dos vetores
  de embedding dos membros) e selecionar os 15 arquivos com maior similaridade
  cosseno ao centróide — os mais representativos do grupo.
- Constante `MAX_FILES_LLM_NAMING: usize = 15` em `services/sugestoes/motor.rs`.
- Timeout: 30 s. Se erro → retornar `Err(...)` para que o motor use o fallback.

### T5 — SugestaoMotorService

Criar `src-tauri/src/services/sugestoes/motor.rs`:
- `processar(app, generation_id)` → `AppResult<SugestaoStats>`
- Pipeline: carregar clusters → filtrar → nomear → montar sugestão → persistir → emitir evento
- Constante `MIN_SUGGESTION_CONFIDENCE: f64 = 0.50`
- Justificativa por template:
  ```
  "{N} arquivos com similaridade semântica média de {X:.2} foram identificados
   no mesmo agrupamento e estão distribuídos em {Y} diretório(s) distintos."
  ```
- Evidências (JSON array):
  ```json
  [
    { "tipo": "arquivos_no_cluster", "valor": "4" },
    { "tipo": "similaridade_media", "valor": "0.87" },
    { "tipo": "diretorios_distintos", "valor": "3" }
  ]
  ```

Criar `src-tauri/src/services/sugestoes/mod.rs`.

### T6 — Commands

Criar `src-tauri/src/commands/sugestoes.rs`:
- `gerar_sugestoes(state, app) -> AppResult<{ suggestionGenerationId }>` — spawna task, retorna id
- `explicar_sugestao(state, suggestion_id) -> AppResult<ExplicacaoSugestao>` — síncrono
- `listar_sugestoes(state, status?) -> AppResult<{ sugestoes }>` — leitura direta

Registrar em `lib.rs`.

### T7 — Frontend: IPC

Atualizar `src/ipc/commands.ts`:
- `gerarSugestoes()`, `explicarSugestao(id)`, `listarSugestoes(status?)`

Atualizar `src/ipc/events.ts`:
- `onSuggestionStarted` — payload: `{ suggestionGenerationId, total }`
- `onSuggestionCreated` — payload: `{ suggestionId, titulo, confianca }` (conforme catálogo-de-eventos.md atualizado)
- `onSuggestionCompleted` — payload: `{ stats }`
- `onSuggestionFailed` — payload: `{ error }`

### T8 — Store frontend

Criar `src/stores/sugestoes.ts`:
- `status: "idle" | "generating" | "done" | "error"`
- `sugestoes: Sugestao[]`
- `erro: string | null`
- actions: `setStarted`, `addSugestao`, `setCompleted`, `setError`, `reset`

Criar `src/stores/sugestoes.test.ts`.

### T9 — UI: Tela de Sugestões

Criar `src/features/sugestoes/Sugestoes.tsx`:
- Botão "Gerar sugestões"
- Lista de sugestões com: título, tipo, badge de confiança (alta/média/baixa)
- Estado vazio com mensagem
- Estado de geração em progresso

Criar `src/features/sugestoes/Sugestoes.test.tsx`.

### T10 — UI: Detalhe da Sugestão (UC-012)

Criar `src/features/sugestoes/DetalhesSugestao.tsx`:
- Justificativa textual
- Evidências listadas
- Confiança exibida
- Badge `"explicação desatualizada"` se `desatualizada: true`

Criar `src/features/sugestoes/DetalhesSugestao.test.tsx`.

### T11 — Integrar em App.tsx + i18n

- Adicionar `<Sugestoes />` em `App.tsx`
- Adicionar namespace `sugestoes` em `src/i18n/pt-BR.ts`

---

## Pontos de integração

| Ponto | Detalhe |
|---|---|
| `clusters` (M2) | `SugestaoMotorService` lê `find_clusters` + `find_members_by_cluster` |
| `embeddings` (M2) | Usado para calcular similaridade média dos pares (evidência) |
| `ServicoIa` (M2) | Novo método `gerar_nome_cluster` |
| `AppState` (M0) | `store_resultado` para CA-HMR (se necessário no futuro) |

---

## Plano de testes

### Rust

| CA | Teste | Arquivo | Nível |
|---|---|---|---|
| UC-005 CA-001 | `motor_gera_sugestao_para_cluster_disperso` | `services/sugestoes/motor.rs` | Integração |
| UC-005 CA-001 | `motor_descarta_cluster_mesmo_diretorio` | `services/sugestoes/motor.rs` | Integração |
| UC-005 CA-001 | `motor_descarta_cluster_baixa_confianca` | `services/sugestoes/motor.rs` | Integração |
| UC-005 CA-002 | `sugestao_tem_justificativa_nao_vazia` | `services/sugestoes/motor.rs` | Unitário |
| UC-005 CA-003 | `sugestao_tem_confianca_entre_0_e_1` | `services/sugestoes/motor.rs` | Unitário |
| UC-005 CA-005 | `gerar_nao_move_arquivos` (implícito — só insert no banco) | — | N/A |
| UC-012 CA-001 | `explicar_retorna_justificativa` | `commands/sugestoes.rs` | Integração |
| UC-012 CA-002 | `evidencias_contem_cluster_id` | `commands/sugestoes.rs` | Integração |
| UC-012 CA-003 | `explicar_retorna_confianca` | `commands/sugestoes.rs` | Integração |
| Repositório | `upsert_suggestion`, `find_suggestions`, `find_files_by_suggestion` | `db/repositories/sugestoes.rs` | Integração |
| ServicoIa mock | `gerar_nome_cluster_mockado` | `services/sugestoes/motor.rs` | Unitário |

### Frontend

| CA | Teste | Arquivo |
|---|---|---|
| UC-005 CA-001 | Exibe sugestão após `onSuggestionCreated` | `Sugestoes.test.tsx` |
| UC-005 CA-003 | Exibe badge de confiança | `Sugestoes.test.tsx` |
| UC-005 CA-004 | Clicar em sugestão abre detalhe | `Sugestoes.test.tsx` |
| UC-012 CA-001 | Exibe justificativa no detalhe | `DetalhesSugestao.test.tsx` |
| UC-012 CA-002 | Exibe lista de evidências | `DetalhesSugestao.test.tsx` |
| UC-012 CA-003 | Exibe valor de confiança | `DetalhesSugestao.test.tsx` |
| UC-012 CA-004 | Exibe badge desatualizada quando `desatualizada: true` | `DetalhesSugestao.test.tsx` |
| Store | Todas as actions e transições de estado | `sugestoes.test.ts` |

---

## Decisões tomadas

| Decisão | Escolha | Motivo |
|---|---|---|
| Algoritmo | Cluster-based + LLM para nomeação | ADR-025 |
| Justificativa | Templates determinísticos | ADR-025; rastreável e sem dependência |
| Limiar de confiança | 0,50 | ADR-024; conforme UC-005 |
| Tipo de sugestão | Só `agrupamento` | ADR-025; único tipo viável sem entidades |
| Schema evidências | JSON inline (coluna TEXT) | ADR-025; sem over-engineering |
| Fallback LLM | Template: `"Grupo semântico com N arquivos"` | ADR-023; local-first deve funcionar sem Ollama |
