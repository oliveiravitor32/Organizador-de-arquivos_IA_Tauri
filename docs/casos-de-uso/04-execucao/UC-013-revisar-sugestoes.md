# Caso de Uso: Revisar Sugestões

## Identificação

**ID:** UC-013

**Nome:** Revisar Sugestões

**Categoria:** Execução

**Prioridade:** Crítica

---

# Objetivo

Permitir que o usuário avalie as sugestões geradas pelo sistema e decida, de forma explícita, quais serão aprovadas, rejeitadas ou ajustadas antes de qualquer alteração física.

Esta etapa concretiza o princípio de Aprovação Obrigatória definido no ADR-007.

---

# Atores

## Primário

Usuário

---

## Secundários

- Motor de Sugestões
- Banco de Dados
- Serviço de Explicação

---

# Pré-condições

- Existe ao menos uma sugestão pendente (UC-005).
- As explicações estão disponíveis (UC-012).

Status esperado das sugestões:

```text
pending
```

---

# Pós-condições

## Sucesso

Cada sugestão revisada possui um novo estado:

- approved
- rejected

Nenhuma alteração física ocorre nesta etapa.

---

## Falha

As sugestões permanecem pendentes.

---

# Objetivo de Negócio

Manter o usuário no controle absoluto da organização.

O sistema recomenda; o usuário decide.

---

# Filosofia

## Usuário no Controle

Nenhuma alteração ocorre sem aprovação explícita.

---

## Decisão Informada

A revisão deve ser apoiada por explicações claras.

---

## Reversibilidade da Decisão

Aprovações e rejeições podem ser revistas enquanto não executadas.

---

# Fluxo Principal

## Passo 1

O usuário acessa a lista de sugestões pendentes.

---

## Passo 2

O sistema apresenta cada sugestão com sua explicação e confiança.

---

## Passo 3

O usuário analisa a sugestão.

---

## Passo 4

O usuário escolhe uma ação.

Ações possíveis:

- aprovar
- rejeitar
- ajustar
- adiar

---

## Passo 5

O sistema registra a decisão.

---

## Passo 6

O sistema atualiza o estado da sugestão.

---

## Passo 7

As sugestões aprovadas ficam disponíveis para execução (UC-006).

---

# Tipos de Decisão

## Aprovar

A sugestão será executada.

---

## Rejeitar

A sugestão é descartada.

---

## Ajustar

O usuário modifica a sugestão antes de aprovar.

Exemplo:

- alterar o nome de pasta proposto
- remover um arquivo do agrupamento

---

## Adiar

A sugestão permanece pendente para decisão futura.

---

# Fluxos Alternativos

## FA-001 — Sugestão Conflitante

### Condição

Duas sugestões aprovadas afetam o mesmo arquivo de forma incompatível.

### Ação

Sinalizar conflito e solicitar resolução.

---

## FA-002 — Conhecimento Alterado

### Condição

O conhecimento que originou a sugestão mudou.

### Ação

Marcar a sugestão como desatualizada.

### Resultado

Recomendar regeneração.

---

## FA-003 — Aprovação em Lote

### Condição

O usuário aprova múltiplas sugestões de uma vez.

### Ação

Registrar cada decisão individualmente.

---

## FA-004 — Reversão de Decisão

### Condição

O usuário muda uma decisão ainda não executada.

### Ação

Atualizar o estado da sugestão.

---

# Regras de Negócio

## RN-001

Nenhuma alteração física ocorre durante a revisão.

---

## RN-002

Somente sugestões aprovadas podem ser executadas.

---

## RN-003

Toda decisão deve ser registrada.

---

## RN-004

Uma decisão pode ser revista enquanto não executada.

---

## RN-005

Sugestões ajustadas mantêm rastreabilidade da sugestão original.

---

## RN-006

Conflitos devem ser resolvidos antes da execução.

---

# Eventos Emitidos

## ReviewStarted

Revisão iniciada.

---

## SuggestionApproved

Sugestão aprovada.

---

## SuggestionRejected

Sugestão rejeitada.

---

## SuggestionAdjusted

Sugestão ajustada.

---

## ReviewCompleted

Revisão concluída.

---

# Dados Consumidos

## Sugestões

Recomendações pendentes.

---

## Explicações

Justificativas e evidências.

---

# Dados Produzidos

## Decisões

Aprovações, rejeições e ajustes.

---

## Histórico de Decisões

Registro consultado por futuras sugestões.

---

# Integrações

## Motor de Sugestões

Origem das recomendações.

---

## Serviço de Explicação

Apoio à decisão.

---

## Banco de Dados

Persistência do estado das sugestões.

---

# Critérios de Aceitação

## CA-001

O usuário visualiza sugestões pendentes com explicação.

---

## CA-002

O usuário pode aprovar, rejeitar, ajustar e adiar.

---

## CA-003

Toda decisão é registrada.

---

## CA-004

Decisões podem ser revistas antes da execução.

---

## CA-005

Apenas sugestões aprovadas seguem para execução.

---

## CA-006

Conflitos são sinalizados.

---

# Dependências

## Pré-requisitos

- UC-005 Gerar Sugestões
- UC-012 Explicar Sugestões

## Próximo passo

- UC-006 Aplicar Alterações

---

# Observações Arquiteturais

A revisão é o ponto de decisão humana do sistema e o último portão antes de qualquer alteração física.

O histórico de decisões alimenta o aprendizado do Motor de Sugestões (ADR-009), tornando o sistema progressivamente mais alinhado às preferências do usuário.

Esta etapa não modifica arquivos; ela apenas converte recomendações em intenções aprovadas.

---

# Fluxo Resumido

```text
Sugestões Pendentes
↓
Apresentação com Explicação
↓
Decisão do Usuário
↓
Registro da Decisão
↓
Sugestões Aprovadas
↓
Execução
```
