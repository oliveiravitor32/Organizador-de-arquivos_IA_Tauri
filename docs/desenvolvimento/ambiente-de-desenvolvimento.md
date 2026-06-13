# Desenvolvimento: Ambiente de Desenvolvimento

## Objetivo

Guia para preparar uma máquina para desenvolver o projeto. É o ponto de partida de qualquer colaborador (e do Marco 0).

---

# Pré-requisitos

| Ferramenta | Uso | Observação |
| --- | --- | --- |
| **Rust** (stable) | backend / Tauri | via `rustup` |
| **Node.js** (LTS) | frontend / Vite | inclui `npm` |
| **Tauri v2 prereqs** | build desktop | dependências por SO (ver abaixo) |
| **Ollama** | IA local | runtime de inferência (ADR-008) |
| **Git** | versionamento | — |

---

# Dependências do Tauri por SO

## Windows

- Microsoft Visual Studio C++ Build Tools
- WebView2 (geralmente já presente no Windows 10/11)

## Linux

- pacotes de desenvolvimento do WebKitGTK e afins (conforme distribuição)

> A lista oficial e atualizada está na documentação do Tauri v2. Plataformas-alvo: Windows e Linux (RNF-017).

---

# Modelo de IA

```bash
# instalar o modelo inicial (ADR-008)
ollama pull qwen3:4b
```

O modelo é configurável (ver `configuracao-e-seguranca.md`); este é o padrão inicial.

---

# Primeira Execução

```bash
# instalar dependências do frontend
npm install

# rodar o app em desenvolvimento
npm run tauri dev
```

> Os scripts exatos são definidos no Marco 0 ao criar o `package.json`. Este guia será atualizado quando o scaffold existir.

---

# Rodando Testes

```bash
# backend (Rust)
cargo test
cargo llvm-cov          # cobertura

# frontend
npm run test
npm run test -- --coverage
```

Convenções e ferramentas em `requisitos/convencoes-de-teste.md` (ADR-014).

---

# Fluxo de Trabalho

A implementação segue o processo de `docs-ai/` (pesquisa → planejamento → execução por feature) e o **Gate de Marco** descrito no `CLAUDE.md`.

Convenções de contribuição (Git, commits) em `CONTRIBUTING.md`.

---

# Observação

Este documento evolui com o projeto. Após o Marco 0, comandos e versões exatas devem ser fixados aqui.
