# 3-execucao.md — Marco 0: Fundação

Derivado de `2-planejamento.md`. **Executado.**

## Pré-requisito

✅ Decisões de ferramenta formalizadas em [ADR-011](../../../docs/decisoes/ADR-011-tauri-v2-sqlx-vite.md) (Tauri v2 + sqlx + Vite).

## Checklist de implementação

- [x] **T1** — Scaffold Tauri v2 + React + Vite. _Já existia no repo (commit `71f7e6b`); construído sobre ele._
- [x] **T2** — Pastas conforme `estrutura-do-projeto.md`: criados `src-tauri/src/{commands,events,core,db}`, `error.rs`, `src/{ipc,components,stores,i18n,lib,styles}`. _`services/` e `domain/` adiados para quando houver conteúdo (não criados vazios)._
- [x] **T3** — `sqlx` + SQLite com `foreign_keys=ON` e `journal_mode=WAL`; pool no `AppState` (`core/state.rs`).
- [x] **T4** — `src-tauri/migrations/0001_init.sql` com todas as tabelas e índices. _Versionamento via `_sqlx_migrations` (padrão do sqlx), **não** `PRAGMA user_version` — ver desvio abaixo. Migração em `migrations/` (raiz do crate), não `db/migrations/`._
- [x] **T5** — `error.rs`: `AppError` serializa como `{code,message,details}`.
- [x] **T6** — Command `ping` → versão do app; registrado no `invoke_handler`.
- [x] **T7** — Evento `app://ready` emitido no `setup`; wrappers em `src/ipc/{commands,events}.ts`.
- [x] **T8** — Tailwind v4 (`@tailwindcss/vite`) + base shadcn (`cn`, `Button`); `globals.css` com tokens claro/escuro na paleta `neutral`; `ThemeToggle` com persistência (Zustand).
- [x] **T9** — Zustand (store de tema) + TanStack Query (`QueryClient` em `main.tsx`).
- [x] **T10** — `src/i18n/` (catálogo pt-BR + `useTranslations`); sem strings hardcoded.
- [x] **T11** — Tela inicial chama `ping` (via `usePing`), escuta `app://ready`, exibe versão/estado; usa tokens, estados (loading/erro) e i18n.
- [x] **T12** — Vitest + RTL (1 teste: `theme-toggle`); `cargo test` unidade (`error`, inline) + integração (`tests/integracao.rs`: migração cria tabelas). _`mockall` ainda não adicionado (M0 não tem trait a mockar); cobertura: `@vitest/coverage-v8` instalado; `cargo-llvm-cov` referenciado no CI mas não instalado localmente._
- [x] **T13** — `.github/workflows/ci.yml`: frontend (typecheck+test) e backend (fmt+clippy+test).

## Definition of Done

- [x] CA-1: app abre e renderiza tela inicial. _Confirmado em runtime (`tauri dev`): tela exibe título + "Versão: 0.1.0"._
- [x] CA-2: migração cria todas as tabelas (teste de integração ✅).
- [x] CA-3: `ping` responde ao frontend. _Confirmado: versão 0.1.0 exibida._
- [x] CA-4: evento do backend chega ao frontend. _Bug de corrida corrigido (listener antes de `announce_ready`) + **teste de regressão** (`src/App.test.tsx`). Re-teste em runtime recomendado para confirmação visual._
- [x] CA-5: suíte roda offline e passa (frontend 1/1, backend 2/2; `npm run build` e `tsc` ok).
- [x] Nenhuma violação das regras inegociáveis (`CLAUDE.md`).
- [x] Acesso ao SQLite isolado no módulo `db/`.
- [x] UI usa só tokens de tema; ambos os temas na paleta `neutral`; alternador funcional (teste ✅).

## Verificação

1. ✅ `cargo test` — unidade (`error`) + integração (migração) passam.
2. ✅ `npm run test` / `tsc --noEmit` / `npm run build` — passam.
3. ✅ `cargo fmt --check` — limpo.
4. ⏳ `npm run tauri dev` — **pendente** (precisa de ambiente gráfico; valida CA-1/CA-3/CA-4 ponta a ponta).

## Registro

Implementado sobre o scaffold pré-existente. Backend: módulos `commands/`, `events/`, `core/`, `db/`, `error.rs`; migração única com o esquema completo; `ping` + evento `app://ready` no `setup`. Frontend: Tailwind v4 + tokens `neutral`, Zustand (tema), TanStack Query (`usePing`), i18n pt-BR, tela inicial. Testes verdes nas duas camadas; `fmt` limpo.

### Desvios do plano (a reconciliar nos docs)
- **Migrações:** sqlx rastreia versão em `_sqlx_migrations`, então **não** uso `PRAGMA user_version`. Sugerido ajustar a nota em `esquema-sql.md`.
- **Local da migração:** `src-tauri/migrations/` (padrão do sqlx) em vez de `db/migrations/`. Sugerido ajustar `estrutura-do-projeto.md`/`convencoes`.
- **`mockall`:** não adicionado (sem trait a mockar no M0); entra no M2 com a abstração do Serviço de IA.
- **`cargo-llvm-cov`:** referenciado no CI; instalação local pendente.

### Correção pós-runtime (CA-4)
A 1ª execução em `tauri dev` confirmou CA-1 e CA-3 (título + "Versão: 0.1.0"), mas não exibiu "Backend pronto": o backend emitia `app://ready` no `setup`, antes de o frontend registrar o listener (corrida). Corrigido com o command `announce_ready`, chamado pelo frontend após registrar o listener. Adicionado teste de regressão `src/App.test.tsx` (ordenação listener→announce + render da mensagem) — política "todo bug vira teste" (ver convencoes-de-teste.md).

Status: **funcional, testes verdes (frontend 3/3, backend 2/2)** — CA-1/CA-3 confirmados em runtime; CA-4 corrigido e coberto por teste (confirmação visual em runtime recomendada).
