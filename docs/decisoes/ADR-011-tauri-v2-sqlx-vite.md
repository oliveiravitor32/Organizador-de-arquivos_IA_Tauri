# ADR-011 — Tauri v2, sqlx e Vite como Base Técnica

## Status

Aceito

---

## Data

2026-06-13

---

## Contexto

O Marco 0 (Fundação) exige escolher as ferramentas concretas que sustentarão toda a implementação: versão do Tauri, biblioteca de acesso ao SQLite em Rust e bundler do frontend.

Essas escolhas estavam implícitas nos ADRs anteriores (ADR-002 Tauri, ADR-005 SQLite, ADR-001 React), mas sem definição de versão ou bibliotecas específicas.

O processo `docs-ai` expôs essa lacuna ao planejar o Marco 0: havia decisões de ferramenta sem ADR, violando o princípio "nenhuma decisão fora dos ADRs".

---

## Problema

Quais versões e bibliotecas adotar para a fundação, de forma alinhada aos princípios local-first, modularidade e reprocessamento seguro?

Decisões necessárias:

- versão do Tauri
- biblioteca de acesso ao SQLite
- bundler do frontend

---

## Decisão

Adotar como base técnica do projeto:

- **Tauri v2** como plataforma desktop.
- **sqlx** (com driver SQLite) para acesso ao banco.
- **Vite** como bundler do frontend React.

---

## Justificativa

### Tauri v2

- API atual de commands, events e estado compartilhado.
- Melhor suporte e ciclo de vida ativo.
- Modelo de permissões mais granular, alinhado ao escopo de diretório raiz.

---

### sqlx

- Migrações versionadas em arquivos `.sql`.
- Verificação de queries em tempo de compilação.
- Assíncrono, coerente com o processamento não bloqueante da arquitetura.
- Mantém o acesso isolado em repositórios (ver `estrutura-do-projeto.md`).

---

### Vite

- Bundler padrão do template React do Tauri.
- Desenvolvimento rápido com HMR.
- Configuração mínima.

---

## Alternativas Consideradas

### rusqlite (em vez de sqlx)

Acesso síncrono e direto ao SQLite.

#### Motivo da Rejeição

Migrações manuais e ausência de verificação de queries em tempo de compilação; menos alinhado ao modelo assíncrono.

---

### Tauri v1

Versão anterior, mais difundida.

#### Motivo da Rejeição

API legada de eventos/permissões; sem vantagem que justifique iniciar um projeto novo nela.

---

## Consequências

### Positivas

- Base moderna e bem suportada.
- Migrações versionadas e seguras desde o início.
- Padrões claros para todos os marcos seguintes.

---

### Negativas

- Tauri v2 tem ecossistema de exemplos menor que v1.
- `sqlx` exige cuidado com a configuração de verificação em tempo de compilação (modo offline).

---

## Impacto

- **Marco 0** — define o scaffold e a migração inicial `0001_init.sql` com o esquema de `esquema-sql.md`.
- **estrutura-do-projeto.md** — confirma `db/migrations/` e repositórios sobre `sqlx`.
- **Todos os marcos** — herdam estas escolhas como base estável.

---

## Decisão Final

A base técnica do projeto é **Tauri v2 + sqlx + Vite**, formalizando as ferramentas concretas sobre as quais os ADRs anteriores serão implementados.
