# 3-execucao — Marco 3: Inteligência

## Checklist de implementação

### Banco
- [ ] T1: Criar `src-tauri/migrations/0004_marco3_sugestoes.sql` (tables: suggestions, suggestion_files)

### Rust — Domain + Repositório
- [ ] T2: Criar `src-tauri/src/domain/sugestoes.rs` (Sugestao, SugestaoStats, SugestaoStatus, SugestaoTipo)
- [ ] T2: Registrar em `src-tauri/src/domain/mod.rs`
- [ ] T3: Criar `src-tauri/src/db/repositories/sugestoes.rs` (insert_suggestion, insert_suggestion_operation, find_suggestions, find_suggestion_by_id, find_operations_by_suggestion)
- [ ] T3: Registrar em `src-tauri/src/db/repositories/mod.rs`
- [ ] T3: Escrever testes de integração do repositório (≥ 4 testes)

### Rust — ServicoIa
- [ ] T4: Adicionar `gerar_nome_cluster` ao trait `ServicoIa` em `services/ia/mod.rs`
- [ ] T4: Implementar em `services/ia/ollama/mod.rs` com prompt e timeout 30 s
- [ ] T4: Escrever teste unitário com `MockServicoIa`

### Rust — Motor de Sugestões
- [ ] T5: Criar `src-tauri/src/services/sugestoes/motor.rs`
- [ ] T5: Criar `src-tauri/src/services/sugestoes/mod.rs`
- [ ] T5: Registrar em `src-tauri/src/services/mod.rs`
- [ ] T5: Escrever testes (cluster disperso, cluster mesmo dir, baixa confiança, fallback LLM)

### Rust — Commands
- [ ] T6: Criar `src-tauri/src/commands/sugestoes.rs` (gerar_sugestoes, explicar_sugestao, listar_sugestoes)
- [ ] T6: Registrar em `src-tauri/src/lib.rs`
- [ ] T6: Escrever testes de integração dos commands (explicar_sugestao, listar_sugestoes)

### Frontend — IPC
- [ ] T7: Adicionar `gerarSugestoes`, `explicarSugestao`, `listarSugestoes` em `src/ipc/commands.ts`
- [ ] T7: Adicionar `onSuggestionStarted`, `onSuggestionCreated`, `onSuggestionCompleted`, `onSuggestionFailed` em `src/ipc/events.ts`

### Frontend — Store
- [ ] T8: Criar `src/stores/sugestoes.ts`
- [ ] T8: Criar `src/stores/sugestoes.test.ts` (≥ 6 testes)

### Frontend — UI
- [ ] T9: Criar `src/features/sugestoes/Sugestoes.tsx`
- [ ] T9: Criar `src/features/sugestoes/Sugestoes.test.tsx` (CA-001, CA-003, CA-004 do UC-005)
- [ ] T10: Criar `src/features/sugestoes/DetalhesSugestao.tsx`
- [ ] T10: Criar `src/features/sugestoes/DetalhesSugestao.test.tsx` (CA-001..CA-004 do UC-012)
- [ ] T11: Adicionar `<Sugestoes />` em `src/App.tsx`
- [ ] T11: Adicionar mocks de novos commands/events em `src/App.test.tsx`
- [ ] T11: Adicionar namespace `sugestoes` em `src/i18n/pt-BR.ts`

---

## Definition of Done

- [ ] Cada CA tem ≥ 1 teste (ver mapeamento em `2-planejamento.md`)
- [ ] Testes unitários seguem `convencoes-de-teste.md`
- [ ] Contratos (commands/events) respeitados conforme `contratos-tauri.md`
- [ ] Sem violação das regras inegociáveis (CLAUDE.md)
- [ ] UI usa só tokens de tema (`text-foreground`, `bg-background`, `text-muted-foreground`, `text-destructive`)
- [ ] Sugestões não movem arquivos em nenhuma circunstância
- [ ] Fallback LLM funciona sem Ollama (título genérico, motor não falha)
- [ ] Suíte passa offline e determinística

---

## Verificação

```bash
# Rust
cd src-tauri && cargo test 2>&1 | tail -5

# Frontend
npm run test -- --run 2>&1 | tail -10

# Fluxo manual (com Ollama rodando):
# 1. Escanear diretório → Indexar → Analisar
# 2. Clicar "Gerar sugestões"
# 3. Verificar: lista com título, confiança, tipo "agrupamento"
# 4. Clicar em sugestão → verificar justificativa + evidências

# Fluxo manual (sem Ollama):
# 1. Parar Ollama
# 2. Repetir passos acima
# 3. Verificar: título fallback "Grupo semântico com N arquivos", motor não falha
```

---

## Registro

_Preencher após a implementação com desvios do plano, decisões emergentes e observações._
