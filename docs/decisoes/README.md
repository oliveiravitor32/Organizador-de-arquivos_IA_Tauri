# Decisões de Arquitetura (ADRs)

Este diretório contém os **Architecture Decision Records** do projeto.

Cada ADR registra uma decisão arquitetural relevante, seu contexto, a decisão tomada e suas consequências.

---

# Convenção de Nomes

Todo arquivo de ADR deve seguir o padrão:

```text
ADR-NNN-titulo-em-kebab-case.md
```

Regras:

- Prefixo `ADR-` sempre em **maiúsculas**.
- `NNN` é um número sequencial com três dígitos (`001`, `002`, ...).
- O título é descritivo, em minúsculas, separado por hífens.
- Um ADR por arquivo.

Exemplos válidos:

```text
ADR-001-react.md
ADR-005-sqlite-persistencia.md
ADR-010-estrategia-de-snapshot-e-rollback.md
```

---

# Numeração

- Os números são atribuídos de forma sequencial e **não são reutilizados**.
- Um número já usado permanece associado à sua decisão mesmo que ela seja superada.
- ADRs nunca são apagados; quando superados, têm o status atualizado.

---

# Status Possíveis

- **Proposto** — em discussão.
- **Aceito** — decisão vigente.
- **Superado** — substituído por outra decisão (referenciar o ADR que o substitui).
- **Rejeitado** — avaliado e descartado.

---

# Índice de Decisões

| ADR | Título | Status |
| --- | --- | --- |
| ADR-001 | React como Framework Frontend | Aceito |
| ADR-002 | Tauri como Plataforma Desktop | Aceito |
| ADR-003 | IA Local como Estratégia Principal | Aceito |
| ADR-004 | Grafo de Conhecimento como Fonte da Verdade | Aceito |
| ADR-005 | SQLite como Persistência Principal | Aceito |
| ADR-006 | Indexação Independente da IA | Aceito |
| ADR-007 | Aprovação Obrigatória para Alterações | Aceito |
| ADR-008 | Qwen 3 4B como modelo inicial | Aceito |
| ADR-009 | Motor de Sugestões orientado a conhecimento | Aceito |
| ADR-010 | Estratégia de snapshot e rollback | Aceito |
| ADR-011 | Tauri v2, sqlx e Vite como base técnica | Aceito |
| ADR-012 | shadcn/ui + Tailwind como base de UI | Aceito |
| ADR-013 | Configuração em arquivo e observabilidade local | Aceito |

---

# Observação

Esboços e diagramas iniciais podem utilizar nomes informais para as decisões.

O nome **oficial e vigente** de cada ADR é sempre o do arquivo correspondente neste diretório.
