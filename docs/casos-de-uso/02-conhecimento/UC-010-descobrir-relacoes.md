# Caso de Uso: Descobrir Relações

## Identificação

**ID:** UC-010

**Nome:** Descobrir Relações

**Categoria:** Conhecimento

**Prioridade:** Crítica

---

# Objetivo

Identificar conexões relevantes entre entidades e entre arquivos, enriquecendo o conhecimento com relações que não são visíveis pela estrutura física.

Esta etapa é um subprocesso da Análise de Arquivos (UC-003).

---

# Atores

## Primário

Sistema

---

## Secundários

- Serviço de Análise
- Modelo de IA
- Banco de Dados

---

# Pré-condições

- Entidades extraídas (UC-008).
- Embeddings disponíveis (UC-009) quando aplicável.

---

# Pós-condições

## Sucesso

Relações são identificadas, qualificadas com confiança e persistidas.

---

## Falha

Nenhuma relação é criada.

O conhecimento já existente permanece intacto.

---

# Fluxo Principal

## Passo 1

O sistema carrega entidades e contexto do arquivo.

---

## Passo 2

O sistema analisa coocorrência e proximidade semântica entre entidades.

---

## Passo 3

O sistema infere relações candidatas.

Exemplos:

- related_to
- parent_of
- derived_from

---

## Passo 4

O sistema identifica relações entre arquivos.

Exemplos:

- referencia
- similar_a

---

## Passo 5

O sistema atribui confiança e evidências a cada relação.

---

## Passo 6

O sistema verifica se a relação já existe.

---

## Passo 7

Relações existentes têm peso e evidências reforçados.

Relações novas são criadas.

---

## Passo 8

O sistema persiste as relações.

---

# Fluxos Alternativos

## FA-001 — Relação Duplicada

### Condição

Relacionamento já existe.

### Ação

Atualizar peso e evidências.

### Resultado

Reforço da confiança.

---

## FA-002 — Baixa Confiança

### Condição

Relação inferida com confiança insuficiente.

### Ação

Registrar como candidata.

### Resultado

Não incorporar ao grafo como definitiva.

---

## FA-003 — Entidades Insuficientes

### Condição

Não há entidades suficientes para relacionar.

### Resultado

Nenhuma relação é produzida.

---

# Regras de Negócio

## RN-001

Toda relação deve possuir confiança.

---

## RN-002

Toda relação deve possuir evidências rastreáveis.

---

## RN-003

Relações duplicadas devem ser consolidadas.

---

## RN-004

Relações de baixa confiança não são definitivas.

---

## RN-005

A descoberta não modifica arquivos físicos.

---

# Eventos Emitidos

## RelationDiscoveryStarted

Descoberta iniciada.

---

## RelationDiscovered

Relação identificada.

---

## RelationReinforced

Relação existente reforçada.

---

## RelationDiscoveryCompleted

Descoberta concluída.

---

## RelationDiscoveryFailed

Falha na descoberta.

---

# Dados Consumidos

## Entidades

Conhecimento identificado.

---

## Embeddings

Proximidade semântica.

---

## Conhecimento Existente

Relações previamente conhecidas.

---

# Dados Produzidos

## Relações

Conexões inferidas.

---

## Evidências

Justificativas das inferências.

---

# Integrações

## Serviço de IA

Inferência de relações.

---

## Banco de Dados

Persistência na tabela `relationships`.

---

# Critérios de Aceitação

## CA-001

Relações entre entidades são identificadas.

---

## CA-002

Relações entre arquivos são identificadas.

---

## CA-003

Cada relação possui confiança e evidências.

---

## CA-004

Relações duplicadas são consolidadas.

---

## CA-005

Relações ficam disponíveis para o grafo.

---

# Dependências

## Pré-requisitos

- UC-008 Extrair Entidades
- UC-009 Gerar Embeddings

## Parte de

- UC-003 Analisar Arquivos

## Consumidores

- UC-004 Construir Grafo
- UC-011 Construir Clusters

---

# Observações Arquiteturais

As relações transformam entidades isoladas em uma rede de conhecimento.

São elas que permitem revelar contextos e conexões ocultas pela organização física dos diretórios.

Toda relação é tratada como hipótese sustentada por evidências, sujeita a reforço ou revisão.

---

# Fluxo Resumido

```text
Entidades
↓
Coocorrência e Proximidade
↓
Inferência de Relações
↓
Confiança e Evidências
↓
Consolidação
↓
Persistência
```
