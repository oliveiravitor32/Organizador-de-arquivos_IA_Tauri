# Organizador de Arquivos com IA

Aplicação **desktop local-first** (Tauri + React + Rust) que transforma arquivos em conhecimento e sugere organização baseada em **significado**, não em nome/extensão/localização. A IA descobre conhecimento; o usuário decide; o sistema organiza de forma reversível.

---

## Regras inegociáveis

Nunca viole estas regras. Elas vêm dos ADRs e definem a identidade do projeto.

1. **Local-first** — nada sai da máquina; nenhum serviço externo é obrigatório. (ADR-003)
2. **Aprovação obrigatória** — nenhuma alteração física ocorre sem confirmação explícita do usuário. (ADR-007)
3. **Reversibilidade** — toda alteração exige snapshot prévio e pode ser desfeita. (ADR-010)
4. **Grafo é a fonte da verdade** — a estrutura de pastas é projeção do conhecimento, nunca o contrário. (ADR-004)
5. **A IA não move arquivos** — ela produz conhecimento; a organização é derivada dele. (ADR-009)
6. **Indexação independe da IA** — funciona mesmo sem modelo disponível. (ADR-006)

---

## Stack

- Frontend: React (ADR-001) · Desktop: Tauri (ADR-002) · Backend: Rust
- Persistência: SQLite (ADR-005) · IA: Ollama + Qwen 3 4B (ADR-008), atrás de uma abstração trocável

---

## Como trabalhar neste projeto

A especificação completa está em `docs/` (Spec Driven Development) — **é a fonte da verdade**.

**Ao implementar qualquer feature, siga o processo de `docs-ai/`:**

1. Crie `docs-ai/features/<nome-da-feature>/` copiando `docs-ai/TEMPLATE.md`.
2. Preencha em ordem: `1-pesquisa.md` → `2-planejamento.md` → `3-execucao.md`.
3. Cada etapa deriva da anterior. Só comece a codar após a execução estar planejada.

Não invente decisões fora dos ADRs. Se algo não está especificado, pergunte ou registre uma decisão antes.

## Gate de Marco (OBRIGATÓRIO antes de implementar qualquer marco)

Antes de escrever qualquer código de um marco, execute este gate e **pare para falar com o usuário**:

1. Releia o marco no `docs/roadmap.md` (escopo, UCs, decisões pendentes listadas).
2. Identifique toda **decisão/ferramenta ainda não documentada** que o marco exige (ex.: bibliotecas de extração no M1, busca vetorial no M2, motor de OCR no M1).
3. **Solicite essas decisões ao usuário**, apresentando recomendações — e registre cada uma como **ADR** antes de codar.
4. Só então crie/finalize `docs-ai/features/<marco>/` e implemente.

**Nunca inicie a implementação de um marco com decisões pendentes.** Documentar primeiro, codar depois.

---

## Mapa de navegação do `docs/`

| Preciso entender… | Leia |
| --- | --- |
| Visão e termos | `docs/visao.md`, `docs/glossario.md` |
| O que o sistema faz | `docs/requisitos/`, `docs/casos-de-uso/` |
| Conceitos centrais | `docs/dominios/` |
| Arquitetura | `docs/arquitetura/visao-geral.md`, `docs/arquitetura/estrutura-do-projeto.md` |
| Decisões | `docs/decisoes/` (índice em `docs/decisoes/README.md`) |
| Ordem de construção | `docs/roadmap.md` |
| Índice geral | `docs/README.md` |

---

## Fontes únicas da verdade

Em caso de divergência, estes documentos têm autoridade final:

| Tema | Documento |
| --- | --- |
| Comportamento esperado | Critérios de Aceitação dos casos de uso |
| Commands (frontend ↔ backend) | `docs/arquitetura/contratos-tauri.md` |
| Nomes de eventos | `docs/arquitetura/catalogo-de-eventos.md` |
| Esquema físico do banco | `docs/arquitetura/esquema-sql.md` |
| Organização do código | `docs/arquitetura/estrutura-do-projeto.md` |

---

## Convenções

- ADRs: `ADR-NNN-titulo-kebab.md`, prefixo maiúsculo, nunca apagados. (ver `docs/decisoes/README.md`)
- Erros entre backend e frontend são **estruturados** (`code`, `message`, `details`).
- Operações longas retornam início imediato e comunicam progresso por **events**.
- Apenas repositórios acessam o SQLite; serviços consomem repositórios.
- **UI 100% desacoplada de tema:** componentes usam só tokens semânticos (`bg-background`, `text-foreground`…), nunca cor crua ou classe de cor direta do Tailwind (`bg-neutral-900`). Vale para toda feature. (ADR-012, `docs/arquitetura/frontend-ui.md`)
- **Todo CA vira teste:** nenhuma feature fecha sem teste para cada Critério de Aceitação; testes unitários seguem `docs/requisitos/convencoes-de-teste.md` (Vitest+RTL no front, `#[cfg(test)]`+mockall no Rust). (ADR-014)
