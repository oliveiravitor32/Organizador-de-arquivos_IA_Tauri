# Caso de Uso: Gerar Embeddings

## Identificação

**ID:** UC-009

**Nome:** Gerar Embeddings

**Categoria:** Conhecimento

**Prioridade:** Crítica

---

# Objetivo

Produzir representações vetoriais do significado do conteúdo dos arquivos, permitindo comparações de proximidade semântica.

Esta etapa é um subprocesso da Análise de Arquivos (UC-003) e sustenta a busca semântica e a clusterização.

---

# Atores

## Primário

Sistema

---

## Secundários

- Serviço de Análise
- Modelo de Embeddings
- Banco de Dados

---

# Pré-condições

- Arquivo com conteúdo textual extraído.
- Modelo de embeddings disponível localmente.

---

# Pós-condições

## Sucesso

O arquivo possui ao menos um embedding persistido, associado ao modelo utilizado.

---

## Falha

Nenhum embedding é persistido.

A análise pode continuar de forma parcial (ver UC-003 FA-003).

---

# Fluxo Principal

## Passo 1

O sistema recebe o conteúdo textual do arquivo.

---

## Passo 2

O sistema prepara o texto para vetorização.

Operações possíveis:

- normalização
- segmentação em trechos

---

## Passo 3

O sistema submete o conteúdo ao modelo de embeddings.

---

## Passo 4

O modelo retorna a representação vetorial.

---

## Passo 5

O sistema registra o modelo utilizado.

Exemplo:

```text
qwen-embedding
```

---

## Passo 6

O sistema serializa e persiste o vetor.

---

# Fluxos Alternativos

## FA-001 — Conteúdo Vazio

### Condição

Arquivo sem texto útil.

### Ação

Não gerar embedding.

### Resultado

Arquivo continua participando do sistema com metadados mínimos.

---

## FA-002 — Falha de Vetorização

### Condição

Erro durante a geração.

### Ação

Persistir demais resultados da análise.

### Resultado

Análise parcial.

---

## FA-003 — Conteúdo Extenso

### Condição

Texto excede o limite do modelo.

### Ação

Segmentar e gerar múltiplos embeddings.

---

# Regras de Negócio

## RN-001

Todo embedding deve registrar o modelo que o gerou.

---

## RN-002

Embeddings são representações probabilísticas de significado.

---

## RN-003

A geração não modifica arquivos físicos.

---

## RN-004

O sistema deve suportar múltiplos modelos de embedding ao longo do tempo.

---

## RN-005

O processamento deve ocorrer localmente por padrão.

---

# Eventos Emitidos

## EmbeddingGenerationStarted

Geração iniciada.

---

## EmbeddingGenerated

Vetor produzido.

---

## EmbeddingGenerationFailed

Falha na geração.

---

# Dados Consumidos

## Conteúdo Textual

Texto extraído na indexação.

---

# Dados Produzidos

## Embedding

Representação vetorial serializada.

---

## Referência de Modelo

Identificação do modelo utilizado.

---

# Integrações

## Serviço de IA

Geração de vetores.

---

## Banco de Dados

Persistência na tabela `embeddings`.

---

# Critérios de Aceitação

## CA-001

Embeddings são gerados para arquivos com conteúdo.

---

## CA-002

Cada embedding registra o modelo utilizado.

---

## CA-003

Conteúdos extensos são segmentados.

---

## CA-004

Falhas não interrompem o pipeline global.

---

## CA-005

Os embeddings ficam disponíveis para busca e clusterização.

---

# Dependências

## Pré-requisitos

- UC-002 Indexar Arquivos

## Parte de

- UC-003 Analisar Arquivos

## Consumidores

- UC-011 Construir Clusters
- UC-014 Busca Semântica

---

# Observações Arquiteturais

Os embeddings traduzem significado em geometria.

A proximidade vetorial é o que permite ao sistema perceber semelhança sem depender de nomes, extensões ou localização física.

O modelo foi projetado para acomodar bancos vetoriais e múltiplos modelos no futuro, sem alterar o domínio principal.

---

# Fluxo Resumido

```text
Conteúdo Textual
↓
Preparação
↓
Vetorização
↓
Registro do Modelo
↓
Persistência do Vetor
```
