# Desenvolvimento: Integração Contínua

## Objetivo

Definir a estratégia de CI para garantir que toda mudança seja verificada automaticamente — em especial a regra "todo CA vira teste" (ADR-014).

---

# Plataforma

**GitHub Actions** (o repositório está hospedado no GitHub).

Workflows ficam em `.github/workflows/`.

---

# Princípios

## Verde para Integrar

Mudanças só são integradas com a pipeline passando.

---

## Rápido e Offline

A pipeline não depende de serviços externos de IA; testes usam dublês (mockall, mocks do Vitest). Coerente com local-first.

---

## Espelha o Local

O que roda no CI é o mesmo que o desenvolvedor roda localmente (`cargo test`, `npm run test`).

---

# Pipeline (por pull request e push)

## Backend (Rust)

1. `cargo fmt --check` — formatação.
2. `cargo clippy` — lint.
3. `cargo test` — testes.
4. `cargo llvm-cov` — cobertura (reportada, sem mínimo obrigatório — RNF-025).

---

## Frontend (React)

1. lint (ESLint).
2. `npm run test` — Vitest.
3. cobertura do Vitest (reportada).

---

## Build

Verificação de que o app compila (`npm run tauri build` em modo de verificação) nas plataformas-alvo (Windows, Linux — RNF-017).

---

# Política de Cobertura

- Cobertura é **medida e reportada** em cada execução.
- **Sem percentual mínimo que reprove o build.**
- A garantia vem da regra "todo CA tem teste" (ADR-014), verificada na revisão.

---

# Escopo por Fase

- **Marco 0** — workflow mínimo: lint + testes das duas camadas.
- **Marcos seguintes** — cada feature adiciona seus testes; a pipeline os executa automaticamente.
- **Build multiplataforma** — consolidado no Marco 6 (robustez).

---

# Release e Empacotamento

A geração de instaladores e a estratégia de release (Windows/Linux) serão definidas no **Marco 6**, fora do escopo do MVP inicial. Este documento será estendido então.

---

# Observação

Os arquivos de workflow são criados no Marco 0; este documento define a intenção e o escopo, não o YAML final.
