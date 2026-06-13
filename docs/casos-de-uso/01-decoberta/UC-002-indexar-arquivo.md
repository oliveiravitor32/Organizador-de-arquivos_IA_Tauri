# Caso de Uso: Indexar Arquivos

## Identificação

**ID:** UC-002

**Nome:** Indexar Arquivos

**Categoria:** Indexação

**Prioridade:** Alta

---

# Objetivo

Transformar arquivos descobertos durante o escaneamento em registros indexados que possam ser utilizados pelas etapas posteriores de análise e construção do conhecimento.

---

# Atores

## Primário

Sistema

---

## Secundários

- Serviço de Indexação
- Sistema de Arquivos
- Banco de Dados

---

# Pré-condições

- UC-001 concluído com sucesso.
- Existem arquivos com status:

```text
DISCOVERED
```

- Banco de dados disponível.

---

# Pós-condições

## Sucesso

Os arquivos recebem status:

```text
INDEXED
```

Metadados e conteúdo extraído ficam disponíveis para futuras análises.

---

## Falha

Arquivos com erro recebem status:

```text
FAILED
```

Os demais continuam sendo processados normalmente.

---

# Fluxo Principal

## Passo 1

O Serviço de Indexação consulta arquivos pendentes.

Critério:

```text
status = DISCOVERED
```

---

## Passo 2

O sistema inicia o processamento de cada arquivo.

---

## Passo 3

O sistema coleta metadados atualizados.

---

## Dados Coletados

- tamanho
- hash
- data de criação
- data de modificação
- tipo MIME
- extensão

---

## Passo 4

O sistema identifica o tipo de arquivo.

---

## Passo 5

O sistema seleciona o extrator apropriado.

Exemplos:

```text
TXT
PDF
DOCX
XLSX
MD
PNG
JPG
```

---

## Passo 6

O conteúdo é extraído quando suportado.

---

## Passo 7

O conteúdo bruto é normalizado.

---

## Operações

- normalização de espaços
- remoção de caracteres inválidos
- padronização de quebras de linha

---

## Passo 8

O sistema registra:

- metadados
- conteúdo extraído
- informações de indexação

---

## Passo 9

O status do arquivo é atualizado.

Novo estado:

```text
INDEXED
```

---

## Passo 10

O arquivo é enviado para a fila de análise semântica.

Novo estado:

```text
PENDING_ANALYSIS
```

---

## Passo 11

O progresso é atualizado.

---

## Passo 12

Ao final da execução o sistema apresenta estatísticas da indexação.

---

# Fluxos Alternativos

## FA-001 — Formato Não Suportado

### Condição

O sistema não possui extrator para o arquivo.

### Ação

Indexar apenas metadados.

### Resultado

Arquivo continua elegível para futuras extensões.

---

## FA-002 — Arquivo Corrompido

### Condição

Falha na leitura.

### Ação

Registrar erro.

### Resultado

Status:

```text
FAILED
```

---

## FA-003 — Arquivo Vazio

### Condição

Nenhum conteúdo encontrado.

### Ação

Registrar apenas metadados.

### Resultado

Arquivo permanece indexado.

---

## FA-004 — Arquivo Removido

### Condição

O arquivo foi removido após o escaneamento.

### Ação

Registrar inconsistência.

### Resultado

Arquivo marcado como ausente.

---

## FA-005 — Cancelamento

### Condição

Usuário interrompe a operação.

### Resultado

Arquivos já processados permanecem indexados.

Arquivos pendentes continuam em estado:

```text
DISCOVERED
```

---

# Regras de Negócio

## RN-001

Todo arquivo deve possuir hash.

---

## RN-002

Nenhum arquivo deve ser analisado semanticamente nesta etapa.

---

## RN-003

Nenhum embedding deve ser gerado.

---

## RN-004

Nenhuma entidade deve ser criada.

---

## RN-005

Nenhum relacionamento deve ser inferido.

---

## RN-006

A indexação deve ser idempotente.

Reexecutar o processo não deve criar duplicações.

---

## RN-007

A extração de conteúdo deve ser desacoplada da IA.

---

## RN-008

Arquivos não suportados continuam participando do sistema através dos metadados.

---

# Formatos Inicialmente Suportados

## Texto

- txt
- md
- json
- yaml
- xml

---

## Documentos

- pdf
- docx

---

## Planilhas

- xlsx

---

## Código Fonte

- ts
- tsx
- js
- jsx
- rs
- py
- java
- cs
- cpp
- c

---

## Imagens

- png
- jpg
- jpeg

OCR é opcional nesta etapa.

---

# Dados Produzidos

## Registro do Arquivo

- id
- caminho
- hash
- tamanho
- datas

---

## Conteúdo

- texto extraído
- idioma
- tamanho do conteúdo

---

## Estatísticas

- arquivos processados
- arquivos ignorados
- arquivos com falha
- tempo total

---

# Eventos Emitidos

## IndexingStarted

Início da indexação.

---

## FileIndexingStarted

Processamento de arquivo iniciado.

---

## ContentExtracted

Conteúdo extraído.

---

## FileIndexed

Arquivo indexado com sucesso.

---

## IndexingProgress

Atualização de progresso.

---

## IndexingCompleted

Processo concluído.

---

## IndexingFailed

Falha crítica.

---

## IndexingCancelled

Processo interrompido.

---

# Integrações

## Sistema de Arquivos

Leitura física dos arquivos.

---

## Banco de Dados

Persistência dos resultados.

---

## Fila de Análise

Registro dos arquivos que deverão ser analisados pela IA.

---

# Critérios de Aceitação

CA-001

Metadados são persistidos corretamente.

---

CA-002

Conteúdo é extraído quando suportado.

---

CA-003

Arquivos não suportados continuam indexados.

---

CA-004

Falhas individuais não interrompem a execução.

---

CA-005

Arquivos indexados ficam disponíveis para análise posterior.

---

CA-006

O sistema registra estatísticas completas da operação.

---

# Observações Arquiteturais

Esta etapa representa a fronteira entre:

```text
Sistema de Arquivos
↓
Conhecimento Digitalizado
```

Ao final do UC-002, o sistema já conhece:

- quais arquivos existem
- onde estão
- seus metadados
- seu conteúdo textual

Mas ainda não compreende o significado dessas informações.

A compreensão começa apenas no UC-003.
