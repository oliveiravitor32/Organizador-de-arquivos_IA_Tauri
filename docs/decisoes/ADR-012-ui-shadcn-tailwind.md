# ADR-012 — shadcn/ui + Tailwind como Base de UI

## Status

Aceito

---

## Data

2026-06-13

---

## Contexto

O frontend React (ADR-001) será iniciado no Marco 0, que estabelece o esqueleto da interface e o primeiro componente.

A abordagem de estilização e a origem dos componentes de UI definidas agora se propagam para todos os marcos seguintes. Definir isso antes evita dívida técnica.

O domínio `interface-usuario.md` já especifica telas, estados e a exigência de feedback (loading, erro, vazio). Falta a decisão técnica de **como** construí-los visualmente.

---

## Problema

Qual abordagem de estilização e biblioteca de componentes adotar para um produto desktop com design minimalista e moderno, suporte a tema claro/escuro e boa velocidade de desenvolvimento?

---

## Decisão

Adotar:

- **Tailwind CSS** para estilização utilitária.
- **shadcn/ui** como base de componentes.

shadcn/ui não é uma dependência de runtime fechada: os componentes são copiados para o projeto e ficam sob controle do código, estilizados com Tailwind.

---

## Justificativa

### Tailwind

- Estilização consistente via tokens utilitários.
- Tema claro/escuro nativo via classe `dark`.
- Sem CSS órfão; baixa superfície de drift visual.

---

### shadcn/ui

- Componentes acessíveis (baseados em Radix) e headless.
- Código copiado para o projeto: total controle, sem lock-in.
- Estética minimalista por padrão, alinhada à diretriz do produto.
- Integração direta com Tailwind e com design tokens.

---

## Diretriz de Design

- **Minimalista e moderno.**
- **Ambos os temas usam a paleta `neutral` do Tailwind.**
  - **Tema escuro:** `neutral` escuro (preto neutro), confortável para uso prolongado.
  - **Tema claro:** `neutral` claro (branco neutro), evitando branco puro agressivo.
- Alternância clara/escuro disponível ao usuário.
- Detalhes técnicos de tokens e temas em `frontend-ui.md`.

## Desacoplamento de Tema (regra rígida)

Toda a UI consome **apenas tokens semânticos** (`--background`, `--foreground`, etc.).

Nenhum componente ou tela usa cor crua (hex, `rgb`) nem classe de cor direta do Tailwind (`bg-neutral-900`, `text-black`).

Essa regra vale para **todas as features**: adicionar um novo tema no futuro deve exigir apenas um novo conjunto de valores de tokens, sem tocar em nenhum componente.

---

## Alternativas Consideradas

### CSS puro / CSS Modules

#### Motivo da Rejeição

Maior esforço para consistência e temas; reinventa componentes acessíveis.

---

### Biblioteca de componentes fechada (ex.: MUI)

#### Motivo da Rejeição

Estética opinativa difícil de minimalizar; peso de runtime; menos controle sobre o código.

---

## Consequências

### Positivas

- Desenvolvimento rápido com componentes prontos e acessíveis.
- Tema claro/escuro consistente desde o início.
- Controle total do código dos componentes.

---

### Negativas

- Componentes copiados precisam ser mantidos pelo próprio projeto.
- Tailwind exige disciplina para evitar classes utilitárias excessivas (mitigado por componentes).

---

## Impacto

- **Marco 0** — instala Tailwind + shadcn/ui no esqueleto do frontend; aplica os design tokens base e o alternador de tema.
- **frontend-ui.md** — detalha tokens, temas, organização de componentes e padrões de estado.
- **Todos os marcos de UI** — herdam esta base.

---

## Decisão Final

A base de UI do projeto é **shadcn/ui + Tailwind**, com design minimalista e suporte a tema claro/escuro, formalizando como o frontend (ADR-001) será construído.
