# Roadmap de Implementação

## Objetivo

Este documento define a **ordem de construção** do sistema, organizando os casos de uso, decisões e artefatos de arquitetura em marcos entregáveis.

Ele não altera a especificação: apenas sequencia sua implementação.

---

# Princípios de Sequenciamento

## Fundação Antes de Funcionalidade

Infraestrutura (Tauri, banco, fronteira) precede qualquer caso de uso.

---

## Valor Incremental

Cada marco entrega algo verificável de ponta a ponta.

---

## Reversibilidade Cedo

A segurança (snapshot e rollback) é construída junto com a execução, nunca depois.

---

## IA Desacoplada

A indexação é entregue antes e independentemente da IA (ver ADR-006).

---

# Marco 0 — Fundação

## Objetivo

Estabelecer o esqueleto técnico.

## Escopo

- projeto Tauri + React (ADR-001, ADR-002)
- estrutura de pastas (`estrutura-do-projeto.md`)
- banco SQLite e migrações (`esquema-sql.md`, ADR-005)
- fronteira de commands/events vazia (`contratos-tauri.md`)
- suíte de testes base (`estrategia-de-testes.md`)

## Entregável

Aplicação que abre, conecta ao banco e responde a um command de teste.

---

# Marco 1 — Descoberta

## Objetivo

Transformar um diretório em arquivos indexados.

## Escopo

- UC-001 Escanear Diretório
- UC-002 Indexar Arquivos
- domínio `sistema-arquivos.md` e `indexacao.md`
- tela de Indexação (parte)

## Entregável

O usuário seleciona um diretório e vê seus arquivos indexados com metadados e conteúdo extraído.

---

# Marco 2 — Conhecimento

## Objetivo

Transformar arquivos em conhecimento estruturado.

## Escopo

- abstração do Serviço de IA + adaptador Ollama (ADR-003, ADR-008)
- pipeline de IA (`pipeline-ia.md`)
- UC-008 Extrair Entidades
- UC-009 Gerar Embeddings
- UC-010 Descobrir Relações
- UC-011 Construir Clusters
- UC-003 Analisar Arquivos (orquestração)
- UC-004 Construir Grafo (ADR-004)

## Entregável

Arquivos analisados geram entidades, embeddings, relações e clusters incorporados ao grafo.

---

# Marco 3 — Inteligência

## Objetivo

Transformar conhecimento em recomendações explicáveis.

## Escopo

- UC-005 Gerar Sugestões (ADR-009)
- UC-012 Explicar Sugestões
- tela de Sugestões e Detalhe da Sugestão

## Entregável

O sistema apresenta sugestões com justificativa, evidências e confiança.

---

# Marco 4 — Execução Segura

## Objetivo

Aplicar alterações de forma reversível.

## Escopo

- UC-013 Revisar Sugestões (ADR-007)
- UC-006 Aplicar Alterações
- UC-007 Desfazer Alterações
- snapshot e rollback (ADR-010)
- telas de Execução e Histórico

## Entregável

O usuário aprova sugestões, aplica alterações e consegue reverter qualquer execução.

---

# Marco 5 — Exploração

## Objetivo

Tornar o conhecimento navegável e pesquisável.

## Escopo

- UC-014 Busca Semântica
- UC-015 Explorar Contexto
- telas de Exploração

## Entregável

O usuário busca arquivos por significado e navega pelas conexões do grafo.

---

# Marco 6 — Robustez e Acabamento

## Objetivo

Preparar para uso real.

## Escopo

- cobertura completa dos Critérios de Aceitação
- requisitos não funcionais (desempenho, resiliência)
- configuração (modelo, itens ignorados, OCR)
- tratamento de erros e estados vazios na UI

## Entregável

Aplicação estável, testada e configurável.

---

# Dependências entre Marcos

```text
M0 Fundação
↓
M1 Descoberta
↓
M2 Conhecimento ── depende de M1
↓
M3 Inteligência ── depende do grafo (M2)
↓
M4 Execução ────── depende de sugestões (M3)
↓
M5 Exploração ──── depende do grafo (M2), entregue após M4
↓
M6 Robustez ────── transversal, consolidado ao final
```

A Exploração (M5) depende tecnicamente apenas do grafo (M2), mas é sequenciada após a Execução por prioridade de valor.

---

# Rastreabilidade

| Marco | Casos de Uso | Decisões | Artefatos |
| --- | --- | --- | --- |
| M0 | — | ADR-001, ADR-002, ADR-005 | estrutura-do-projeto, esquema-sql, contratos-tauri |
| M1 | UC-001, UC-002 | ADR-006 | sistema-arquivos, indexacao |
| M2 | UC-003, UC-004, UC-008–011 | ADR-003, ADR-004, ADR-008 | pipeline-ia, analise-ia, grafo-conhecimento |
| M3 | UC-005, UC-012 | ADR-009 | sugestoes |
| M4 | UC-006, UC-007, UC-013 | ADR-007, ADR-010 | catalogo-de-eventos |
| M5 | UC-014, UC-015 | — | grafo-conhecimento |
| M6 | todos (CA) | — | estrategia-de-testes, nao-funcionais |

---

# Decisões Pendentes por Marco (Gate de Marco)

Decisões técnicas deliberadamente adiadas para o momento de cada marco. Antes de implementar o marco, o **Gate de Marco** (ver `CLAUDE.md`) exige decidi-las com o usuário e registrá-las como ADR.

| Marco | Decisões a definir antes de codar |
| --- | --- |
| M0 | ✅ nenhuma — base já decidida (ADR-011 a ADR-015) |
| M1 | ✅ bibliotecas de extração: `pdf-extract` + `docx-rs` + `calamine` (ADR-016); OCR adiado para M6 (ADR-017) |
| M2 | estratégia de **busca vetorial / similaridade** (cosseno em memória, `sqlite-vss`, ou lib); parâmetros de clusterização |
| M3 | heurísticas/limiares do motor de sugestões |
| M4 | (sem pendência conhecida — snapshot/rollback já em ADR-010) |
| M5 | reaproveita a decisão de busca vetorial do M2; roteamento de UI se ainda não definido |
| M6 | estratégia de release/empacotamento (instaladores Windows/Linux); auto-update |

Esta tabela é viva: novas pendências descobertas durante a pesquisa de uma feature entram aqui.

---

# Observação

A ordem reflete dependências reais e priorização de valor.

Marcos podem se sobrepor parcialmente, mas nenhum caso de uso deve ser iniciado antes de suas dependências estarem disponíveis.
