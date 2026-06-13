# Arquitetura: Banco de Dados

## Objetivo

O banco de dados é responsável por persistir todas as informações produzidas pelo processo de indexação e análise.

Ele não é o Grafo de Conhecimento.

O banco é a camada de armazenamento.

O Grafo de Conhecimento é uma representação lógica construída a partir dos dados armazenados.

---

# Tecnologia

## Banco Principal

SQLite

Motivos:

- Local-first
- Sem servidor
- Leve
- Compatível com Tauri
- Excelente desempenho para centenas de milhares de registros
- Fácil backup

---

# Princípios

## Persistência Primeiro

Toda informação deve ser persistida antes de ser utilizada pelo sistema.

---

## Grafo Derivado

O grafo deve ser reconstruível a partir dos dados armazenados.

---

## Reprocessamento Seguro

Qualquer arquivo pode ser reanalisado sem comprometer a integridade do sistema.

---

## Auditabilidade

Todas as operações relevantes devem ser rastreáveis.

---

# Visão Geral

Arquivos
↓
Conteúdo
↓
Entidades
↓
Relacionamentos
↓
Clusters
↓
Sugestões
↓
Operações

---

# Tabela: files

Representa arquivos indexados.

## Campos

id

UUID

Identificador interno.

---

path

TEXT

Caminho absoluto.

---

relative_path

TEXT

Caminho relativo ao diretório raiz.

---

name

TEXT

Nome do arquivo.

---

extension

TEXT

Extensão.

---

size

INTEGER

Tamanho em bytes.

---

hash

TEXT

Hash do conteúdo.

---

created_at

DATETIME

Data de criação.

---

modified_at

DATETIME

Data de modificação.

---

indexed_at

DATETIME

Data da indexação.

---

status

TEXT

Estado atual do processamento.

Valores:

- discovered
- indexed
- pending_analysis
- analyzed
- failed

---

# Tabela: file_contents

Conteúdo textual extraído.

## Campos

id

UUID

---

file_id

FK → files

---

content

TEXT

Texto extraído.

---

language

TEXT

Idioma detectado.

---

content_length

INTEGER

Quantidade de caracteres.

---

extracted_at

DATETIME

Data da extração.

---

# Tabela: entities

Entidades identificadas pela IA.

## Campos

id

UUID

---

name

TEXT

Nome da entidade.

---

type

TEXT

Tipo da entidade.

Exemplos:

- person
- organization
- project
- topic
- document

---

confidence

REAL

Nível de confiança.

---

created_at

DATETIME

---

# Tabela: file_entities

Relacionamento entre arquivos e entidades.

## Campos

id

UUID

---

file_id

FK → files

---

entity_id

FK → entities

---

relationship_type

TEXT

Exemplos:

- mentions
- references
- belongs_to

---

confidence

REAL

---

# Tabela: relationships

Relacionamentos entre entidades.

## Campos

id

UUID

---

source_entity_id

FK → entities

---

target_entity_id

FK → entities

---

relationship_type

TEXT

Exemplos:

- related_to
- parent_of
- derived_from

---

confidence

REAL

---

created_at

DATETIME

---

# Tabela: embeddings

Representações vetoriais.

## Campos

id

UUID

---

file_id

FK → files

---

model

TEXT

Modelo utilizado.

Exemplo:

qwen-embedding

---

vector

BLOB

Representação vetorial serializada.

---

created_at

DATETIME

---

# Tabela: clusters

Agrupamentos semânticos.

## Campos

id

UUID

---

name

TEXT

Nome sugerido.

---

description

TEXT

Descrição gerada.

---

confidence

REAL

---

created_at

DATETIME

---

# Tabela: cluster_members

Participantes de um cluster.

## Campos

id

UUID

---

cluster_id

FK → clusters

---

file_id

FK → files

Pode ser nulo.

---

entity_id

FK → entities

Pode ser nulo.

---

confidence

REAL

---

# Tabela: suggestions

Sugestões geradas pelo sistema.

## Campos

id

UUID

---

type

TEXT

Exemplos:

- move_file
- rename_file
- create_folder
- merge_cluster

---

title

TEXT

Resumo da sugestão.

---

reason

TEXT

Justificativa.

---

confidence

REAL

---

status

TEXT

Valores:

- pending
- approved
- rejected
- executed

---

created_at

DATETIME

---

# Tabela: suggestion_operations

Operações vinculadas a uma sugestão.

## Campos

id

UUID

---

suggestion_id

FK → suggestions

---

operation_type

TEXT

---

payload

JSON

Detalhes da operação.

---

# Tabela: snapshots

Estado salvo antes de alterações.

## Campos

id

UUID

---

created_at

DATETIME

---

description

TEXT

---

# Tabela: operations

Histórico de alterações.

## Campos

id

UUID

---

snapshot_id

FK → snapshots

---

operation_type

TEXT

---

source_path

TEXT

---

target_path

TEXT

---

executed_at

DATETIME

---

# Índices Recomendados

files(hash)

files(path)

entities(name)

file_entities(file_id)

file_entities(entity_id)

relationships(source_entity_id)

relationships(target_entity_id)

clusters(name)

suggestions(status)

---

# Construção do Grafo

O Grafo de Conhecimento não é armazenado diretamente.

Ele é reconstruído utilizando:

files

- entities
- file_entities
- relationships
- clusters

  ***

# Fluxo de Dados

Arquivo
↓
files
↓
file_contents
↓
entities
↓
relationships
↓
embeddings
↓
clusters
↓
suggestions

---

# Evolução Futura

O modelo foi projetado para permitir futuras integrações com:

- Bancos vetoriais
- Bancos de grafos
- Busca semântica avançada
- Múltiplos modelos de IA

Sem necessidade de alterar o domínio principal do sistema.
