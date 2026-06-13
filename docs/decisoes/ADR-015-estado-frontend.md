# ADR-015 — Estado do Frontend: Zustand + TanStack Query

## Status

Aceito

---

## Data

2026-06-13

---

## Contexto

O Marco 0 estabelece o esqueleto do frontend React e os wrappers de `ipc/`. A forma de gerenciar estado de UI e estado assíncrono (commands Tauri) se propaga para todas as telas, então precisa ser decidida antes.

---

## Problema

Como gerenciar:

- **estado de UI** (preferências, seleção, navegação)
- **estado assíncrono** dos commands Tauri (loading, erro, cache, refetch)

de forma simples e escalável para um app desktop local-first?

---

## Decisão

- **Zustand** para estado de UI (cliente).
- **TanStack Query** para estado assíncrono vindo dos commands Tauri.

A separação é clara: TanStack Query cuida do que vem do backend; Zustand cuida do estado puramente de interface.

---

## Justificativa

### Zustand

- Mínimo boilerplate; stores simples.
- Sem provider boilerplate nem verbosidade do Redux.
- Adequado ao volume de estado de um app desktop.

---

### TanStack Query

- Trata loading/erro/cache/refetch de forma declarativa.
- Encaixa nos wrappers de `ipc/commands.ts`.
- Invalidação por eventos: ao receber um event de progresso/conclusão, a query relacionada é invalidada/atualizada.
- Reduz drasticamente código manual de `useEffect`.

---

## Alternativas Consideradas

### Redux Toolkit

Rejeitado: verboso demais para o tamanho do projeto.

---

### Apenas Context + hooks

Rejeitado: não escala bem; reimplementaria cache/async manualmente.

---

### SWR

Rejeitado: bom, mas com menos recursos de mutation/invalidation que o TanStack Query.

---

## Consequências

### Positivas

- Estado assíncrono robusto com pouco código.
- Limite claro entre estado de servidor e de UI.

---

### Negativas

- Duas bibliotecas a aprender (curva pequena).
- É preciso disciplina para não duplicar no Zustand o que pertence ao TanStack Query.

---

## Impacto

- **frontend-ui.md** — detalha a divisão de responsabilidades de estado.
- **estrutura-do-projeto.md** — `stores/` usa Zustand; `ipc/` + hooks usam TanStack Query.
- **Marco 0** — instala e configura ambos.

---

## Decisão Final

Estado do frontend: **Zustand** (UI) + **TanStack Query** (assíncrono/IPC).
