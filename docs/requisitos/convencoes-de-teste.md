# Requisitos: Convenções de Teste

## Objetivo

Este documento define **como e onde** escrever testes — sobretudo unitários — para que toda feature nasça testada e documentada.

Complementa a `estrategia-de-testes.md` (que define *o quê* e em quais níveis testar) com as **convenções concretas de arquivos, nomenclatura e ferramentas**.

Ferramentas decididas em ADR-014.

---

# Ferramentas

| Camada | Unidade | Mock | Cobertura |
| --- | --- | --- | --- |
| Frontend (React) | Vitest + React Testing Library | utilitários do Vitest (`vi.fn`/`vi.mock`) | Vitest coverage (v8) |
| Backend (Rust) | `cargo test` (`#[cfg(test)]`) | mockall | cargo-llvm-cov |

---

# Onde Mora Cada Teste

## Backend (Rust)

### Unidade

**Inline no próprio módulo**, em bloco `#[cfg(test)]` ao final do arquivo.

```rust
// src-tauri/src/services/operacoes/inverse.rs

pub fn inverse_of(op: &Operation) -> Operation { /* ... */ }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_inverso_troca_origem_e_destino() { /* ... */ }
}
```

### Integração

No diretório `src-tauri/tests/integracao/`.

---

## Frontend (React)

### Unidade / Componente

**Colocado ao lado do arquivo testado**, com sufixo `.test.ts` / `.test.tsx`.

```text
src/features/sugestoes/
├── useSugestoes.ts
├── useSugestoes.test.ts        # teste de hook/lógica
├── ListaSugestoes.tsx
└── ListaSugestoes.test.tsx     # teste de componente (RTL)
```

### Integração / e2e

No diretório `tests/` da raiz (`integracao/`, `e2e/`).

---

# Nomenclatura

## Backend

- Módulo de teste: `mod tests`.
- Função: `fn <comportamento_esperado>()` — descritiva, em português, sem prefixo redundante.
  - Bom: `rejeita_caminho_fora_do_diretorio_raiz`.
  - Evitar: `test1`, `test_funcao`.

## Frontend

- Arquivo: `<NomeDoArquivo>.test.ts(x)`.
- `describe('<unidade>')` + `it('<comportamento esperado>')`.

---

# O que é Teste Unitário Aqui

- Testa **uma unidade isolada** (função, regra, hook, componente).
- **Determinístico** e rápido; sem rede, sem disco real, sem IA real.
- Dependências externas (IA, banco, FS) são substituídas por mocks/dublês.

> Banco real, FS real e pipeline completo pertencem a **integração/e2e** (ver `estrategia-de-testes.md`), não a unidade.

---

# Mocks

## Serviço de IA

Sempre mockado em testes unitários. Com **mockall** sobre o trait do Serviço de IA (ver `pipeline-ia.md`).

```rust
#[automock]
trait ServicoIa { /* extrair_entidades, gerar_embedding, ... */ }
```

## Frontend

Commands e events são mockados via `vi.mock` sobre `src/ipc/` — o componente nunca chama o backend real em unidade.

---

# Cobertura por Camada

A regra "todo CA vira teste" garante cobertura do comportamento externo. Mas CAs não exercitam diretamente toda a lógica interna — por isso existem as expectativas abaixo por camada.

## Backend (Rust)

| Camada | O que testar | Nível |
| --- | --- | --- |
| **Repositórios** (`db/repositories/`) | Cada método público: insert, update, select, upsert, count | Integração (SQLite em memória) |
| **Serviços sem AppHandle** | Cada função pública e privada com lógica real (cálculos, filtros, transformações) | Unitário |
| **Serviços com AppHandle** | Lógica extraível em funções puras é testada isoladamente; o método principal é coberto por integração | Unitário (lógica pura) + Integração |
| **Domain models com lógica** | Métodos como `as_str()`, `From`, validações — não testar structs sem lógica | Unitário inline |
| **Estado compartilhado** (`core/state.rs`) | Cada método público do AppState | Unitário |
| **Extratores** | Caminho feliz + arquivo inválido/corrompido para cada formato | Unitário |

