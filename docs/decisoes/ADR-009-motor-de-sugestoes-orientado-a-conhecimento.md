# ADR-009 — Motor de Sugestões Orientado a Conhecimento

## Status

Aceito

---

## Data

2026-06-12

---

## Contexto

O objetivo principal do sistema é auxiliar usuários a organizar arquivos com base no significado e contexto das informações contidas neles.

Abordagens tradicionais de organização normalmente dependem de:

- regras fixas
- extensões de arquivo
- nomes de arquivos
- localização física
- taxonomias pré-definidas

Exemplos:

- mover arquivos `.jpg` para uma pasta "Imagens"
- mover arquivos `.pdf` para uma pasta "Documentos"
- aplicar regras configuradas manualmente pelo usuário

Embora simples, essas abordagens possuem limitações importantes:

- não compreendem conteúdo
- não identificam contexto
- não descobrem relações implícitas
- exigem configuração manual
- produzem organizações artificiais

O objetivo deste projeto é permitir que a estrutura organizacional emerja a partir do conhecimento descoberto nos arquivos.

---

## Problema

Como gerar sugestões de organização sem depender de regras rígidas previamente definidas?

O sistema precisa ser capaz de:

- compreender conteúdo
- identificar contexto
- descobrir relações
- detectar agrupamentos naturais
- justificar suas recomendações

sem depender de classificações pré-estabelecidas.

---

## Decisão

O Motor de Sugestões será orientado por conhecimento derivado do Grafo de Conhecimento.

As sugestões não serão geradas a partir de regras fixas.

As sugestões serão produzidas a partir da análise de:

- entidades
- relações
- embeddings
- clusters
- contextos
- histórico de decisões

armazenados no Grafo de Conhecimento.

---

## Princípios

### Conhecimento Antes da Estrutura

O sistema deve compreender o significado antes de propor alterações estruturais.

A estrutura física é consequência do conhecimento.

---

### Organização Emergente

Não existe uma estrutura ideal universal.

A organização deve emergir dos padrões encontrados nos dados.

---

### Contexto Acima de Extensão

A extensão do arquivo é apenas um atributo técnico.

Ela não deve ser utilizada como principal fonte de decisão.

---

### Explicabilidade Obrigatória

Toda sugestão deve possuir:

- justificativa
- evidências
- confiança

---

### Assistência em vez de Automação

O sistema recomenda.

O usuário decide.

Nenhuma alteração deve ocorrer automaticamente.

---

## Arquitetura Resultante

Fluxo de alto nível:

```text
Arquivos
↓
Indexação
↓
Análise Semântica
↓
Entidades
↓
Embeddings
↓
Relações
↓
Clusters
↓
Grafo de Conhecimento
↓
Motor de Sugestões
↓
Revisão do Usuário
```

---

## Fontes Utilizadas pelo Motor

### Entidades

Exemplos:

- pessoas
- organizações
- projetos
- conceitos

---

### Relações

Exemplos:

- menciona
- relacionado_com
- pertence_a
- referencia

---

### Embeddings

Utilizados para medir proximidade semântica.

---

### Clusters

Representam agrupamentos naturais identificados pelo sistema.

---

### Contextos

Representam conjuntos coerentes de conhecimento.

---

### Histórico

Representa decisões anteriores do usuário.

---

## Exemplo

Estrutura física atual:

```text
Comercial/
  proposta.pdf

Projetos/
  cronograma.xlsx

Apresentacoes/
  alpha-final.pptx
```

Análise semântica identifica:

- Projeto Alpha
- Cliente XPTO
- Mesmo contexto
- Mesmas entidades

O sistema cria um cluster contextual.

O Motor de Sugestões gera:

```text
Sugestão:
Agrupar documentos relacionados ao Projeto Alpha.

Confiança:
0.94
```

A sugestão é baseada em conhecimento e não na localização atual dos arquivos.

---

## Consequências Positivas

### Maior Flexibilidade

O sistema não depende de estruturas pré-definidas.

---

### Descoberta de Contextos Ocultos

Permite identificar relações que não são evidentes pela organização física.

---

### Menor Dependência de Configuração

O usuário não precisa criar regras complexas.

---

### Evolução Contínua

O sistema melhora conforme acumula conhecimento.

---

### Melhor Explicabilidade

As recomendações podem ser justificadas através do grafo.

---

## Consequências Negativas

### Maior Complexidade

A geração de sugestões torna-se mais sofisticada.

---

### Dependência da Qualidade da Análise

Sugestões dependem da qualidade das inferências realizadas.

---

### Necessidade de Processamento Semântico

O sistema exige análise de conteúdo.

---

### Possibilidade de Falsos Positivos

Inferências incorretas podem gerar sugestões inadequadas.

---

## Alternativas Consideradas

### Regras Baseadas em Extensão

Exemplo:

```text
.pdf → Documentos
.jpg → Imagens
```

#### Motivo da Rejeição

Não compreende contexto.

---

### Regras Configuráveis pelo Usuário

Exemplo:

```text
Se nome contém "Projeto"
Mover para Projetos
```

#### Motivo da Rejeição

Exige manutenção manual.

---

### Organização Puramente Estatística

Baseada apenas em frequência e similaridade.

#### Motivo da Rejeição

Produz agrupamentos pouco explicáveis.

---

## Impacto nos Casos de Uso

Afeta diretamente:

- UC-003 Analisar Arquivos
- UC-004 Construir Grafo
- UC-005 Gerar Sugestões
- UC-006 Revisar Sugestões

---

## Impacto no Banco de Dados

Necessidade de armazenar:

- entidades
- relações
- embeddings
- clusters
- evidências
- confiança

---

## Impacto na IA

A IA não é responsável por organizar arquivos.

A IA é responsável por produzir conhecimento.

A organização é derivada desse conhecimento.

---

## Decisão Final

O sistema adotará um Motor de Sugestões Orientado a Conhecimento.

As recomendações serão geradas a partir do Grafo de Conhecimento e não de regras rígidas previamente definidas.

Esta decisão é considerada um dos pilares arquiteturais centrais do projeto.
