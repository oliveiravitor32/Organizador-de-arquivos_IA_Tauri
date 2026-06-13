# Caso de Uso: Gerar Sugestões

## Identificação

**ID:** UC-005

**Nome:** Gerar Sugestões

**Categoria:** Inteligência

**Prioridade:** Crítica

---

# Objetivo

Analisar o Grafo de Conhecimento e identificar oportunidades de organização, agrupamento e melhoria estrutural dos arquivos do usuário.

O sistema não deve impor uma organização específica.

Seu papel é propor alternativas fundamentadas no conhecimento descoberto.

---

# Atores

## Primário

Sistema

## Secundários

- Motor de Sugestões
- Grafo de Conhecimento
- Banco de Dados

---

# Pré-condições

- UC-001 concluído.
- UC-002 concluído.
- UC-003 concluído.
- UC-004 concluído.
- Existe um Grafo de Conhecimento atualizado.

---

# Pós-condições

## Sucesso

Sugestões são geradas e armazenadas para revisão.

## Falha

Nenhuma alteração é realizada nos arquivos.

---

# Objetivo de Negócio

Transformar conhecimento acumulado em recomendações práticas que auxiliem o usuário a:

- localizar informações
- reduzir dispersão
- reduzir redundância
- melhorar navegabilidade
- aproximar conteúdos relacionados

---

# Filosofia

## Organização Emergente

As sugestões devem surgir do conhecimento identificado.

Não devem depender exclusivamente de:

- nomes de arquivos
- extensões
- localização física

## Conhecimento Antes da Estrutura

O sistema organiza significado.

A estrutura física é apenas uma consequência.

## Assistência e Não Automação

Toda sugestão deve ser revisada pelo usuário.

Nenhuma alteração é aplicada automaticamente.

---

# Fluxo Principal

## Passo 1

O sistema inicia uma análise do Grafo de Conhecimento.

## Passo 2

O sistema identifica agrupamentos semânticos relevantes.

## Passo 3

O sistema identifica arquivos semanticamente relacionados.

## Passo 4

O sistema detecta estruturas físicas inconsistentes com o conhecimento descoberto.

## Passo 5

O sistema identifica oportunidades de reorganização.

## Passo 6

O sistema calcula evidências para cada oportunidade.

## Passo 7

O sistema calcula confiança.

## Passo 8

O sistema transforma oportunidades em sugestões concretas.

## Passo 9

O sistema gera justificativas explicáveis.

## Passo 10

As sugestões são persistidas.

## Passo 11

As sugestões ficam disponíveis para revisão.

---

# Tipos de Sugestão

## Agrupamento

### Objetivo

Aproximar conteúdos fortemente relacionados.

### Exemplo

Arquivos:

```text
Contrato_Alpha.pdf
Cronograma_Alpha.xlsx
Apresentacao_Alpha.pptx
```

Encontrados em diretórios diferentes.

Sugestão:

Representar fisicamente o contexto identificado.

---

## Consolidação

### Objetivo

Reduzir fragmentação desnecessária.

### Exemplo

Múltiplas estruturas contendo informações do mesmo contexto.

---

## Separação

### Objetivo

Dividir agrupamentos semanticamente heterogêneos.

### Exemplo

Uma pasta contendo documentos de vários projetos distintos.

---

## Renomeação

### Objetivo

Melhorar representatividade.

### Exemplo

```text
arquivo_final_v2_ok.pdf
```

↓

```text
Contrato_Projeto_Alpha.pdf
```

---

## Estruturação

### Objetivo

Criar representações físicas para contextos relevantes.

---

# Fontes de Evidência

## Entidades Compartilhadas

Arquivos relacionados às mesmas entidades.

## Relações do Grafo

Conexões explícitas e implícitas.

## Similaridade Semântica

Obtida por embeddings.

## Clusters

Participação em agrupamentos.

## Contexto Histórico

Conhecimento acumulado pelo sistema.

## Decisões Anteriores

Preferências observadas no comportamento do usuário.

---

# Modelo de Confiança

