# 🗂️ Organizador de Arquivos com IA

> Um aplicativo desktop **local-first** que entende o **conteúdo** dos seus arquivos — não só o nome ou a extensão — e sugere como organizá-los de forma explicável e reversível.

A organização das pastas deixa de ser uma decisão manual e passa a **emergir do significado** dos documentos: projetos, clientes, pessoas e temas são descobertos por IA local e conectados em um **grafo de conhecimento**. Você revisa, aprova e — se quiser — desfaz. Nada sai da sua máquina.

---

## 💡 A ideia em 30 segundos

```text
Seus arquivos  →  Indexação  →  IA local  →  Grafo de Conhecimento
                                                     ↓
                              Sugestões explicáveis (você aprova)
                                                     ↓
                          Organização aplicada — e reversível
```

- **Conteúdo acima da estrutura.** O que importa é o significado, não onde o arquivo está.
- **Você no controle.** Nenhuma alteração acontece sem sua aprovação.
- **Reversível.** Toda mudança tem snapshot e pode ser desfeita.
- **Privado.** Processamento local; nenhum serviço externo é obrigatório.

---

## 🧱 Stack

| Camada | Tecnologia |
| --- | --- |
| Interface | React + Vite |
| Desktop | Tauri v2 |
| Backend | Rust |
| Persistência | SQLite (`sqlx`) |
| IA | Ollama + Qwen 3 4B (local, trocável) |

---

## 🏛️ Arquitetura com Spec Driven Development (SDD)

Este projeto é construído com **Spec Driven Development**: a **especificação vem antes do código** e é a fonte da verdade. Toda funcionalidade nasce de um documento — caso de uso, decisão de arquitetura e contrato — e só então é implementada.

Isso significa que, para entender o projeto, **você lê a spec, não engenharia reversa do código.**

### Onde está tudo — `docs/`

```text
docs/
├── README.md          # índice navegável de toda a documentação
├── visao.md           # propósito e objetivos
├── glossario.md       # termos do projeto
├── requisitos/        # o que o sistema faz (+ estratégia de testes)
├── dominios/          # conceitos centrais (grafo, IA, indexação, UI…)
├── casos-de-uso/      # 15 UCs descrevendo cada fluxo (UC-001 a UC-015)
├── arquitetura/       # visão, contratos, eventos, esquema SQL, estrutura
├── decisoes/          # 11 ADRs (decisões de arquitetura registradas)
└── roadmap.md         # ordem de construção por marcos
```

👉 **Comece por [`docs/README.md`](docs/README.md)** — ele mapeia todo o resto.

### Por que SDD aqui?

- **Onboarding rápido** — um colaborador entende o *porquê* de cada parte sem decifrar código.
- **Decisões rastreáveis** — cada escolha tem um ADR explicando contexto e alternativas.
- **Sem ambiguidade** — comportamento, contratos e esquema têm "fontes únicas da verdade" declaradas.

---

## 🤖 O diferencial: SDD + Claude (agentes de IA)

A spec não serve só para humanos — ela foi desenhada para que **agentes de IA (como o Claude Code) implementem funcionalidades sem se perder no contexto**.

Para isso existe uma camada operacional dedicada, o **[`docs-ai/`](docs-ai/)**, que conecta a IA diretamente ao SDD:

```text
CLAUDE.md                      # sempre carregado: regras inegociáveis + mapa do projeto
docs-ai/
├── TEMPLATE.md                # molde do fluxo de trabalho por feature
└── features/<feature>/
    ├── 1-pesquisa.md          # o que a spec (docs/) diz sobre a feature
    ├── 2-planejamento.md      # plano derivado da pesquisa
    └── 3-execucao.md          # tarefas + definition of done
```

### Como funciona a ligação com o SDD

1. O [`CLAUDE.md`](CLAUDE.md) é lido automaticamente em toda sessão e fixa as **regras inegociáveis** (local-first, aprovação obrigatória, reversibilidade, grafo como fonte da verdade) e aponta para o `docs/`.
2. Ao implementar algo, o agente cria uma pasta em `docs-ai/features/` e segue o fluxo **pesquisa → planejamento → execução**, sempre derivando do `docs/`.
3. Cada feature deixa um **rastro auditável** do que foi pesquisado, planejado e feito.

### Benefícios

| Benefício | Como o SDD + Claude entrega |
| --- | --- |
| 🎯 **Foco de contexto** | O agente lê só a spec da feature, não o repo inteiro. |
| 🔒 **Consistência garantida** | Regras inegociáveis e ADRs impedem decisões fora da arquitetura. |
| 🔁 **Repetível** | Implementar uma feature nova é copiar o template e seguir o fluxo. |
| 🧭 **Sem drift** | `docs-ai/` aponta para o `docs/`, nunca duplica — a verdade fica num lugar só. |
| 📜 **Auditável** | Cada decisão e cada feature ficam registradas e rastreáveis. |

> Na prática: o `docs/` diz **o quê** e **por quê**; o `docs-ai/` diz **como trabalhar**; o Claude executa seguindo ambos.

---

## 🚀 Status do projeto

📋 **Especificação concluída (SDD v1)** — pronta para implementação.
🔨 **Marco 0 (Fundação)** — planejado, aguardando início.

A ordem de construção está em [`docs/roadmap.md`](docs/roadmap.md):

```text
M0 Fundação → M1 Descoberta → M2 Conhecimento → M3 Inteligência
→ M4 Execução Segura → M5 Exploração → M6 Robustez
```

---

## 🤝 Como contribuir

1. Leia [`docs/README.md`](docs/README.md) para entender o projeto.
2. Confira o [`docs/roadmap.md`](docs/roadmap.md) para ver o que vem a seguir.
3. Antes de implementar, siga o fluxo de [`docs-ai/`](docs-ai/): crie `docs-ai/features/<sua-feature>/` a partir do `TEMPLATE.md`.
4. Respeite as **regras inegociáveis** do [`CLAUDE.md`](CLAUDE.md) e os [ADRs](docs/decisoes/).
5. Toda mudança de comportamento começa atualizando a spec — *a spec vem primeiro*.

---

## 📄 Licença

A definir.
