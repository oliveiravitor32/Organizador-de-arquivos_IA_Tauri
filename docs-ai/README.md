# docs-ai

Camada **operacional para agentes de IA**, complementar ao `docs/` (a especificação SDD).

O `docs/` responde *o quê* e *por quê*. O `docs-ai/` responde *como trabalhar* e mantém o *rastro de cada feature*.

> O `docs/` é a fonte da verdade. Este diretório **aponta para ele** e organiza o trabalho — nunca recopia conteúdo, para não divergir.

---

## Como usar

Ao implementar uma feature:

1. Crie uma pasta em `docs-ai/features/<nome-da-feature>/`.
2. Copie `TEMPLATE.md` em três arquivos e preencha **em ordem**:
   - `1-pesquisa.md` — o que o `docs/` diz sobre esta feature.
   - `2-planejamento.md` — plano de implementação derivado da pesquisa.
   - `3-execucao.md` — tarefas, ordem e definition of done, derivados do plano.
3. Só comece a codar quando os três estiverem preenchidos.

Cada etapa **deriva da anterior**. Pesquisa mal feita produz plano fraco.

---

## Estrutura

```text
docs-ai/
├── README.md
├── TEMPLATE.md              # molde dos 3 passos
└── features/
    └── <nome-da-feature>/
        ├── 1-pesquisa.md
        ├── 2-planejamento.md
        └── 3-execucao.md
```

---

## Princípios

- **Sem duplicação** — referencie `docs/...`, não copie.
- **Efêmero por feature** — cada pasta é um rastro daquela entrega.
- **Encadeado** — pesquisa → planejamento → execução, sempre nessa ordem.
- **Regras inegociáveis** — estão no `CLAUDE.md` da raiz; valem sempre.

---

## Inspiração

O fluxo **pesquisa → planejamento → execução** é inspirado no padrão *research → plan → execute* para Spec Driven Development apresentado pelo Google Developers:

- 🎥 [Short — research, plan, execute (Google Developers)](https://youtube.com/shorts/zn4mPqPPDFQ)
