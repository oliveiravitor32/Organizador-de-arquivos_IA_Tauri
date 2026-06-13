# 3-execucao.md — Marco 0: Fundação

Derivado de `2-planejamento.md`. **Ainda não executado — aguardando validação do usuário.**

## Pré-requisito

✅ Decisões de ferramenta formalizadas em [ADR-011](../../../docs/decisoes/ADR-011-tauri-v2-sqlx-vite.md) (Tauri v2 + sqlx + Vite).

## Checklist de implementação

- [ ] **T1** — Scaffold Tauri v2 + React + Vite; app abre janela vazia.
- [ ] **T2** — Reorganizar pastas conforme `estrutura-do-projeto.md` (`src-tauri/src/{commands,events,core,services,db,domain}`, `error.rs`, `src/ipc/`).
- [ ] **T3** — Configurar `sqlx` + SQLite; `PRAGMA foreign_keys=ON`, `journal_mode=WAL`; pool no `core/state.rs`.
- [ ] **T4** — `db/migrations/0001_init.sql` com todas as tabelas e índices de `esquema-sql.md`; aplicar no boot; `user_version`.
- [ ] **T5** — `error.rs`: erro estruturado (`code`, `message`, `details`) serializável.
- [ ] **T6** — Command `ping` → versão do app; registrar na fronteira Tauri.
- [ ] **T7** — Evento de teste emitido pelo backend; wrappers em `src/ipc/{commands,events}.ts`.
- [ ] **T8** — Base de UI: Tailwind + shadcn/ui; `styles/globals.css` com tokens dos temas claro/escuro; alternador de tema com persistência; `components/ui/`. (ADR-012, `frontend-ui.md`)
- [ ] **T9** — Tela inicial: chama `ping`, escuta o evento, exibe resultado; respeita tokens e estados.
- [ ] **T10** — Ferramentas de teste (ADR-014, `convencoes-de-teste.md`): Vitest + RTL no front, `mockall` + cobertura (`cargo-llvm-cov`) no Rust; 1 teste unidade (erro, inline), 1 integração (migração cria tabelas), pasta `tests/e2e` pronta.

## Definition of Done

- [ ] CA-1: app abre e renderiza tela inicial.
- [ ] CA-2: migração cria todas as tabelas do esquema (verificado por teste).
- [ ] CA-3: `ping` responde ao frontend.
- [ ] CA-4: evento do backend chega ao frontend.
- [ ] CA-5: suíte roda offline e passa.
- [ ] Nenhuma violação das regras inegociáveis (`CLAUDE.md`).
- [ ] Acesso ao SQLite isolado (preparado para repositórios; nada de query fora de `db/`).
- [ ] UI usa só tokens de tema; ambos os temas (claro/escuro) na paleta `neutral`; alternador funcional.

## Verificação

1. `npm run tauri dev` — app abre sem erros; tela mostra a versão (via `ping`) e a mensagem do evento.
2. `cargo test` — teste de migração confirma tabelas; teste de erro passa.
3. Inspecionar o arquivo SQLite gerado e conferir as tabelas de `esquema-sql.md`.

## Registro

_(a preencher durante a execução: o que foi feito, desvios do plano, decisões novas)_

Status: **não iniciado** — aguardando validação do usuário para começar o Marco 0.
