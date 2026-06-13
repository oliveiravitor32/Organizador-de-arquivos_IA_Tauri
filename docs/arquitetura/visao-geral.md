# Arquitetura: Visão Geral

## Objetivo

Este documento descreve a arquitetura de alto nível do sistema.

Seu objetivo é apresentar os principais componentes, responsabilidades e fluxos de dados da aplicação.

Não define detalhes de implementação.

---

# Visão Geral

O sistema é uma aplicação desktop local-first responsável por transformar arquivos em conhecimento estruturado.

A arquitetura é dividida em cinco grandes áreas:

- Interface
- Core de Aplicação
- Indexação
- Inteligência Artificial
- Persistência

---

# Arquitetura Conceitual

Usuário
↓
Interface React
↓
Tauri Commands
↓
Core da Aplicação (Rust)
↓
──────────────────────
↓ ↓
Indexação IA
↓ ↓
Persistência Grafo
↓ ↓
SQLite Conhecimento
↓
Sugestões
↓
Sistema de Arquivos

---

# Princípios Arquiteturais

## Local First

Todo o sistema deve funcionar localmente.

Nenhum serviço externo é obrigatório.

---

## Grafo como Fonte da Verdade

O Grafo de Conhecimento representa o entendimento oficial do sistema.

A estrutura física de diretórios é apenas uma projeção desse conhecimento.

---

## Processamento Assíncrono

Operações demoradas não devem bloquear a interface.

Exemplos:

- Escaneamento
- OCR
- Embeddings
- Inferências

---

## Modularidade

Cada componente deve possuir responsabilidade única.

---

## Reprocessamento Seguro

Qualquer etapa deve poder ser executada novamente sem comprometer a consistência dos dados.

---

# Camada de Interface

## Responsabilidade

Interação com o usuário.

Implementada utilizando React.

---

# Funcionalidades

- Seleção de diretórios
- Visualização de arquivos
- Busca
- Exploração do grafo
- Revisão de sugestões
- Configurações
- Monitoramento de progresso

---

# Comunicação

A interface não acessa diretamente:

- Banco de dados
- Sistema de arquivos
- Serviço de IA

Toda comunicação ocorre através do Tauri.

---

# Camada Tauri

## Responsabilidade

Atuar como ponte entre React e Rust.

---

# Responsabilidades

- Expor comandos
- Emitir eventos
- Encaminhar solicitações

---

# Exemplo

Frontend:

"Iniciar indexação"

↓

Command Tauri

↓

Serviço de Indexação

---

# Core da Aplicação

## Responsabilidade

Orquestrar todo o sistema.

Implementado em Rust.

---

# Responsabilidades

- Coordenar serviços
- Gerenciar estados
- Processar filas
- Aplicar regras de negócio

---

# Serviços Principais

## Serviço de Indexação

Responsável por:

- Escaneamento
- Descoberta de arquivos
- Extração de conteúdo

---

## Serviço de Persistência

Responsável por:

- SQLite
- Consultas
- Atualizações

---

## Serviço de IA

Responsável por:

- Comunicação com Ollama
- Extração de entidades
- Embeddings

---

## Serviço de Grafo

Responsável por:

- Construção do grafo
- Consultas semânticas
- Relacionamentos

---

## Serviço de Sugestões

Responsável por:

- Análise de contexto
- Agrupamentos
- Recomendações

---

## Serviço de Operações

Responsável por:

- Movimentação de arquivos
- Renomeações
- Rollback

---

# Subsistema de Indexação

## Objetivo

Transformar arquivos físicos em dados estruturados.

---

# Pipeline

Sistema de Arquivos
↓
Scanner
↓
Metadados
↓
Extração de Conteúdo
↓
Persistência
↓
Fila de IA

---

# Resultado

Arquivos indexados e prontos para enriquecimento.

---

# Subsistema de IA

## Objetivo

Transformar conteúdo em conhecimento.

---

# Componentes

## Ollama

Runtime de execução local.

---

## Modelo Principal

Inicialmente:

Qwen 3 4B

---

## Modelos Futuros

- Gemma
- Mistral
- Llama

---

# Pipeline

Conteúdo
↓
Extração de Entidades
↓
Embeddings
↓
Inferências
↓
Relacionamentos

---

# Resultado

Conhecimento estruturado.

---

# Subsistema de Grafo

## Objetivo

Representar relações entre informações.

---

# Elementos

Nós:

- Arquivos
- Entidades
- Clusters

Relações:

- menciona
- relacionado_com
- similar_a
- pertence_a

---

# Responsabilidades

- Navegação
- Consultas
- Descoberta de conexões

---

# Persistência

## Tecnologia

SQLite

---

# Dados Armazenados

- Arquivos
- Conteúdo
- Entidades
- Relacionamentos
- Embeddings
- Clusters
- Sugestões
- Snapshots

---

# Observação

O banco não é o grafo.

O banco apenas armazena os dados necessários para reconstruí-lo.

---

# Subsistema de Sugestões

## Objetivo

Transformar conhecimento em ações úteis.

---

# Fontes

- Grafo
- Entidades
- Similaridade
- Clusters

---

# Exemplos

- Agrupamento de documentos
- Sugestão de diretórios
- Renomeação
- Consolidação de estruturas

---

# Requisito Fundamental

Toda sugestão deve possuir:

- Justificativa
- Evidências
- Nível de confiança

---

# Subsistema de Operações

## Objetivo

Aplicar alterações aprovadas.

---

# Operações

- Mover arquivo
- Renomear arquivo
- Criar diretório

---

# Segurança

Antes de qualquer alteração:

1. Criar snapshot
2. Registrar plano de execução
3. Executar alterações
4. Registrar resultado

---

# Fluxo Principal do Sistema

Usuário seleciona diretório
↓
Indexação
↓
Extração de conteúdo
↓
Persistência
↓
IA
↓
Entidades
↓
Embeddings
↓
Grafo
↓
Sugestões
↓
Revisão do usuário
↓
Aplicação das alterações

---

# Escalabilidade Futura

A arquitetura deve permitir evolução para:

- Múltiplos modelos de IA
- Banco vetorial dedicado
- Banco de grafos dedicado
- Monitoramento em tempo real
- Busca semântica avançada
- Agentes especializados

Sem alterar os conceitos centrais do domínio.

---

# Resumo Arquitetural

Frontend

React

↓

Desktop Runtime

Tauri

↓

Backend

Rust

↓

Persistência

SQLite

↓

Inteligência Artificial

Ollama + Qwen

↓

Conhecimento

Grafo de Conhecimento

↓

Valor ao Usuário

Descoberta, compreensão e organização inteligente dos arquivos.
