# Caso de Uso: Construir Grafo

## Identificação

**ID:** UC-004

**Nome:** Construir Grafo

**Categoria:** Conhecimento

**Prioridade:** Crítica

---

# Objetivo

Construir e manter o Grafo de Conhecimento do sistema a partir das informações produzidas durante a análise dos arquivos.

O Grafo de Conhecimento representa a compreensão acumulada do sistema sobre os arquivos, entidades, contextos e relações existentes.

---

# Atores

## Primário

Sistema

---

## Secundários

- Serviço de Grafo
- Banco de Dados
- Serviço de Análise
- Motor de Sugestões

---

# Pré-condições

- Arquivos analisados.
- Entidades identificadas.
- Embeddings disponíveis.
- Relações descobertas.

Status esperado:

```text
ANALYZED
```

---

# Pós-condições

## Sucesso

O conhecimento produzido é incorporado ao Grafo de Conhecimento.

---

## Falha

O conhecimento permanece persistido, mas não disponível para consultas através do grafo.

---

# Objetivo de Negócio

Transformar informações isoladas em uma rede de conhecimento capaz de revelar:

- contextos
- conexões
- agrupamentos
- padrões

que não são visíveis apenas pela estrutura física dos diretórios.

---

# Fluxo Principal

## Passo 1

O sistema identifica conhecimento pendente de incorporação ao grafo.

---

## Passo 2

O sistema carrega:

- entidades
- embeddings
- relações
- clusters
- metadados

associados aos arquivos analisados.

---

## Passo 3

O sistema verifica a existência prévia dos nós.

---

## Passo 4

Nós já existentes são reutilizados.

---

## Passo 5

Novos nós são criados quando necessário.

---

## Passo 6

O sistema cria ou atualiza relacionamentos.

---

## Passo 7

O sistema calcula métricas estruturais do grafo.

---

## Exemplos

- centralidade
- conectividade
- densidade local
- importância relativa

---

## Passo 8

O sistema identifica comunidades e agrupamentos emergentes.

---

## Passo 9

O sistema recalcula contextos afetados.

---

## Passo 10

O estado consolidado do grafo é persistido.

---

## Passo 11

O sistema atualiza índices de consulta.

---

## Passo 12

O grafo fica disponível para exploração e geração de sugestões.

---

# Estrutura Conceitual

O grafo é composto por:

## Nós

Representam elementos do conhecimento.

---

## Arestas

Representam relações entre elementos.

---

## Propriedades

Representam informações adicionais associadas aos elementos.

---

# Tipos de Nós

## Arquivo

Representa um arquivo físico indexado.

---

## Entidade

Representa um conceito identificado.

Exemplos:

- pessoa
- organização
- projeto
- tema

---

## Cluster

Representa um agrupamento semântico.

---

## Contexto

Representa um conjunto coerente de conhecimento.

---

# Tipos de Relações

## menciona

Arquivo → Entidade

---

## relacionado_com

Entidade → Entidade

---

## pertence_a

Arquivo → Cluster

---

## participa_de

Entidade → Contexto

---

## similar_a

Arquivo → Arquivo

---

## referencia

Arquivo → Arquivo

---

# Fluxos Alternativos

## FA-001 — Nó Duplicado

### Condição

Entidade semelhante já existe.

### Ação

Realizar fusão lógica.

### Resultado

Evitar duplicação de conhecimento.

---

## FA-002 — Relação Duplicada

### Condição

Relacionamento já existe.

### Ação

Atualizar pesos e evidências.

### Resultado

Reforço da confiança.

---

## FA-003 — Cluster Inconsistente

### Condição

Agrupamento apresenta baixa coerência.

### Ação

Marcar para reavaliação futura.

---

## FA-004 — Falha de Persistência

### Condição

Erro ao salvar.

### Resultado

Operação abortada sem corromper o estado anterior.

---

# Regras de Negócio

## RN-001

O grafo é a representação oficial do conhecimento do sistema.

---

## RN-002

A estrutura de diretórios não é considerada fonte da verdade.

---

## RN-003

Todo nó deve possuir identificador único.

---

## RN-004

Toda relação deve possuir confiança.

---

## RN-005

Toda inferência deve possuir evidências rastreáveis.

---

## RN-006

Conhecimento duplicado deve ser consolidado sempre que possível.

---

## RN-007

O histórico de evidências deve ser preservado.

---

# Dados Consumidos

## Arquivos

- identificador
- metadados

---

## Entidades

- nome
- tipo
- confiança

---

## Relações

- origem
- destino
- tipo
- confiança

---

## Embeddings

Representações vetoriais.

---

## Clusters

Agrupamentos identificados.

---

# Dados Produzidos

## Nós

Conhecimento consolidado.

---

## Relações

Conexões navegáveis.

---

## Contextos

Agrupamentos semânticos.

---

## Comunidades

Estruturas emergentes.

---

## Índices

Estruturas auxiliares para consulta.

---

# Eventos Emitidos

## GraphBuildStarted

Início da construção.

---

## NodeCreated

Novo nó criado.

---

## NodeMerged

Nós consolidados.

---

## RelationCreated

Relacionamento criado.

---

## ClusterUpdated

Cluster atualizado.

---

## GraphUpdated

Grafo atualizado.

---

## GraphBuildCompleted

Processo concluído.

---

## GraphBuildFailed

Falha durante a atualização.

---

# Integrações

## Serviço de Análise

Origem do conhecimento.

---

## Banco de Dados

Persistência.

---

## Motor de Sugestões

Consumidor principal do grafo.

---

## Busca Semântica

Consumidor secundário.

---

# Critérios de Aceitação

CA-001

Entidades são representadas como nós.

---

CA-002

Relações são representadas como arestas.

---

CA-003

Conhecimento duplicado é consolidado.

---

CA-004

O grafo pode ser consultado.

---

CA-005

O grafo pode ser utilizado pelo motor de sugestões.

---

CA-006

A atualização não corrompe conhecimento existente.

---

# Métricas de Qualidade

O sistema deve monitorar:

- quantidade de nós
- quantidade de relações
- densidade do grafo
- entidades órfãs
- clusters ativos
- contextos identificados

---

# Dependências

Pré-requisitos:

- UC-001 Escanear Diretório
- UC-002 Indexar Arquivos
- UC-003 Analisar Arquivos

Consumidores:

- UC-005 Gerar Sugestões
- UC-014 Busca Semântica
- UC-015 Explorar Contexto

---

# Observações Arquiteturais

O Grafo de Conhecimento é o núcleo cognitivo do sistema.

Ele não representa arquivos.

Ele representa significado.

A estrutura física dos diretórios pode mudar completamente sem alterar o conhecimento armazenado.

O grafo deve sobreviver a reorganizações físicas e continuar preservando o entendimento construído pelo sistema.

---

# Fluxo Resumido

Arquivos Analisados
↓
Entidades
↓
Relações
↓
Clusters
↓
Consolidação
↓
Grafo de Conhecimento
↓
Consulta
↓
Sugestões
↓
Organização Inteligente
