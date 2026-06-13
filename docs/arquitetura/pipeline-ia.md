# Arquitetura: Pipeline de IA

## Objetivo

Este documento descreve a arquitetura técnica do pipeline de Inteligência Artificial.

O domínio `analise-ia.md` define **o que** a IA faz e **por que**.

Este documento define **como** o pipeline é estruturado, orquestrado e executado.

---

# Princípios

## Desacoplamento da Indexação

A indexação não depende da IA (ver ADR-006).

O pipeline de IA consome arquivos previamente indexados a partir de uma fila.

---

## Execução Local

A inferência ocorre localmente por padrão (ver ADR-003).

Runtime inicial: Ollama. Modelo inicial: Qwen 3 4B (ver ADR-008).

---

## Assíncrono e Não Bloqueante

O processamento ocorre em segundo plano, comunicando progresso por eventos.

---

## Tolerância a Falhas

A falha em um arquivo não interrompe o pipeline.

Arquivos com falha permanecem disponíveis para nova tentativa.

---

## Substituível

Runtime e modelo são intercambiáveis sem alterar o domínio.

O pipeline depende de uma abstração de Serviço de IA, não de Ollama diretamente.

---

# Visão Geral

```text
Fila de Enriquecimento
↓
Pré-processamento
↓
Extração de Entidades
↓
Geração de Embeddings
↓
Descoberta de Relações
↓
Formação de Clusters
↓
Atualização do Grafo
↓
Persistência
```

Cada etapa corresponde a um caso de uso especificado:

- UC-008 Extrair Entidades
- UC-009 Gerar Embeddings
- UC-010 Descobrir Relações
- UC-011 Construir Clusters
- UC-004 Construir Grafo

---

# Fila de Processamento

## Origem

Arquivos com status `pending_analysis`.

---

## Responsabilidades

- detectar arquivos pendentes
- ordenar por prioridade
- evitar reprocessamento desnecessário
- controlar concorrência

---

## Controle de Concorrência

O número de inferências simultâneas é limitado para respeitar o hardware local (alvo inicial: 8 GB de RAM).

---

# Abstração do Serviço de IA

O pipeline interage com uma interface estável, independente do runtime.

Operações esperadas:

- extrair_entidades(texto)
- gerar_embedding(texto)
- inferir_relacoes(contexto)

Implementação inicial: adaptador Ollama.

Implementações futuras: outros runtimes ou APIs, sem impacto no pipeline.

---

# Etapas Técnicas

## Pré-processamento

- normalização de texto
- detecção de idioma
- segmentação de conteúdos extensos

---

## Extração de Entidades

Submissão do texto ao modelo e mapeamento das entidades para o esquema (`entities`, `file_entities`).

---

## Geração de Embeddings

Vetorização do conteúdo e persistência em `embeddings`, registrando o modelo utilizado.

---

## Descoberta de Relações

Inferência de conexões entre entidades e arquivos, persistidas em `relationships` com confiança e evidências.

---

## Formação de Clusters

Agrupamento por proximidade semântica, persistido em `clusters` e `cluster_members`.

---

## Atualização do Grafo

Incorporação do conhecimento ao Grafo de Conhecimento, reaproveitando nós existentes.

---

# Confiança

Toda inferência carrega um nível de confiança.

Faixas (coerentes com o domínio de Análise):

- 0.90 – 1.00 — alta
- 0.70 – 0.89 — média
- 0.50 – 0.69 — baixa
- < 0.50 — não utilizar para sugestões automáticas

---

# Tratamento de Falhas

## Falha de Modelo

O modelo não responde.

Ação: registrar erro; arquivo permanece pendente.

---

## Falha Parcial

Uma etapa falha, mas outras concluem.

Ação: persistir resultados disponíveis; marcar análise parcial.

---

## Reprocessamento

Arquivos com falha ou desatualizados retornam à fila.

---

# Eventos Emitidos

- AnalysisStarted
- EntityExtractionStarted
- EmbeddingGenerationStarted
- RelationsDiscovered
- GraphUpdated
- AnalysisCompleted
- AnalysisFailed

Comunicados ao frontend pela camada Tauri.

---

# Persistência

O pipeline escreve em:

- entities
- file_entities
- embeddings
- relationships
- clusters
- cluster_members

O grafo é reconstruído logicamente a partir dessas tabelas (ver ADR-005).

---

# Métricas

O pipeline deve registrar:

- tempo médio por etapa
- quantidade de entidades extraídas
- quantidade de relações criadas
- confiança média
- taxa de falhas

---

# Evolução Futura

A arquitetura deve permitir, sem alterar o domínio:

- múltiplos modelos simultâneos
- banco vetorial dedicado
- modelos especializados por etapa
- processamento distribuído

---

# Resumo

```text
Indexação (independente)
↓
Fila de Enriquecimento
↓
Serviço de IA (abstração)
   └─ Adaptador Ollama + Qwen 3 4B
↓
Entidades · Embeddings · Relações · Clusters
↓
Grafo de Conhecimento
↓
Motor de Sugestões
```

O pipeline transforma conteúdo em conhecimento, de forma local, assíncrona, tolerante a falhas e independente de qualquer runtime específico.
