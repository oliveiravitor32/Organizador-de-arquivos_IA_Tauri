# Caso de Uso: Extrair Entidades

## Identificação

**ID:** UC-008

**Nome:** Extrair Entidades

**Categoria:** Conhecimento

**Prioridade:** Crítica

---

# Objetivo

Identificar, a partir do conteúdo textual de um arquivo, as entidades relevantes que representam o significado do documento.

Esta etapa é um subprocesso da Análise de Arquivos (UC-003) e produz a matéria-prima do Grafo de Conhecimento.

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

- Arquivo com conteúdo textual extraído.
- Modelo de IA disponível localmente.

Status esperado:

```text
PENDING_ANALYSIS
```

---

# Pós-condições

## Sucesso

O arquivo possui entidades identificadas e associadas.

As entidades existentes são reutilizadas quando equivalentes.

---

## Falha

Nenhuma entidade é persistida.

O subprocesso é registrado como falho sem interromper os demais.

---

# Fluxo Principal

## Passo 1

O sistema recebe o conteúdo textual do arquivo.

---

## Passo 2

O sistema submete o conteúdo ao modelo de IA para reconhecimento de entidades.

---

## Passo 3

O modelo retorna entidades candidatas.

Tipos esperados:

- person
- organization
- project
- topic
- document

---

## Passo 4

O sistema atribui nível de confiança a cada entidade.

---

## Passo 5

O sistema verifica se cada entidade já existe.

---

## Passo 6

Entidades equivalentes são reutilizadas.

Entidades novas são criadas.

---

## Passo 7

O sistema cria o vínculo entre arquivo e entidade.

Tipos de vínculo:

- mentions
- references
- belongs_to

---

## Passo 8

O sistema persiste entidades e vínculos.

---

# Fluxos Alternativos

## FA-001 — Entidade Ambígua

### Condição

Entidade identificada com baixa confiança.

### Ação

Registrar como entidade candidata.

### Resultado

Não criar vínculo definitivo.

---

## FA-002 — Conteúdo Insuficiente

### Condição

Texto curto ou irrelevante.

### Ação

Extrair apenas entidades de alta confiança.

---

## FA-003 — Falha do Modelo

### Condição

O modelo não responde.

### Resultado

Subprocesso falha; arquivo permanece pendente.

---

# Regras de Negócio

## RN-001

Toda entidade deve possuir tipo e confiança.

---

## RN-002

Entidades existentes devem ser reutilizadas quando possível.

---

## RN-003

Entidades de baixa confiança não geram vínculos definitivos.

---

## RN-004

A extração não modifica arquivos físicos.

---

## RN-005

O processamento deve ocorrer localmente por padrão.

---

# Eventos Emitidos

## EntityExtractionStarted

Extração iniciada.

---

## EntityIdentified

Entidade identificada.

---

## EntityReused

Entidade existente reutilizada.

---

## EntityExtractionCompleted

Extração concluída.

---

## EntityExtractionFailed

Falha na extração.

---

# Dados Consumidos

## Conteúdo Textual

Texto extraído na indexação.

---

## Conhecimento Existente

Entidades previamente identificadas.

---

# Dados Produzidos

## Entidades

Conhecimento identificado.

---

## Vínculos Arquivo-Entidade

Relações entre arquivo e entidades.

---

# Integrações

## Serviço de IA

Reconhecimento de entidades.

---

## Banco de Dados

Persistência em `entities` e `file_entities`.

---

# Critérios de Aceitação

## CA-001

Entidades são identificadas a partir do conteúdo.

---

## CA-002

Entidades possuem tipo e confiança.

---

## CA-003

Entidades equivalentes são reutilizadas.

---

## CA-004

Vínculos arquivo-entidade são criados.

---

## CA-005

Falhas individuais não interrompem o pipeline.

---

# Dependências

## Pré-requisitos

- UC-002 Indexar Arquivos

## Parte de

- UC-003 Analisar Arquivos

## Consumidores

- UC-010 Descobrir Relações
- UC-004 Construir Grafo

---

# Observações Arquiteturais

A extração de entidades é o primeiro ponto em que dado se torna conhecimento.

A qualidade das entidades determina a qualidade de todo o restante do pipeline semântico.

As entidades são tratadas como conhecimento probabilístico, nunca como verdade absoluta.

---

# Fluxo Resumido

```text
Conteúdo Textual
↓
Reconhecimento de Entidades
↓
Atribuição de Confiança
↓
Reuso ou Criação
↓
Vínculo Arquivo-Entidade
↓
Persistência
```
