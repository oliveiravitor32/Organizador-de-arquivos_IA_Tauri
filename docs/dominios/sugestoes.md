# Domínio: Motor de Sugestões

## Objetivo

Este documento define como o sistema transforma conhecimento armazenado no Grafo de Conhecimento em sugestões úteis para o usuário.

O motor de sugestões não possui como objetivo impor uma organização pré-definida.

Seu objetivo é identificar oportunidades de organização que emergem naturalmente dos dados.

---

# Filosofia

## Organização Emergente

O sistema não assume uma estrutura ideal de diretórios.

A organização deve surgir da compreensão do conteúdo existente.

---

## Conhecimento Antes da Ação

Nenhuma sugestão deve ser baseada apenas em:

- nome do arquivo
- extensão
- localização física

Essas informações podem ser utilizadas como sinais auxiliares.

A principal fonte de decisão deve ser o conhecimento presente no grafo.

---

## Assistente, Não Autoridade

O sistema sugere.

O usuário decide.

---

# Papel do Motor de Sugestões

O motor é responsável por:

- detectar oportunidades de organização
- identificar agrupamentos
- propor alterações
- justificar recomendações
- calcular confiança

O motor não executa alterações.

---

# Fontes de Conhecimento

As sugestões podem utilizar:

- entidades
- relacionamentos
- clusters
- embeddings
- contexto compartilhado
- histórico de decisões do usuário

---

# Pipeline de Sugestões

Grafo
↓
Detecção de Oportunidades
↓
Análise de Evidências
↓
Cálculo de Confiança
↓
Geração da Sugestão
↓
Revisão do Usuário

---

# Oportunidades de Organização

## Conceito

Uma oportunidade representa uma possível melhoria percebida pelo sistema.

Nem toda oportunidade gera uma sugestão.

---

# Exemplos

Documentos relacionados dispersos.

---

Arquivos com contexto semelhante separados em múltiplos diretórios.

---

Pastas contendo conteúdos semanticamente heterogêneos.

---

Estruturas redundantes.

---

Clusters fortemente conectados sem representação física.

---

# Tipos de Sugestão

## Agrupamento

Sugere aproximar arquivos relacionados.

---

## Separação

Sugere dividir conjuntos excessivamente heterogêneos.

---

## Consolidação

Sugere unir estruturas redundantes.

---

## Renomeação

Sugere nomes mais representativos.

---

## Criação de Diretórios

Sugere criar uma estrutura física baseada em conhecimento descoberto.

---

# Sugestões Baseadas em Contexto

## Objetivo

Descobrir relações que não são evidentes pela estrutura atual.

---

# Exemplo

Arquivo:

Contrato_2024.pdf

Diretório:

Documentos/Contratos

---

Arquivo:

Proposta_Comercial.pdf

Diretório:

Comercial

---

Arquivo:

Apresentacao_Final.pptx

Diretório:

Apresentacoes

---

O grafo identifica:

- mesmo cliente
- mesmo projeto
- mesmas entidades

---

Resultado:

Sugestão de agrupamento contextual.

---

# Sugestões Baseadas em Similaridade

## Objetivo

Identificar documentos semanticamente próximos.

---

# Fontes

- embeddings
- entidades
- relações

---

# Exemplo

Dois documentos possuem:

- terminologia semelhante
- contexto semelhante
- entidades semelhantes

Mesmo sem compartilhar diretórios.

---

# Sugestões Baseadas em Clusters

## Objetivo

Transformar agrupamentos semânticos em estruturas navegáveis.

---

# Exemplo

Cluster identificado:

Projeto Alpha

---

Documentos encontrados:

- contrato
- apresentação
- cronograma
- orçamento

---

Resultado:

Sugestão para representar fisicamente esse contexto.

---

# Evidências

Toda sugestão deve possuir evidências.

---

# Exemplos

Entidades compartilhadas.

---

Relações existentes.

---

Similaridade vetorial.

---

Participação em clusters.

---

Histórico de organização.

---

# Modelo de Confiança

Toda sugestão possui confiança.

---

# Componentes

## Similaridade

Peso associado à proximidade semântica.

---

## Relações

Peso associado às conexões existentes.

---

## Consistência

Peso associado à coerência do agrupamento.

---

## Histórico

Peso associado a decisões anteriores do usuário.

---

# Faixas

0.90 - 1.00

Alta confiança

---

0.75 - 0.89

Média confiança

---

0.50 - 0.74

Baixa confiança

---

Abaixo de 0.50

Não sugerir.

---

# Explicabilidade

Toda sugestão deve responder:

## O que?

Qual alteração está sendo sugerida.

---

## Por quê?

Quais evidências motivaram a sugestão.

---

## Com qual confiança?

Nível estimado de confiabilidade.

---

# Exemplo

Sugestão:

Agrupar 12 documentos relacionados ao Projeto Alpha.

---

Evidências:

- Compartilham 7 entidades.
- Similaridade média de 0.92.
- Pertencem ao mesmo cluster.
- Relacionamentos recorrentes.

---

Confiança:

0.94

---

# Aprendizado Baseado no Usuário

## Objetivo

Adaptar futuras sugestões ao comportamento do usuário.

---

# Exemplos

Sugestões frequentemente rejeitadas reduzem peso de determinados critérios.

---

Sugestões frequentemente aceitas reforçam padrões observados.

---

# Observação

O aprendizado não altera arquivos.

Ele apenas ajusta a geração de sugestões.

---

# Restrições

## Nenhuma Alteração Automática

O motor nunca modifica arquivos diretamente.

---

## Nenhuma Regra Obrigatória

Não existe estrutura ideal fixa.

---

## Nenhuma Categoria Pré-definida Obrigatória

O sistema não depende de taxonomias rígidas.

---

# Integração com o Grafo

O motor consulta:

- entidades
- relações
- clusters
- embeddings

---

O motor produz:

- sugestões
- justificativas
- confiança

---

# Fluxo Completo

Arquivos
↓
Indexação
↓
Conteúdo
↓
IA
↓
Entidades
↓
Embeddings
↓
Relações
↓
Clusters
↓
Grafo
↓
Motor de Sugestões
↓
Revisão do Usuário
↓
Aplicação das Alterações

---

# Resultado Esperado

Ao final do processo, o sistema deve ser capaz de identificar oportunidades de organização que não seriam facilmente percebidas por inspeção manual.

As sugestões devem emergir do conhecimento descoberto e não de regras rígidas previamente definidas.

O objetivo final é auxiliar o usuário a compreender e organizar seus arquivos com base em significado, contexto e relações reais existentes nos dados.
