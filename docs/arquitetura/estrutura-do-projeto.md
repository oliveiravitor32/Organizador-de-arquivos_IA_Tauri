# Arquitetura: Estrutura do Projeto

## Objetivo

Este documento define a organização de pastas do código-fonte.

Ele traduz os componentes descritos em `visao-geral.md` e os serviços do núcleo em uma estrutura física de diretórios, separando frontend (React) e backend (Rust) conforme a fronteira definida em `tauri.md`.

---

# Princípios

## Espelhar o Domínio

A estrutura de pastas deve refletir os domínios e serviços, não detalhes técnicos acidentais.

---

## Fronteira Explícita

Frontend e backend são fisicamente separados.

A comunicação ocorre apenas pelos contratos de `contratos-tauri.md`.

---

## Responsabilidade Única

Cada serviço do núcleo corresponde a um módulo isolado.

---

## Independência de Reprocessamento

A indexação não importa código da IA (ver ADR-006).

---

# Estrutura de Alto Nível

```text
project-root/
├── docs/                  # Especificação (este conjunto de documentos)
├── src/                   # Frontend React
├── src-tauri/             # Backend Rust + camada Tauri
├── tests/                 # Testes de integração e e2e
├── package.json
└── README.md
```

---

# Frontend (src/)

```text
src/
├── main.tsx               # Ponto de entrada
├── app/                   # Composição da aplicação e rotas
├── pages/                 # Telas principais
│   ├── Indexacao/
│   ├── Sugestoes/
│   ├── Execucao/
│   ├── Historico/
│   └── Exploracao/
├── components/            # Componentes reutilizáveis
├── features/              # Lógica por funcionalidade
│   ├── descoberta/
│   ├── conhecimento/
│   ├── sugestoes/
│   ├── execucao/
│   └── exploracao/
├── ipc/                   # Wrappers dos commands e listeners de events
│   ├── commands.ts
│   └── events.ts
├── hooks/
├── stores/                # Estado de UI
└── styles/
```

## Responsabilidades do Frontend

- apresentação e interação
- invocação de commands
- escuta de events de progresso
- nenhum acesso direto a arquivos, banco ou IA

---

# Backend (src-tauri/)

```text
src-tauri/
├── Cargo.toml
├── tauri.conf.json
└── src/
    ├── main.rs            # Bootstrap do Tauri
    ├── commands/          # Handlers dos commands (fronteira)
    ├── events/            # Definição e emissão de events
    ├── core/              # Orquestração e regras de negócio
    │   ├── mod.rs
    │   └── state.rs       # Estado compartilhado
    ├── services/          # Serviços do núcleo
    │   ├── indexacao/
    │   ├── persistencia/
    │   ├── ia/
    │   ├── grafo/
    │   ├── sugestoes/
    │   └── operacoes/
    ├── domain/            # Modelos de domínio e tipos
    ├── db/                # Migrações e acesso ao SQLite
    │   ├── migrations/
    │   └── repositories/
    └── error.rs           # Tipos de erro estruturados
```

---

# Mapeamento Serviço → Módulo

| Serviço (visao-geral) | Módulo | Casos de uso |
| --- | --- | --- |
| Serviço de Indexação | services/indexacao | UC-001, UC-002 |
| Serviço de IA | services/ia | UC-003, UC-008, UC-009, UC-010, UC-011 |
| Serviço de Grafo | services/grafo | UC-004, UC-015 |
| Serviço de Sugestões | services/sugestoes | UC-005, UC-012, UC-013 |
| Serviço de Operações | services/operacoes | UC-006, UC-007 |
| Serviço de Persistência | services/persistencia + db | transversal |

A busca semântica (UC-014) é atendida por `services/grafo` e `services/ia` em conjunto.

---

# Camada de Comandos

```text
commands/
├── descoberta.rs     # escanear_diretorio, indexar_arquivos
├── conhecimento.rs   # analisar_arquivos, construir_grafo
├── inteligencia.rs   # gerar_sugestoes, explicar_sugestao
├── revisao.rs        # listar/aprovar/rejeitar/ajustar_sugestao
├── execucao.rs       # aplicar_alteracoes, desfazer_alteracoes, listar_execucoes
├── exploracao.rs     # buscar_semantica, explorar_contexto
└── sistema.rs        # cancelar_operacao, configuracao
```

Cada handler valida a entrada, delega ao serviço correspondente e retorna erro estruturado.

---

# Abstração da IA

```text
services/ia/
├── mod.rs            # Interface estável do Serviço de IA
├── pipeline.rs       # Orquestração do pipeline (ver pipeline-ia.md)
└── adapters/
    └── ollama.rs     # Adaptador inicial (Qwen 3 4B)
```

O pipeline depende da interface, nunca do adaptador diretamente — permitindo substituir o runtime sem afetar o restante.

---

# Acesso ao Banco

```text
db/
├── migrations/       # DDL incremental (ver esquema-sql.md)
└── repositories/     # Acesso por agregado
    ├── files.rs
    ├── entities.rs
    ├── relationships.rs
    ├── embeddings.rs
    ├── clusters.rs
    ├── suggestions.rs
    └── snapshots.rs
```

Apenas os repositórios acessam o SQLite; os serviços consomem repositórios.

---

# Testes

Testes **unitários ficam colocados junto ao código** (Rust inline em `#[cfg(test)]`; frontend em arquivos `*.test.ts(x)` ao lado do arquivo). Apenas integração e e2e ficam em `tests/`.

```text
tests/
├── integracao/       # Serviços contra banco/FS reais
└── e2e/              # Fluxos completos via commands

src-tauri/src/**/*.rs # unidade inline (#[cfg(test)])
src/**/*.test.ts(x)   # unidade/componente colocados
```

Estratégia em `requisitos/estrategia-de-testes.md`; convenções de arquivo e ferramentas em `requisitos/convencoes-de-teste.md` (ADR-014).

---

# Resumo

```text
React (src/)
  └─ ipc → commands / events
Tauri (src-tauri/src/commands, events)
  └─ core (orquestração)
       └─ services (indexacao, ia, grafo, sugestoes, operacoes, persistencia)
            └─ db (repositories → SQLite)
```

A estrutura física reforça a fronteira única, a responsabilidade por serviço e a substituibilidade da IA definidas na arquitetura.
