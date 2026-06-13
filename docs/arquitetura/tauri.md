# Arquitetura: Camada Tauri

## Objetivo

Este documento descreve a camada Tauri, responsável por conectar a interface (React) ao núcleo da aplicação (Rust).

A decisão de adotar Tauri está registrada no ADR-002. Este documento trata de **como** a camada é estruturada, não do porquê.

---

# Papel da Camada

A camada Tauri é a fronteira entre dois mundos:

- Frontend — React, executado em uma WebView.
- Backend — Rust, com acesso a sistema de arquivos, banco e IA.

Toda comunicação entre eles passa por esta camada.

---

# Princípios

## Fronteira Única

O frontend nunca acessa diretamente:

- sistema de arquivos
- banco de dados
- serviço de IA

Todo acesso ocorre através de comandos Tauri.

---

## Backend Autoritativo

Regras de negócio, validação e persistência residem no backend.

O frontend é uma camada de apresentação.

---

## Comunicação Assíncrona

Operações demoradas não bloqueiam a interface.

O progresso é comunicado por eventos.

---

## Contratos Explícitos

Cada comando possui entrada e saída bem definidas.

---

# Mecanismos de Comunicação

## Commands

Chamadas do frontend para o backend.

Fluxo:

```text
React
↓ invoke
Command Tauri
↓
Serviço (Rust)
↓
Resposta
```

Exemplos:

- iniciar_indexacao
- listar_sugestoes
- aprovar_sugestao
- aplicar_alteracoes
- desfazer_alteracoes
- buscar_semantica

---

## Events

Mensagens do backend para o frontend.

Utilizados para progresso e notificações assíncronas.

Exemplos:

- IndexingProgress
- AnalysisCompleted
- ExecutionProgress
- RollbackProgress

---

## State

Estado compartilhado gerenciado pelo backend.

Exemplos:

- conexão com o banco
- filas de processamento
- configuração do serviço de IA

---

# Responsabilidades

## Da Camada Tauri

- expor comandos
- emitir eventos
- encaminhar solicitações ao núcleo
- serializar e desserializar dados

---

## Fora da Camada Tauri

- lógica de negócio
- persistência
- inferência de IA
- manipulação física de arquivos

Essas responsabilidades pertencem aos serviços do núcleo.

---

# Fluxo de Exemplo

```text
Usuário clica "Indexar"
↓
React invoke("iniciar_indexacao", { caminho })
↓
Command Tauri
↓
Serviço de Indexação (Rust)
↓
Eventos IndexingProgress
↓
React atualiza a UI
```

---

# Segurança

## Superfície Restrita

Apenas comandos explicitamente registrados são acessíveis ao frontend.

---

## Validação no Backend

Toda entrada vinda do frontend é validada no Rust antes de uso.

---

## Permissões de Sistema

O acesso a arquivos respeita o escopo do diretório raiz definido pelo domínio de Sistema de Arquivos.

---

# Tratamento de Erros

Erros do backend são retornados ao frontend de forma estruturada.

O frontend nunca recebe estados inconsistentes silenciosamente.

---

# Evolução Futura

A camada deve permitir, sem alterar a interface:

- substituição do serviço de IA
- novos comandos
- novos eventos de progresso

A estabilidade dos contratos é prioridade.

---

# Resumo

```text
React (apresentação)
↓ commands / events
Tauri (fronteira)
↓
Core Rust (regras, serviços)
↓
SQLite · IA · Sistema de Arquivos
```

A camada Tauri garante que a interface permaneça simples e que toda a autoridade do sistema resida no backend.
