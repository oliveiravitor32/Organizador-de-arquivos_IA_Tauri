# Domínio: Grafo de Conhecimento

## Objetivo

O Grafo de Conhecimento é a principal representação de conhecimento do sistema.

Seu objetivo é armazenar, conectar e representar informações extraídas dos arquivos analisados.

Toda inferência, agrupamento, busca semântica e sugestão de organização deve ser baseada no grafo.

A estrutura física de diretórios não é considerada fonte da verdade.

---

# Conceitos Fundamentais

O grafo é composto por:

- Nós
- Relações

Os nós representam elementos de conhecimento.

As relações representam conexões entre esses elementos.

---

# Estrutura Geral

Arquivo
↓
Entidades
↓
Relações
↓
Clusters
↓
Sugestões

---

# Tipos de Nós

## Nó Arquivo

Representa um arquivo físico existente no sistema.

### Responsabilidades

- Representar um arquivo indexado.
- Armazenar metadados.
- Servir como origem para extração de conhecimento.

### Atributos

- id
- caminho
- nome
- extensão
- tamanho
- hash
- data_criacao
- data_modificacao
- data_indexacao

### Exemplo

Contrato_Cliente_X.pdf

---

## Nó Entidade

Representa um conceito identificado pelo sistema.

Entidades não correspondem necessariamente a arquivos.

Podem representar pessoas, projetos, organizações, temas ou qualquer conceito relevante encontrado durante a análise.

### Exemplos

- Cliente XPTO
- Projeto Alpha
- João Silva
- Redes de Computadores
- Contrato

### Atributos

- id
- nome
- tipo
- confiança
- data_criacao

---

## Nó Cluster

Representa um agrupamento semântico.

Um cluster é formado por arquivos e entidades fortemente relacionados.

### Exemplos

Cluster: Projeto Alpha

Arquivos:

- proposta.pdf
- cronograma.xlsx
- reuniao.docx

Entidades:

- Projeto Alpha
- Cliente XPTO

### Atributos

- id
- nome
- descrição
- confiança

---

# Tipos de Relações

Toda relação deve possuir:

- origem
- destino
- tipo
- confiança

---

## MENCIONA

Indica que um arquivo contém referência a uma entidade.

### Exemplo

Contrato.pdf

MENCIONA

Cliente XPTO

---

## RELACIONADO_COM

Indica associação semântica entre dois nós.

### Exemplo

Projeto Alpha

RELACIONADO_COM

Projeto Beta

---

## SIMILAR_A

Indica proximidade semântica baseada em embeddings.

### Exemplo

proposta_v1.docx

SIMILAR_A

proposta_final.docx

---

## PERTENCE_A

Indica participação em um cluster.

### Exemplo

Contrato.pdf

PERTENCE_A

Cluster Cliente XPTO

---

## DERIVADO_DE

Indica que uma informação foi inferida a partir de outra.

### Exemplo

Cluster Projeto Alpha

DERIVADO_DE

Conjunto de arquivos relacionados

---

# Níveis de Confiança

Toda inferência deve possuir um nível de confiança.

Faixas sugeridas:

0.90 - 1.00

Alta confiança

---

0.70 - 0.89

Média confiança

---

0.50 - 0.69

Baixa confiança

---

Abaixo de 0.50

Não gerar sugestões automaticamente.

---

# Construção do Grafo

## Etapa 1

Indexação

O sistema identifica arquivos.

Resultado:

Nós Arquivo.

---

## Etapa 2

Extração de Conteúdo

O sistema obtém texto dos arquivos.

Resultado:

Conteúdo disponível para análise.

---

## Etapa 3

Extração de Entidades

A IA identifica conceitos relevantes.

Resultado:

Nós Entidade.

---

## Etapa 4

Relacionamento

O sistema conecta arquivos e entidades.

Resultado:

Arestas entre nós.

---

## Etapa 5

Similaridade

Embeddings são gerados.

Arquivos semanticamente próximos são conectados.

Resultado:

Relações SIMILAR_A.

---

## Etapa 6

Agrupamento

Clusters são criados.

Resultado:

Nós Cluster.

---

# Consultas Esperadas

O grafo deve permitir responder perguntas como:

## Arquivos relacionados

"Quais arquivos estão relacionados a este documento?"

---

## Entidades relacionadas

"Quais entidades aparecem neste arquivo?"

---

## Contexto compartilhado

"Quais arquivos mencionam o mesmo cliente?"

---

## Similaridade

"Quais arquivos possuem conteúdo semelhante?"

---

## Agrupamentos

"Quais clusters existem atualmente?"

---

## Navegação

"Como este arquivo se conecta aos demais?"

---

# Papel dos Embeddings

Embeddings não substituem entidades.

Embeddings complementam entidades.

Entidades representam conhecimento explícito.

Embeddings representam conhecimento implícito.

Exemplo:

Arquivo A

"Proposta Comercial Cliente XPTO"

Arquivo B

"Orçamento Final XPTO"

Mesmo sem palavras idênticas, embeddings podem indicar proximidade semântica.

---

# Papel da Inteligência Artificial

A IA é responsável por:

- Extrair entidades.
- Identificar temas.
- Detectar relações.
- Gerar embeddings.
- Sugerir agrupamentos.

A IA não altera diretamente o sistema de arquivos.

Seu papel é enriquecer o grafo de conhecimento.

---

# Fonte da Verdade

O Grafo de Conhecimento é a representação oficial do conhecimento do sistema.

Pastas, diretórios e localização física dos arquivos são considerados projeções desse conhecimento.

Toda sugestão de organização deve ser derivada do grafo.

Nunca o contrário.
