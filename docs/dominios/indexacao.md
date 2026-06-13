# Domínio: Indexação

## Objetivo

O processo de indexação é responsável por transformar arquivos físicos em conhecimento utilizável pelo sistema.

A indexação constitui a ponte entre o sistema de arquivos e o Grafo de Conhecimento.

Seu objetivo é coletar, extrair, estruturar e armazenar informações necessárias para análises futuras.

---

# Definição

Indexação é o processo de converter arquivos físicos em representações estruturadas que possam ser consultadas, analisadas e relacionadas.

O resultado final da indexação é um conjunto de informações persistidas localmente e prontas para enriquecimento semântico.

---

# Princípios

## Independência da IA

A indexação deve funcionar mesmo sem modelos de IA disponíveis.

A extração de conhecimento baseada em IA é uma etapa posterior.

---

## Reprocessamento Incremental

Arquivos já processados não devem ser reindexados desnecessariamente.

O sistema deve detectar alterações utilizando hash e metadados.

---

## Tolerância a Falhas

Falhas na indexação de um arquivo não devem interromper a indexação dos demais.

---

## Escalabilidade

O sistema deve ser capaz de indexar dezenas ou centenas de milhares de arquivos.

---

# Pipeline de Indexação

Sistema de Arquivos
↓
Descoberta de Arquivos
↓
Extração de Metadados
↓
Extração de Conteúdo
↓
Normalização
↓
Persistência
↓
Fila de Enriquecimento
↓
IA
↓
Grafo de Conhecimento

---

# Etapa 1: Descoberta de Arquivos

## Objetivo

Localizar arquivos dentro do diretório raiz selecionado pelo usuário.

## Entrada

Diretório raiz.

## Saída

Lista de arquivos encontrados.

## Responsabilidades

- Percorrer diretórios recursivamente.
- Identificar arquivos suportados.
- Registrar caminhos encontrados.
- Ignorar arquivos configurados pelo usuário.

## Dados Coletados

- caminho absoluto
- caminho relativo
- nome
- extensão

---

# Etapa 2: Extração de Metadados

## Objetivo

Coletar informações básicas sem analisar o conteúdo interno.

## Dados Extraídos

- tamanho
- hash
- data de criação
- data de modificação
- extensão
- tipo MIME

## Resultado

Criação ou atualização do registro do arquivo indexado.

---

# Etapa 3: Extração de Conteúdo

## Objetivo

Obter uma representação textual do conteúdo do arquivo.

O texto extraído será utilizado posteriormente para análise semântica.

---

# Formatos Suportados

## TXT

Extração direta.

Resultado:

Texto completo.

---

## PDF

Extração textual.

Resultado:

Texto extraído das páginas.

---

## DOCX

Extração textual.

Resultado:

Texto dos parágrafos.

---

## XLSX

Extração estrutural.

Resultado:

- nomes das planilhas
- cabeçalhos
- conteúdo textual relevante

---

## Markdown

Extração textual.

Resultado:

Conteúdo completo.

---

## Imagens

OCR opcional.

Resultado:

Texto identificado na imagem.

---

# Formatos Não Suportados

Arquivos sem suporte devem continuar sendo indexados.

Entretanto:

- não terão conteúdo extraído
- participarão apenas através de metadados

---

# Etapa 4: Normalização

## Objetivo

Preparar o conteúdo para análises posteriores.

---

# Operações

## Limpeza

Remover caracteres inválidos.

---

## Padronização

Normalizar espaços e quebras de linha.

---

## Limitação

Definir limites máximos para armazenamento.

---

## Detecção de Idioma

Identificar idioma predominante quando possível.

---

# Etapa 5: Persistência

## Objetivo

Armazenar os dados extraídos localmente.

---

# Dados Persistidos

Arquivo

- identificador
- caminho
- hash
- metadados

Conteúdo

- texto extraído
- idioma
- tamanho

Status

- indexado
- pendente de análise
- analisado

---

# Etapa 6: Fila de Enriquecimento

## Objetivo

Preparar arquivos para processamento por IA.

A indexação não depende desta etapa.

---

# Responsabilidades

- detectar arquivos pendentes
- organizar processamento
- controlar prioridades
- evitar reprocessamentos

---

# Enriquecimento Semântico

## Objetivo

Transformar conteúdo em conhecimento.

---

# Entrada

Conteúdo textual previamente indexado.

---

# Saída

- entidades
- embeddings
- relações
- contexto

---

# Processos

## Extração de Entidades

Exemplo:

Texto:

Contrato firmado entre Empresa XPTO e João Silva.

Resultado:

- Empresa XPTO
- João Silva
- Contrato

---

## Classificação de Contexto

Exemplo:

Texto:

Proposta comercial para fornecimento de serviços.

Resultado:

- Comercial
- Contratos
- Cliente

---

## Geração de Embeddings

Transformação do conteúdo em representação vetorial.

Utilizada para:

- busca semântica
- agrupamento
- descoberta de relações

---

# Atualização Incremental

O sistema deve identificar alterações em arquivos previamente indexados.

Um arquivo deve ser reprocessado quando:

- conteúdo mudar
- hash mudar
- arquivo for renomeado
- arquivo for movido

---

# Estados de Indexação

## Descoberto

Arquivo localizado.

---

## Indexando

Metadados sendo processados.

---

## Indexado

Dados básicos armazenados.

---

## Pendente de Análise

Aguardando IA.

---

## Analisando

Processamento semântico em andamento.

---

## Analisado

Conhecimento já incorporado ao grafo.

---

## Falha

Erro durante alguma etapa.

---

# Métricas

O sistema deve registrar:

- quantidade de arquivos indexados
- quantidade de arquivos analisados
- tempo médio de indexação
- tempo médio de análise
- quantidade de entidades extraídas
- quantidade de relações criadas

---

# Resultado Esperado

Ao final do processo de indexação, cada arquivo deve possuir:

- identidade única
- metadados
- conteúdo extraído (quando possível)
- histórico de processamento

E estar pronto para ser enriquecido semanticamente e incorporado ao Grafo de Conhecimento.
