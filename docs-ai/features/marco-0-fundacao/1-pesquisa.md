# 1-pesquisa.md — Marco 0: Fundação

## Feature

Estabelecer o esqueleto técnico do projeto: app Tauri + React abrindo, conectado ao SQLite com o esquema criado, com a fronteira de commands/events funcional e a suíte de testes base. **Nenhum caso de uso de usuário é implementado neste marco.**

## Casos de uso envolvidos

Nenhum UC funcional. O marco é infraestrutura que habilita todos os UCs seguintes.

## Decisões aplicáveis

- [ADR-001](../../../docs/decisoes/ADR-001-react.md) — React no frontend.
- [ADR-002](../../../docs/decisoes/ADR-002-tauri.md) — Tauri como plataforma desktop.
- [ADR-005](../../../docs/decisoes/ADR-005-sqlite-persistencia.md) — SQLite como persistência.
- Regras inegociáveis do `CLAUDE.md` valem desde já (especialmente: apenas repositórios acessam o banco).

## Contratos e dados

- **Commands:** ainda nenhum command de domínio. Implementar apenas **um command de teste** (ex.: `ping` → versão do app) para validar a fronteira. Formato e convenções em [contratos-tauri.md](../../../docs/arquitetura/contratos-tauri.md).
- **Events:** validar emissão com **um evento de teste**. Convenção em [catalogo-de-eventos.md](../../../docs/arquitetura/catalogo-de-eventos.md).
- **Esquema:** criar **todas** as tabelas e índices de [esquema-sql.md](../../../docs/arquitetura/esquema-sql.md) via migração inicial, mesmo que ainda não usadas. Aplicar `PRAGMA foreign_keys = ON` e `journal_mode = WAL`; versionar via `user_version`.

## Onde mora o código

Conforme [estrutura-do-projeto.md](../../../docs/arquitetura/estrutura-do-projeto.md):

- `src/` — esqueleto React (`main.tsx`, `app/`, `ipc/`).
- `src-tauri/src/` — `main.rs`, `commands/`, `events/`, `core/state.rs`, `db/migrations/`, `error.rs`.
- `tests/` — `integracao/` e `e2e/`.

## Critérios de aceitação

Derivados do entregável do Marco 0 ([roadmap.md](../../../docs/roadmap.md)):

- CA-1: a aplicação abre e renderiza uma tela inicial vazia.
- CA-2: a conexão SQLite é estabelecida e todas as tabelas do esquema existem.
- CA-3: o frontend invoca o command de teste e recebe a resposta.
- CA-4: o frontend recebe um evento emitido pelo backend.
- CA-5: a suíte de testes roda (1 teste por nível) offline e passa.

## Riscos e questões em aberto

- **Versão do Tauri (v1 vs v2):** muda a API de commands/events e a config. → decidir no planejamento.
- **Biblioteca de acesso ao SQLite em Rust** (ex.: `sqlx`, `rusqlite`) e estratégia de migração. → decidir no planejamento.
- **Gerenciador de pacotes/bundler do frontend** (Vite assumido pelo template Tauri). → confirmar.
- Não há, ainda, doc de configuração/segurança dedicado — para o Marco 0 basta o escopo mínimo; não bloqueia.
