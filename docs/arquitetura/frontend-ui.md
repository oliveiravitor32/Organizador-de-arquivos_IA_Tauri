# Arquitetura: Fundações do Frontend (UI)

## Objetivo

Este documento define as **fundações técnicas** da interface: organização de componentes, design tokens, temas e padrões de estado.

Ele é o equivalente, para o frontend, do que `estrutura-do-projeto.md` e os ADRs de ferramenta fazem para o backend.

- **O quê** a UI mostra → `dominios/interface-usuario.md`.
- **Com o quê** ela é construída → ADR-012 (shadcn/ui + Tailwind).
- **Como** organizá-la e estilizá-la → este documento.

---

# Princípios

## Minimalista e Moderno

Menos é mais: hierarquia clara, espaço em branco generoso, poucos elementos por tela.

---

## Conforto Visual

Temas pensados para uso prolongado. Ambos baseados na paleta `neutral` do Tailwind: escuro em neutro escuro (preto neutro), claro em neutro claro (branco neutro).

---

## Desacoplamento Total de Tema

Esta é a **regra mais importante da UI** e vale para todas as features.

A UI consome **apenas tokens semânticos**. Nenhum componente usa:

- cor crua (`#000`, `rgb(...)`)
- classe de cor direta do Tailwind (`bg-neutral-900`, `text-white`, `border-black`)

Sempre usar os tokens (`bg-background`, `text-foreground`, `border-border`, …).

**Por quê:** adicionar um novo tema no futuro (ex.: um terceiro tema, ou cores de marca) deve exigir **apenas um novo conjunto de valores de tokens**, sem alterar uma única tela ou componente.

---

## Consistência por Tokens

Cores, espaçamento e tipografia vêm de tokens, nunca de valores soltos.

---

## Apresentação Pura

A UI não contém regra de negócio; consome apenas commands e events (ver `tauri.md`).

---

# Estado (ADR-015)

Divisão clara de responsabilidades:

- **TanStack Query** — estado assíncrono vindo dos commands Tauri (loading, erro, cache, refetch). Eventos do backend invalidam/atualizam queries.
- **Zustand** — estado puramente de UI (preferências, seleção, tema, navegação).

Regra: dados do backend nunca são duplicados manualmente no Zustand — pertencem ao TanStack Query.

---

# Internacionalização (i18n-ready)

O MVP é **pt-BR**, mas o código é preparado para tradução futura:

- Nenhuma string de UI fica hardcoded em componente.
- Textos ficam centralizados (catálogo de mensagens), prontos para um mecanismo de i18n.
- Adicionar um idioma no futuro não deve exigir refatorar componentes.

---

# Organização de Componentes

Conforme `estrutura-do-projeto.md`:

```text
src/
├── components/
│   ├── ui/         # componentes shadcn/ui (copiados, base)
│   └── shared/     # componentes compostos do projeto
├── pages/          # telas (ver interface-usuario.md)
├── features/       # lógica por funcionalidade
├── ipc/            # commands e events
├── hooks/
├── stores/         # estado de UI
└── styles/         # tokens e tema (globals.css)
```

## Regras

- `components/ui/` é a base shadcn/ui; ajustes globais de estilo ocorrem aqui.
- `components/shared/` compõe a base em peças reutilizáveis do produto.
- Telas vivem em `pages/`; lógica de dados em `features/`.

---

# Design Tokens

Tokens são variáveis CSS semânticas, expostas ao Tailwind como cores (`bg-background`, `text-foreground`, etc.). **Os componentes só usam estes tokens** — ver a regra de Desacoplamento Total de Tema.

Ambos os temas derivam da paleta **`neutral` do Tailwind**. Os mapeamentos abaixo são a **diretriz**; o ajuste fino de cada degrau ocorre na implementação.

## Tema Claro (neutral claro)

Branco neutro, evitando branco puro.

```text
--background        neutral-50    (fundo)
--foreground        neutral-950   (texto)
--muted             neutral-100
--muted-foreground  neutral-500
--border            neutral-200
--primary           neutral-900   (ações)
--primary-foreground neutral-50
--ring              neutral-400   (foco)
```

## Tema Escuro (neutral escuro)

Preto neutro, confortável para uso prolongado.

```text
--background        neutral-950   (fundo)
--foreground        neutral-50    (texto)
--muted             neutral-900
--muted-foreground  neutral-400
--border            neutral-800
--primary           neutral-50    (ações)
--primary-foreground neutral-900
--ring              neutral-600   (foco)
```

> Os degraus `neutral-*` são o ponto de partida; o que é fixo é a **semântica do token** e o uso da família `neutral`. Trocar de tema = trocar os valores, nunca os componentes.

## Escala

- **Espaçamento:** escala do Tailwind (múltiplos de 4px).
- **Tipografia:** uma família sem serifa; pesos regular/medium/semibold.
- **Raio:** cantos suaves e consistentes (token `--radius`).

---

# Tema Claro/Escuro

## Estratégia

- Tailwind em modo `dark` por classe na raiz (`<html class="dark">`).
- Tokens trocam de valor conforme o tema; componentes não conhecem o tema.

## Comportamento

- Alternador acessível ao usuário.
- Preferência persistida localmente.
- Respeitar a preferência do sistema na primeira execução.

---

# Padrões de Estado de UI

Coerentes com `interface-usuario.md`. Toda tela que carrega dados trata os quatro estados:

## Carregando

Indicação clara de operação em andamento (alimentada por events de progresso).

---

## Conteúdo

Estado normal com dados.

---

## Vazio

Ausência de dados, com orientação para o próximo passo.

---

## Erro

Erro estruturado recebido do backend, exibido de forma compreensível, com ação sugerida.

---

# Feedback e Acessibilidade

- Toda ação destrutiva exige confirmação.
- Todo processo longo exibe progresso.
- Componentes acessíveis por padrão (base Radix do shadcn/ui).
- Contraste adequado em ambos os temas.

---

# Relação com o Marco 0

No Marco 0, esta base é estabelecida no mínimo viável:

- Tailwind + shadcn/ui instalados.
- `styles/globals.css` com os tokens dos dois temas.
- Alternador de tema funcional.
- Tela inicial vazia já respeitando tokens e estados.

O design detalhado de cada tela é feito por feature (via `docs-ai`), sobre esta fundação.

---

# Observação

Este documento define as fundações, não o design final de cada tela.

`dominios/interface-usuario.md` permanece a referência das telas e fluxos; este documento define como construí-las de forma consistente.
