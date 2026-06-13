# Contribuindo

Obrigado pelo interesse em contribuir! Este projeto segue **Spec Driven Development**: a especificação vem antes do código.

---

## Antes de começar

1. Leia [`docs/README.md`](docs/README.md) e o [`CLAUDE.md`](CLAUDE.md) (regras inegociáveis).
2. Prepare o ambiente: [`docs/desenvolvimento/ambiente-de-desenvolvimento.md`](docs/desenvolvimento/ambiente-de-desenvolvimento.md).
3. Veja o que vem a seguir em [`docs/roadmap.md`](docs/roadmap.md).

---

## Fluxo de uma contribuição

A spec vem primeiro. Toda mudança de comportamento começa atualizando o `docs/`.

Ao implementar uma feature, siga o processo de [`docs-ai/`](docs-ai/):

1. Crie `docs-ai/features/<sua-feature>/` a partir do `TEMPLATE.md`.
2. Preencha `1-pesquisa.md` → `2-planejamento.md` → `3-execucao.md`.
3. Implemente seguindo o plano.

---

## Regras de pronto

- Cada Critério de Aceitação (CA) tem ≥ 1 teste (ADR-014, `docs/requisitos/convencoes-de-teste.md`).
- Testes unitários seguem as convenções de arquivo/nomenclatura.
- Nenhuma violação das regras inegociáveis do `CLAUDE.md`.
- A suíte passa offline.

---

## Convenções de Git

### Branches

- `main` — estável.
- `feat/<nome>`, `fix/<nome>`, `docs/<nome>` — trabalho.

### Commits

Mensagens no formato:

```text
<tipo>: <descrição curta no imperativo>

<corpo opcional>
```

Tipos: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`.

### Pull Requests

- Descreva o que muda e referencie o UC/ADR relacionado.
- A pipeline de CI deve passar (ver `docs/desenvolvimento/integracao-continua.md`).

---

## Idioma

Documentação e interface em **pt-BR**. O código é estruturado para internacionalização futura (i18n-ready), mas o MVP é pt-BR.

---

## Decisões de arquitetura

Não tome decisões fora dos ADRs. Se algo não está especificado, abra a discussão e registre um novo ADR antes de implementar (ver `docs/decisoes/README.md`).
