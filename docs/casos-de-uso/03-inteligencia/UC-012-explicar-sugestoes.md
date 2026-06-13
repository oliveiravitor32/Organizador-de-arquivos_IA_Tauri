# Caso de Uso: Explicar Sugestões

## Identificação

**ID:** UC-012

**Nome:** Explicar Sugestões

**Categoria:** Inteligência

**Prioridade:** Alta

---

# Objetivo

Produzir, para cada sugestão gerada, uma explicação compreensível que apresente justificativa, evidências e confiança.

Esta etapa garante o princípio de Explicabilidade Obrigatória definido no ADR-009.

---

# Atores

## Primário

Usuário

---

## Secundários

- Motor de Sugestões
- Grafo de Conhecimento
- Banco de Dados

---

# Pré-condições

- Existe ao menos uma sugestão gerada (UC-005).
- O conhecimento de origem está disponível no grafo.

---

# Pós-condições

## Sucesso

Cada sugestão possui uma explicação consultável pelo usuário.

---

## Falha

A sugestão permanece, mas é marcada como sem explicação completa.

---

# Objetivo de Negócio

Permitir que o usuário confie nas recomendações por compreendê-las, e não por aceitá-las cegamente.

A confiança nasce da transparência.

---

# Filosofia

## Transparência Total

Nenhuma sugestão deve ser uma caixa-preta.

---

## Evidência Acima de Autoridade

A recomendação se justifica pelos dados, não pela autoridade do sistema.

---

# Fluxo Principal

## Passo 1

O sistema seleciona uma sugestão.

---

## Passo 2

O sistema recupera o conhecimento que originou a sugestão.

Fontes:

- entidades
- relações
- clusters
- contextos
- histórico de decisões

---

## Passo 3

O sistema reúne as evidências relevantes.

---

## Passo 4

O sistema compõe uma justificativa textual.

---

## Passo 5

O sistema apresenta a confiança associada.

---

## Passo 6

O sistema disponibiliza a explicação ao usuário.

---

## Exemplo

```text
Sugestão:
Agrupar documentos relacionados ao Projeto Alpha.

Justificativa:
3 arquivos mencionam a entidade "Projeto Alpha"
e compartilham o cliente "XPTO".

Evidências:
- contrato.pdf menciona Projeto Alpha
- cronograma.xlsx menciona Projeto Alpha
- alpha-final.pptx menciona Projeto Alpha

Confiança:
0.94
```

---

# Fluxos Alternativos

## FA-001 — Evidência Insuficiente

### Condição

Não há evidências suficientes para justificar.

### Ação

Apresentar explicação parcial e reduzir a confiança exibida.

---

## FA-002 — Conhecimento Removido

### Condição

Entidade ou relação de origem não existe mais.

### Ação

Sinalizar explicação desatualizada.

---

# Regras de Negócio

## RN-001

Toda sugestão deve possuir justificativa.

---

## RN-002

Toda sugestão deve apresentar evidências rastreáveis.

---

## RN-003

Toda sugestão deve exibir confiança.

---

## RN-004

A explicação deve ser derivada do Grafo de Conhecimento.

---

## RN-005

A explicação não deve inventar evidências inexistentes.

---

# Eventos Emitidos

## ExplanationRequested

Explicação solicitada.

---

## ExplanationGenerated

Explicação produzida.

---

## ExplanationIncomplete

Explicação parcial gerada.

---

# Dados Consumidos

## Sugestão

Recomendação a ser explicada.

---

## Grafo de Conhecimento

Origem das evidências.

---

# Dados Produzidos

## Explicação

Justificativa, evidências e confiança.

---

# Integrações

## Motor de Sugestões

Origem das sugestões.

---

## Grafo de Conhecimento

Origem das evidências.

---

# Critérios de Aceitação

## CA-001

Cada sugestão possui justificativa.

---

## CA-002

As evidências são rastreáveis até o grafo.

---

## CA-003

A confiança é exibida.

---

## CA-004

Explicações desatualizadas são sinalizadas.

---

# Dependências

## Pré-requisitos

- UC-005 Gerar Sugestões

## Consumidores

- UC-013 Revisar Sugestões

---

# Observações Arquiteturais

A explicabilidade é um pilar do ADR-009 e a base da relação de confiança entre usuário e sistema.

A explicação não é um recurso cosmético: é a ponte entre o conhecimento interno do sistema e a decisão humana.

---

# Fluxo Resumido

```text
Sugestão
↓
Recuperação do Conhecimento
↓
Reunião de Evidências
↓
Justificativa e Confiança
↓
Explicação ao Usuário
```
