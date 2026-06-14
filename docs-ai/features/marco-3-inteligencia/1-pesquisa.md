# 1-pesquisa — Marco 3: Inteligência

## Feature

Transformar clusters semânticos do Marco 2 em sugestões de organização explicáveis, com nomeação via LLM e justificativa por template.

---

## Casos de uso envolvidos

| ID | Nome | Arquivo |
|---|---|---|
| UC-005 | Gerar Sugestões | `docs/casos-de-uso/03-inteligencia/UC-005-gerar-sugestoes.md` |
| UC-012 | Explicar Sugestões | `docs/casos-de-uso/03-inteligencia/UC-012-explicar-sugestoes.md` |

---

## Decisões aplicáveis

| ADR | Impacto |
|---|---|
| ADR-003 | Local-first — nenhum serviço externo obrigatório |
| ADR-007 | Aprovação obrigatória — sugestões não se aplicam automaticamente |
| ADR-009 | Motor orientado a conhecimento — sugestões derivam dos clusters |
| ADR-012 | UI: apenas tokens semânticos de tema |
| ADR-014 | Ferramentas de teste: Vitest+RTL frontend, `#[cfg(test)]`+mockall Rust |
| ADR-023 | LLM: 1 chamada por cluster (metadados), fallback por template |
| ADR-024 | Limiar de confiança mínima: 0,50 |
| ADR-025 | Algoritmo: cluster-based determinístico + LLM para nomeação + templates para justificativa |

---

## Contratos e dados

### Commands (contratos-tauri.md)

| Command | Entrada | Saída | UC |
|---|---|---|---|
| `gerar_sugestoes` | `{}` | `{ suggestionGenerationId }` | UC-005 |
| `explicar_sugestao` | `{ suggestionId }` | `{ justificativa, evidencias, confianca, desatualizada }` | UC-012 |
| `listar_sugestoes` | `{ status? }` | `{ sugestoes[] }` | Marco 4 (pré-requisito de UI) |

### Events (catalogo-de-eventos.md)

| Evento | Payload | Quando |
|---|---|---|
| `suggestion://started` | `{ suggestionGenerationId, total }` | Motor iniciado |
| `suggestion://created` | `{ suggestionId, titulo, confianca }` | Cada sugestão criada |
| `suggestion://completed` | `{ stats }` | Motor concluído |
| `suggestion://failed` | `{ error }` | Erro fatal |

### Tabelas (migration 0004)

```sql
suggestions (id, tipo, titulo, descricao, confianca, status, cluster_id, evidencias TEXT/JSON, criado_em)
suggestion_files (suggestion_id, file_id)
```

---

## Onde mora o código

```
src-tauri/src/
  commands/sugestoes.rs          ← novo: gerar_sugestoes, explicar_sugestao, listar_sugestoes
  services/sugestoes/
    mod.rs
    motor.rs                     ← SugestaoMotorService: detecção + nomeação LLM + templates
  db/repositories/
    sugestoes.rs                 ← novo: upsert_suggestion, add_suggestion_file, find_suggestions
  services/ia/mod.rs             ← estender ServicoIa com gerar_nome_cluster()
  domain/sugestoes.rs            ← Sugestao, SugestaoStats structs

src/
  features/sugestoes/
    Sugestoes.tsx                ← lista de sugestões com confiança e status
    DetalhesSugestao.tsx         ← justificativa + evidências (UC-012)
    Sugestoes.test.tsx
    DetalhesSugestao.test.tsx
  stores/sugestoes.ts
  stores/sugestoes.test.ts
  ipc/commands.ts                ← gerarSugestoes, explicarSugestao, listarSugestoes
  ipc/events.ts                  ← onSuggestionStarted, onSuggestionCreated, onSuggestionCompleted
  i18n/pt-BR.ts                  ← namespace `sugestoes`
```

---

## Critérios de aceitação

### UC-005

| CA | Descrição |
|---|---|
| CA-001 | O sistema gera sugestões baseadas em conhecimento (clusters) |
| CA-002 | Toda sugestão possui justificativa |
| CA-003 | Toda sugestão possui confiança |
| CA-004 | Sugestões podem ser revisadas pelo usuário |
| CA-005 | Nenhuma alteração física ocorre nesta etapa |

### UC-012

| CA | Descrição |
|---|---|
| CA-001 | Cada sugestão possui justificativa |
| CA-002 | As evidências são rastreáveis até o grafo (cluster_id, file_ids) |
| CA-003 | A confiança é exibida |
| CA-004 | Explicações desatualizadas são sinalizadas (`desatualizada: true`) |

---

## Riscos e questões em aberto

| Risco | Mitigation |
|---|---|
| LLM indisponível durante nomeação | Fallback por template: `"Grupo semântico com N arquivos"` |
| Cluster com arquivos todos no mesmo dir | Não gera sugestão (sem dispersão a resolver) |
| Nenhum cluster com confiança ≥ 0,50 | Tela mostra estado vazio com mensagem explicativa |
| `listar_sugestoes` é UC-013 (Marco 4) | Implementar subset mínimo aqui para a UI funcionar (só leitura) |
