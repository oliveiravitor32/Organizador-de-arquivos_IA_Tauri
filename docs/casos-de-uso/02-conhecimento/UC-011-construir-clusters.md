# Caso de Uso: Construir Clusters

## Identificação

**ID:** UC-011

**Nome:** Construir Clusters

**Categoria:** Conhecimento

**Prioridade:** Alta

---

# Objetivo

Agrupar arquivos e entidades semanticamente próximos em clusters, revelando organizações naturais que emergem do conhecimento.

Esta etapa é um subprocesso da Análise de Arquivos (UC-003) e alimenta diretamente o Motor de Sugestões.

---

# Atores

## Primário

Sistema

---

## Secundários

- Serviço de Análise
- Banco de Dados

---

# Pré-condições

- Embeddings disponíveis (UC-009).
- Entidades e relações identificadas.

---

# Pós-condições

## Sucesso

Clusters são criados ou atualizados, com membros e confiança associados.

---

## Falha

Nenhum cluster é alterado.

O conhecimento existente permanece intacto.

---

# Fluxo Principal

## Passo 1

O sistema carrega embeddings, entidades e relações.

---

## Passo 2

O sistema calcula a proximidade semântica entre elementos.

---

## Passo 3

O sistema identifica agrupamentos naturais.

---

## Passo 4

O sistema avalia a coerência de cada agrupamento.

---

## Passo 5

O sistema gera nome e descrição sugeridos para cada cluster.

---

## Passo 6

O sistema associa membros ao cluster.

Membros possíveis:

- arquivos
- entidades

---

## Passo 7

O sistema atribui confiança ao cluster.

---

## Passo 8

O sistema persiste clusters e membros.

---

# Fluxos Alternativos

## FA-001 — Cluster Inconsistente

### Condição

Agrupamento com baixa coerência.

### Ação

Marcar para reavaliação futura.

### Resultado

Cluster não é utilizado em sugestões definitivas.

---

## FA-002 — Sobreposição de Clusters

### Condição

Elemento pertence a múltiplos agrupamentos.

### Ação

Permitir associação múltipla com confiança distinta.

---

## FA-003 — Dados Insuficientes

### Condição

Embeddings insuficientes para agrupar.

### Resultado

Nenhum cluster é gerado.

---

# Regras de Negócio

## RN-001

Todo cluster deve possuir confiança.

---

## RN-002

Clusters de baixa coerência não geram sugestões definitivas.

---

## RN-003

Um elemento pode pertencer a mais de um cluster.

---

## RN-004

A clusterização não modifica arquivos físicos.

---

## RN-005

Nome e descrição de cluster são sugestões, não verdades.

---

# Eventos Emitidos

## ClusteringStarted

Clusterização iniciada.

---

## ClusterCreated

Cluster criado.

---

## ClusterUpdated

Cluster atualizado.

---

## ClusteringCompleted

Clusterização concluída.

---

## ClusteringFailed

Falha na clusterização.

---

# Dados Consumidos

## Embeddings

Proximidade semântica.

---

## Entidades

Conhecimento identificado.

---

## Relações

Conexões inferidas.

---

# Dados Produzidos

## Clusters

Agrupamentos semânticos.

---

## Membros de Cluster

Arquivos e entidades associados.

---

# Integrações

## Serviço de Análise

Cálculo de agrupamentos.

---

## Banco de Dados

Persistência em `clusters` e `cluster_members`.

---

# Critérios de Aceitação

## CA-001

Agrupamentos naturais são identificados.

---

## CA-002

Cada cluster possui confiança.

---

## CA-003

Membros são associados aos clusters.

---

## CA-004

Clusters incoerentes são sinalizados.

---

## CA-005

Clusters ficam disponíveis para o grafo e o motor de sugestões.

---

# Dependências

## Pré-requisitos

- UC-009 Gerar Embeddings
- UC-010 Descobrir Relações

## Parte de

- UC-003 Analisar Arquivos

## Consumidores

- UC-004 Construir Grafo
- UC-005 Gerar Sugestões

---

# Observações Arquiteturais

Os clusters são a manifestação da organização emergente defendida pelo ADR-009.

Não existe uma estrutura ideal universal: os agrupamentos surgem dos padrões encontrados nos dados.

A estrutura física proposta ao usuário é consequência dos clusters, e nunca o contrário.

---

# Fluxo Resumido

```text
Embeddings e Relações
↓
Proximidade Semântica
↓
Agrupamentos Naturais
↓
Avaliação de Coerência
↓
Nome e Membros
↓
Persistência
```
