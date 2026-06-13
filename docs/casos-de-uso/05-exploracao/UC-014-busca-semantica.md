# Caso de Uso: Busca Semântica

## Identificação

**ID:** UC-014

**Nome:** Busca Semântica

**Categoria:** Exploração

**Prioridade:** Alta

---

# Objetivo

Permitir que o usuário encontre arquivos pelo significado do seu conteúdo, e não apenas por nome, extensão ou localização.

Esta etapa consome embeddings e o Grafo de Conhecimento para retornar resultados por proximidade semântica.

---

# Atores

## Primário

Usuário

---

## Secundários

- Serviço de Busca
- Grafo de Conhecimento
- Banco de Dados

---

# Pré-condições

- Existem arquivos analisados.
- Existem embeddings disponíveis (UC-009).

---

# Pós-condições

## Sucesso

O usuário recebe uma lista de resultados ordenada por relevância semântica.

---

## Falha

Nenhuma alteração ocorre.

O usuário é informado da ausência de resultados.

---

# Objetivo de Negócio

Tornar o conhecimento acumulado pelo sistema imediatamente útil para o usuário no dia a dia.

---

# Filosofia

## Significado Acima de Nome

A busca compreende intenção, não apenas texto literal.

---

## Exploração Sem Risco

A busca nunca modifica arquivos.

---

# Fluxo Principal

## Passo 1

O usuário informa um termo ou frase de busca.

---

## Passo 2

O sistema interpreta a intenção da consulta.

---

## Passo 3

O sistema gera um embedding da consulta.

---

## Passo 4

O sistema compara a consulta com os embeddings existentes.

---

## Passo 5

O sistema enriquece os resultados com conhecimento do grafo.

Exemplos:

- entidades relacionadas
- clusters de origem
- contextos associados

---

## Passo 6

O sistema ordena os resultados por relevância.

---

## Passo 7

O sistema apresenta os resultados ao usuário.

---

## Exemplo

```text
Consulta:
"contratos do cliente XPTO"

Resultados:
1. contrato-xpto.pdf        (0.92)
2. aditivo-xpto.docx        (0.88)
3. proposta-comercial.pdf   (0.81)
```

---

# Modos de Busca

## Por Conteúdo

Busca pelo significado do texto.

---

## Por Entidade

Busca por arquivos relacionados a uma entidade.

---

## Por Similaridade

Busca por arquivos semelhantes a um arquivo de referência.

---

# Fluxos Alternativos

## FA-001 — Sem Resultados

### Condição

Nenhum arquivo atinge o limiar de relevância.

### Resultado

Informar ausência de resultados.

---

## FA-002 — Embeddings Indisponíveis

### Condição

Arquivos ainda não analisados.

### Ação

Recomendar conclusão da análise.

---

## FA-003 — Consulta Ambígua

### Condição

A intenção não é clara.

### Ação

Apresentar resultados amplos e sugerir refinamento.

---

# Regras de Negócio

## RN-001

A busca não modifica arquivos físicos.

---

## RN-002

Os resultados devem ser ordenados por relevância semântica.

---

## RN-003

A busca deve considerar o conhecimento do grafo.

---

## RN-004

A busca deve operar sobre o estado atual dos arquivos.

---

## RN-005

O processamento deve ocorrer localmente por padrão.

---

# Eventos Emitidos

## SearchStarted

Busca iniciada.

---

## SearchCompleted

Busca concluída.

---

## SearchEmpty

Nenhum resultado encontrado.

---

# Dados Consumidos

## Consulta

Termo informado pelo usuário.

---

## Embeddings

Representações vetoriais.

---

## Grafo de Conhecimento

Enriquecimento dos resultados.

---

# Dados Produzidos

## Resultados

Lista ordenada de arquivos relevantes.

---

# Integrações

## Serviço de Busca

Comparação semântica.

---

## Grafo de Conhecimento

Enriquecimento contextual.

---

## Banco de Dados

Origem de embeddings e metadados.

---

# Critérios de Aceitação

## CA-001

O usuário busca por significado.

---

## CA-002

Os resultados são ordenados por relevância.

---

## CA-003

Os resultados são enriquecidos pelo grafo.

---

## CA-004

A ausência de resultados é comunicada.

---

## CA-005

A busca não altera arquivos.

---

# Dependências

## Pré-requisitos

- UC-009 Gerar Embeddings
- UC-004 Construir Grafo

## Casos Relacionados

- UC-015 Explorar Contexto

---

# Observações Arquiteturais

A busca semântica é a contrapartida de leitura do conhecimento que o sistema constrói.

Enquanto a organização reposiciona arquivos fisicamente, a busca permite encontrá-los pelo significado, independentemente de onde estejam.

É um consumidor de leitura pura: nunca altera o estado do sistema de arquivos.

---

# Fluxo Resumido

```text
Consulta
↓
Interpretação
↓
Embedding da Consulta
↓
Comparação Semântica
↓
Enriquecimento pelo Grafo
↓
Resultados Ordenados
```
