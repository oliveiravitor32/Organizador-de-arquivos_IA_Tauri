# ADR-014 — Ferramentas de Teste

## Status

Aceito

---

## Data

2026-06-13

---

## Contexto

A `estrategia-de-testes.md` define os níveis de teste, mas não as ferramentas concretas. O Marco 0 cria a suíte base e cada feature seguinte exige testes unitários, então as ferramentas precisam estar decididas antes.

---

## Problema

Quais frameworks e ferramentas adotar para testes de unidade, mocks e cobertura, no frontend (React) e no backend (Rust)?

---

## Decisão

### Frontend (React)

- **Vitest** como framework de testes.
- **React Testing Library** para testes de componente.
- Cobertura via **Vitest coverage (v8)**.

### Backend (Rust)

- Test runner **nativo do Cargo** (`#[cfg(test)]` para unidade; diretório `tests/` para integração).
- **mockall** para test doubles de traits (incluindo a abstração do Serviço de IA).
- Cobertura via **cargo-llvm-cov**.

### Política

- Todo Critério de Aceitação de um caso de uso vira **≥ 1 teste** (regra de pronto).
- Cobertura é **medida e reportada**, sem percentual mínimo que reprove o build (metas suaves, coerente com RNF-025).

---

## Justificativa

- **Vitest** é nativo do ecossistema Vite (ADR-011): mesma configuração, rápido, API compatível com Jest.
- **React Testing Library** testa comportamento, não implementação — reduz testes frágeis.
- **mockall** encaixa na arquitetura de abstrações trocáveis (Serviço de IA, `pipeline-ia.md`).
- **cargo-llvm-cov** e cobertura do Vitest são padrões maduros, locais e sem dependência externa.

---

## Alternativas Consideradas

### Jest (frontend)

Rejeitado: exige configuração extra para Vite/ESM; menos alinhado ao stack.

---

### Dublês manuais (backend)

Rejeitado como padrão: repetitivo; `mockall` reduz boilerplate. Dublês manuais permanecem permitidos quando mais simples.

---

### Limite mínimo de cobertura

Rejeitado para o MVP: adiciona atrito; a regra "todo CA tem teste" garante a cobertura relevante sem perseguir percentual.

---

## Consequências

### Positivas

- Stack de teste coeso e alinhado às ferramentas do projeto.
- Mocks simples para a IA e demais serviços.
- Cobertura visível desde o início.

---

### Negativas

- `mockall` exige traits bem definidos (já é a direção da arquitetura).
- Sem limite de cobertura, é preciso disciplina para não deixar lacunas (mitigado pela regra por CA).

---

## Impacto

- **convencoes-de-teste.md** — detalha onde ficam os testes, nomenclatura e o fluxo por feature.
- **Marco 0** — instala Vitest + RTL + mockall e configura cobertura.
- **Todas as features** — herdam estas ferramentas e a regra "todo CA tem teste".

---

## Decisão Final

Ferramentas de teste: **Vitest + React Testing Library** (frontend), **Cargo test + mockall** (backend), cobertura por **cargo-llvm-cov** e **Vitest coverage**, com a política "todo CA vira teste; cobertura medida sem mínimo obrigatório".