Toda sugestão deve possuir uma pontuação.

## Componentes

### Similaridade

Peso da proximidade semântica.

### Relações

Peso das conexões existentes.

### Contexto

Peso do contexto compartilhado.

### Coerência

Peso da consistência do agrupamento.

### Histórico

Peso derivado de decisões passadas.

---

## Faixas

### Alta Confiança

```text
0.90 – 1.00
```

### Média Confiança

```text
0.75 – 0.89
```

### Baixa Confiança

```text
0.50 – 0.74
```

### Muito Baixa

```text
< 0.50
```

Não sugerir.

---

# Estrutura da Sugestão

Toda sugestão deve conter:

```json
{
  "id": "suggestion-001",
  "tipo": "agrupamento",
  "titulo": "Agrupar documentos do Projeto Alpha",
  "descricao": "Foram encontrados arquivos relacionados ao mesmo contexto.",
  "confianca": 0.94,
  "evidencias": [
    "7 entidades compartilhadas",
    "similaridade média de 0.92",
    "mesmo cluster"
  ]
}
```

---

# Fluxos Alternativos

## FA-001 — Grafo Pequeno

### Condição

Quantidade insuficiente de conhecimento.

### Resultado

Nenhuma sugestão gerada.

---

## FA-002 — Baixa Confiança

### Condição

Não há evidências suficientes.

### Resultado

Sugestão descartada.

---

## FA-003 — Sugestão Duplicada

### Condição

Sugestão semelhante já existe.

### Resultado

Atualizar sugestão existente.

---

# Regras de Negócio

## RN-001

Toda sugestão deve possuir justificativa.

## RN-002

Toda sugestão deve possuir evidências.

## RN-003

Toda sugestão deve possuir confiança.

## RN-004

Nenhuma sugestão pode ser aplicada automaticamente.

## RN-005

O sistema deve privilegiar explicabilidade.

## RN-006

O sistema deve priorizar conhecimento em vez de estrutura física.

## RN-007

Sugestões rejeitadas devem ser registradas para aprendizado futuro.

---

# Eventos Emitidos

## SuggestionGenerationStarted

Processo iniciado.

## OpportunityDetected

Oportunidade identificada.

## SuggestionCreated

Sugestão criada.

## SuggestionDiscarded

Sugestão descartada.

## SuggestionGenerationCompleted

Processo concluído.

---

# Dados Consumidos

## Grafo

- nós
- relações
- clusters

## Embeddings

Representações vetoriais.

## Histórico

Decisões anteriores.

---

# Dados Produzidos

## Sugestões

Recomendações prontas para revisão.

## Justificativas

Explicações geradas.

## Evidências

Base factual das recomendações.

## Métricas

- quantidade de sugestões
- confiança média
- tipos de sugestão
- taxa de descarte

---

# Integrações

## Grafo de Conhecimento

Fonte principal.

## Banco de Dados

Persistência.

## Módulo de Revisão

Consumidor das sugestões.

---

# Critérios de Aceitação

## CA-001

O sistema gera sugestões baseadas em conhecimento.

## CA-002

Toda sugestão possui justificativa.

## CA-003

Toda sugestão possui confiança.

## CA-004

Sugestões podem ser revisadas pelo usuário.

## CA-005

Nenhuma alteração física ocorre nesta etapa.

---

# Dependências

## Pré-requisitos

- Escanear Diretório
- Indexar Arquivos
- Analisar Arquivos
- Construir Grafo

## Próximo Passo

- Revisar Sugestões

---

# Observações Arquiteturais

O Motor de Sugestões não organiza arquivos.

Ele identifica oportunidades.

A organização física é apenas uma possível materialização dessas oportunidades.

O verdadeiro valor desta etapa é transformar conhecimento em recomendações compreensíveis e justificáveis.

---

# Fluxo Resumido

```text
Arquivos
↓
Conhecimento
↓
Grafo
↓
Oportunidades
↓
Evidências
↓
Sugestões
↓
Revisão do Usuário
↓
Execução
```