**Não testar:** structs puramente de dados (sem impl), constantes, boilerplate de `new()` triviais.

## Frontend (TypeScript/React)

| Camada | O que testar | Nível |
| --- | --- | --- |
| **Stores Zustand** | Cada action em isolamento; estado inicial; reset | Unitário (`*.test.ts`) |
| **Componentes** | Cada estado visual (idle, loading, success, error, cancelled) e cada interação relevante | Componente (RTL) |
| **Hooks customizados** | Cada estado e transição do hook | Unitário (renderHook) |
| **Funções utilitárias puras** | Cada transformação/validação em `lib/` ou `utils/` | Unitário |

**Não testar:** wrappers de IPC (`ipc/commands.ts`, `ipc/events.ts`) — são chamadas diretas ao Tauri, sem lógica própria.

## Cobertura numérica

- Medida e reportada em cada execução de suíte.
- **Sem percentual mínimo obrigatório** (metas suaves — RNF-025).
- A garantia real vem das regras por camada acima + "todo CA vira teste", não de um número.

Comandos:

```bash
# backend
cargo llvm-cov

# frontend
npm run test -- --coverage
```

---

# Integração com o Fluxo de Features (docs-ai)

Esta é a parte que garante testes **em toda nova feature**.

Ao implementar uma feature pelo fluxo `docs-ai`:

1. **`1-pesquisa.md`** — listar os CAs dos UCs envolvidos.
2. **`2-planejamento.md`** — o "Plano de testes" mapeia **cada CA → teste(s)**, indicando nível e arquivo previsto.
3. **`3-execucao.md`** — o Definition of Done só fecha quando:
   - todo CA tem ≥ 1 teste correspondente;
   - testes unitários seguem estas convenções (local, nome, mocks);
   - a suíte passa offline e a cobertura foi reportada.

Nenhuma feature é considerada concluída sem seus testes e sem o mapeamento CA→teste documentado.

---

# Testes de Regressão (bugs)

Todo bug corrigido deve virar um teste que o impeça de voltar.

Fluxo obrigatório ao corrigir um bug (encontrado durante o trabalho ou apontado pelo usuário):

1. Corrigir o bug.
2. **Perguntar ao usuário, com recomendação**, se deve gerar um teste de regressão.
   - A recomendação avalia se o bug é testável de forma determinística e se o teste agrega valor (ex.: lógica/contrato/ordenação) ou não (ex.: ajuste visual trivial, config).
3. Se confirmado, escrever o teste que **falha sem a correção e passa com ela**, seguindo as convenções deste documento.
4. O teste referencia o bug no nome/descrição (ex.: `nao_emite_ready_antes_do_listener`).

Não fechar a correção sem passar por este fluxo.

---

# Critérios de Aceitação

- CA-001: testes unitários de backend ficam inline (`#[cfg(test)]`); integração em `tests/`.
- CA-002: testes de frontend ficam colocados ao lado do arquivo, com sufixo `.test`.
- CA-003: o Serviço de IA é sempre mockado em testes unitários.
- CA-004: cada CA de uma feature possui ao menos um teste.
- CA-005: cobertura é reportada (sem mínimo obrigatório).
- CA-006: toda feature documenta o mapeamento CA→teste no seu `2-planejamento.md`.
- CA-007: toda camada com lógica não-trivial atende às expectativas da seção "Cobertura por Camada" — repositórios, stores, serviços puros e componentes não ficam sem teste só porque um CA não os exercita diretamente.

---

# Observação

`estrategia-de-testes.md` permanece a referência de *estratégia*; este documento é a referência de *convenções de arquivo e fluxo*. Ferramentas em ADR-014.
