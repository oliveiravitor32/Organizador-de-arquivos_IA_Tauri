# Documentação — Organizador de Arquivos com IA

Este diretório contém a especificação completa do projeto, organizada segundo o **Spec Driven Development**: a documentação é a fonte da verdade e precede a implementação.

---

# Como Ler

A leitura recomendada segue do "porquê" ao "como":

```text
Visão → Requisitos → Domínios → Casos de Uso → Arquitetura → Decisões → Planejamento
```

---

# Visão Geral

- [visao.md](visao.md) — propósito e objetivos do produto.
- [glossario.md](glossario.md) — termos e conceitos do projeto.

---

# Requisitos

- [requisitos/funcionais.md](requisitos/funcionais.md) — o que o sistema deve fazer.
- [requisitos/nao-funcionais.md](requisitos/nao-funcionais.md) — desempenho, privacidade, resiliência.
- [requisitos/estrategia-de-testes.md](requisitos/estrategia-de-testes.md) — como a qualidade é verificada.

---

# Domínios

- [dominios/sistema-arquivos.md](dominios/sistema-arquivos.md) — origem e destino das alterações.
- [dominios/indexacao.md](dominios/indexacao.md) — arquivos em dados estruturados.
- [dominios/analise-ia.md](dominios/analise-ia.md) — dados em conhecimento.
- [dominios/grafo-conhecimento.md](dominios/grafo-conhecimento.md) — núcleo cognitivo do sistema.
- [dominios/sugestoes.md](dominios/sugestoes.md) — conhecimento em recomendações.
- [dominios/interface-usuario.md](dominios/interface-usuario.md) — telas e fluxos de UI.

---

# Casos de Uso

## 01 — Descoberta

- [UC-001 Escanear Diretório](casos-de-uso/01-descoberta/UC-001-escanear-diretorio.md)
- [UC-002 Indexar Arquivos](casos-de-uso/01-descoberta/UC-002-indexar-arquivo.md)

## 02 — Conhecimento

- [UC-003 Analisar Arquivos](casos-de-uso/02-conhecimento/UC-003-analisar-arquivos.md)
- [UC-008 Extrair Entidades](casos-de-uso/02-conhecimento/UC-008-extrair-entidades.md)
- [UC-009 Gerar Embeddings](casos-de-uso/02-conhecimento/UC-009-gerar-embeddings.md)
- [UC-010 Descobrir Relações](casos-de-uso/02-conhecimento/UC-010-descobrir-relacoes.md)
- [UC-011 Construir Clusters](casos-de-uso/02-conhecimento/UC-011-construir-clusters.md)
- [UC-004 Construir Grafo](casos-de-uso/02-conhecimento/UC-004-construir-grafo.md)

## 03 — Inteligência

- [UC-005 Gerar Sugestões](casos-de-uso/03-inteligencia/UC-005-gerar-sugestoes.md)
- [UC-012 Explicar Sugestões](casos-de-uso/03-inteligencia/UC-012-explicar-sugestoes.md)

## 04 — Execução

- [UC-013 Revisar Sugestões](casos-de-uso/04-execucao/UC-013-revisar-sugestoes.md)
- [UC-006 Aplicar Alterações](casos-de-uso/04-execucao/UC-006-aplicar-alteracoes.md)
- [UC-007 Desfazer Alterações](casos-de-uso/04-execucao/UC-007-desfazer-alteracoes.md)

## 05 — Exploração

- [UC-014 Busca Semântica](casos-de-uso/05-exploracao/UC-014-busca-semantica.md)
- [UC-015 Explorar Contexto](casos-de-uso/05-exploracao/UC-015-explorar-contexto.md)

---

# Arquitetura

- [arquitetura/visao-geral.md](arquitetura/visao-geral.md) — componentes e fluxos de alto nível.
- [arquitetura/tauri.md](arquitetura/tauri.md) — fronteira frontend ↔ backend.
- [arquitetura/contratos-tauri.md](arquitetura/contratos-tauri.md) — catálogo de commands.
- [arquitetura/catalogo-de-eventos.md](arquitetura/catalogo-de-eventos.md) — catálogo de events (fonte única).
- [arquitetura/banco-de-dados.md](arquitetura/banco-de-dados.md) — modelo conceitual de dados.
- [arquitetura/esquema-sql.md](arquitetura/esquema-sql.md) — esquema físico SQLite (fonte única).
- [arquitetura/pipeline-ia.md](arquitetura/pipeline-ia.md) — arquitetura técnica da IA.
- [arquitetura/estrutura-do-projeto.md](arquitetura/estrutura-do-projeto.md) — organização do código.
- [arquitetura/frontend-ui.md](arquitetura/frontend-ui.md) — fundações do frontend (tokens, temas, estados).
- [arquitetura/configuracao-e-seguranca.md](arquitetura/configuracao-e-seguranca.md) — config (TOML) e modelo de segurança.
- [arquitetura/observabilidade.md](arquitetura/observabilidade.md) — logs, métricas e histórico (local, sem telemetria).

---

# Decisões de Arquitetura

- [decisoes/README.md](decisoes/README.md) — convenção e índice dos ADRs.

| ADR | Título |
| --- | --- |
| [ADR-001](decisoes/ADR-001-react.md) | React como Framework Frontend |
| [ADR-002](decisoes/ADR-002-tauri.md) | Tauri como Plataforma Desktop |
| [ADR-003](decisoes/ADR-003-ia-local.md) | IA Local como Estratégia Principal |
| [ADR-004](decisoes/ADR-004-grafo-fonte-da-verdade.md) | Grafo como Fonte da Verdade |
| [ADR-005](decisoes/ADR-005-sqlite-persistencia.md) | SQLite como Persistência |
| [ADR-006](decisoes/ADR-006-indexacao-desacoplada.md) | Indexação Independente da IA |
| [ADR-007](decisoes/ADR-007-aprovacao-obrigatoria.md) | Aprovação Obrigatória |
| [ADR-008](decisoes/ADR-008-qwen3-4b.md) | Qwen 3 4B como Modelo Inicial |
| [ADR-009](decisoes/ADR-009-motor-de-sugestoes-orientado-a-conhecimento.md) | Motor de Sugestões Orientado a Conhecimento |
| [ADR-010](decisoes/ADR-010-estrategia-de-snapshot-e-rollback.md) | Estratégia de Snapshot e Rollback |
| [ADR-011](decisoes/ADR-011-tauri-v2-sqlx-vite.md) | Tauri v2, sqlx e Vite como Base Técnica |
| [ADR-012](decisoes/ADR-012-ui-shadcn-tailwind.md) | shadcn/ui + Tailwind como Base de UI |
| [ADR-013](decisoes/ADR-013-configuracao-e-observabilidade.md) | Configuração em Arquivo e Observabilidade Local |

---

# Planejamento

- [roadmap.md](roadmap.md) — ordem de construção por marcos.

---

# Fontes Únicas da Verdade

Para evitar divergências, alguns documentos têm autoridade final sobre seu tema:

| Tema | Documento |
| --- | --- |
| Comportamento esperado | Critérios de Aceitação dos casos de uso |
| Nomes de eventos | [catalogo-de-eventos.md](arquitetura/catalogo-de-eventos.md) |
| Esquema físico do banco | [esquema-sql.md](arquitetura/esquema-sql.md) |
| Conhecimento do sistema | Grafo de Conhecimento (ADR-004) |

---

# Fluxo do Sistema em Uma Linha

```text
Arquivos → Indexação → IA → Grafo → Sugestões → Revisão → Execução → (Desfazer)
                                       ↘ Exploração (busca e contexto)
```
