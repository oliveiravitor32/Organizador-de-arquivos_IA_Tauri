# Caso de Uso: Desfazer Alterações

## Identificação

**ID:** UC-007

**Nome:** Desfazer Alterações

**Categoria:** Execução

**Prioridade:** Crítica

---

# Objetivo

Restaurar o estado anterior do sistema de arquivos a partir de um snapshot previamente criado.

O objetivo é permitir que o usuário reverta total ou parcialmente alterações aplicadas pelo sistema.

---

# Atores

## Primário

Usuário

## Secundários

- Serviço de Snapshot
- Serviço de Execução
- Sistema de Arquivos
- Banco de Dados
- Grafo de Conhecimento

---

# Pré-condições

- Existe pelo menos uma execução concluída.
- Existe um snapshot válido associado à execução.
- Os arquivos continuam acessíveis pelo sistema.

---

# Pós-condições

## Sucesso

O estado do sistema retorna ao ponto anterior à execução selecionada.

---

## Falha

Nenhuma modificação parcial deve permanecer sem rastreamento.

O sistema registra a falha e preserva o estado atual.

---

# Objetivo de Negócio

Garantir segurança e confiança durante o uso do sistema.

O usuário deve poder experimentar sugestões sem medo de perder sua organização anterior.

---

# Filosofia

## Reversibilidade Obrigatória

Toda alteração executada pelo sistema deve poder ser revertida.

---

## Segurança Acima da Automação

A confiança do usuário é mais importante do que a quantidade de alterações realizadas.

---

## Histórico Permanente

As execuções devem permanecer registradas mesmo após a reversão.

---

# Fluxo Principal

## Passo 1

O usuário acessa o histórico de execuções.

---

## Passo 2

O sistema exibe as execuções disponíveis para restauração.

---

## Passo 3

O usuário seleciona uma execução.

---

## Passo 4

O sistema localiza o snapshot associado.

---

## Passo 5

O sistema valida a integridade do snapshot.

---

## Passo 6

O sistema gera um plano de restauração.

---

### Exemplo

```text
Restaurar:
- contrato.pdf
- cronograma.xlsx

Remover:
- Projeto Alpha/

Reverter:
- apresentação-final.pptx
→ apresentação.pptx
```

---

## Passo 7

O usuário confirma a restauração.

---

## Passo 8

O sistema inicia o processo de rollback.

---

## Passo 9

Os arquivos são restaurados.

---

## Passo 10

Os índices internos são atualizados.

---

## Passo 11

O Grafo de Conhecimento é sincronizado.

---

## Passo 12

O sistema registra a restauração.

---

## Passo 13

O usuário recebe confirmação da conclusão.

---

# Modos de Restauração

## Completa

Restaura toda a execução.

---

## Parcial

Restaura apenas operações selecionadas.

---

## Arquivo Específico

Restaura um único arquivo.

---

## Diretório Específico

Restaura um diretório completo.

---

# Fluxos Alternativos

## FA-001 — Snapshot Não Encontrado

### Condição

Snapshot removido ou corrompido.

### Resultado

Rollback indisponível.

---

## FA-002 — Arquivo Atualizado Manualmente

### Condição

Usuário alterou o arquivo após a execução.

### Resultado

Solicitar confirmação antes da restauração.

---

## FA-003 — Conflito de Caminho

### Condição

Já existe arquivo no destino da restauração.

### Resultado

Solicitar resolução.

---

## FA-004 — Falha Durante Rollback

### Condição

Erro inesperado.

### Resultado

Interromper restauração e registrar evento.

---

## FA-005 — Cancelamento

### Condição

Usuário cancela a operação.

### Resultado

Rollback interrompido.

---

# Regras de Negócio

## RN-001

Toda execução deve possuir snapshot associado.

---

## RN-002

Somente execuções concluídas podem ser revertidas.

---

## RN-003

Toda restauração deve ser registrada.

---

## RN-004

O histórico não deve ser apagado após a reversão.

---

## RN-005

O sistema deve preservar a consistência dos dados.

---

## RN-006

O usuário deve confirmar a restauração antes da execução.

---

## RN-007

O Grafo de Conhecimento deve refletir o estado restaurado.

---

# Estrutura de Rollback

## Exemplo

```json
{
  "rollbackId": "rollback-001",
  "executionId": "exec-001",
  "snapshotId": "snapshot-123",
  "timestamp": "2026-06-12T19:00:00Z"
}
```

---

# Eventos Emitidos

## RollbackStarted

Restauração iniciada.

---

## SnapshotLoaded

Snapshot carregado.

---

## RollbackProgress

Atualização de progresso.

---

## RollbackCompleted

Restauração concluída.

---

## RollbackFailed

Falha durante restauração.

---

## RollbackCancelled

Restauração cancelada.

---

# Dados Consumidos

## Snapshot

Estado anterior do sistema.

---

## Histórico de Execução

Informações da execução original.

---

## Estrutura Atual

Estado atual dos arquivos.

---

# Dados Produzidos

## Histórico de Rollback

Registro da restauração.

---

## Logs de Auditoria

Detalhamento das operações realizadas.

---

## Atualização do Índice

Sincronização dos caminhos restaurados.

---

## Atualização do Grafo

Recomposição das relações afetadas.

---

# Integrações

## Serviço de Snapshot

Origem dos dados de restauração.

---

## Sistema de Arquivos

Execução das operações de rollback.

---

## Banco de Dados

Persistência dos registros.

---

## Grafo de Conhecimento

Sincronização após restauração.

---

# Critérios de Aceitação

## CA-001

O usuário consegue visualizar execuções anteriores.

---

## CA-002

O sistema consegue localizar snapshots válidos.

---

## CA-003

A restauração recupera corretamente os arquivos.

---

## CA-004

Os índices internos são atualizados.

---

## CA-005

O Grafo de Conhecimento permanece consistente.

---

## CA-006

Todas as operações são auditáveis.

---

## CA-007

O usuário recebe feedback de progresso durante a restauração.

---

# Dependências

## Pré-requisitos

- UC-006 Aplicar Alterações

## Casos Relacionados

- Gerar Sugestões
- Aplicar Alterações

---

# Observações Arquiteturais

A capacidade de desfazer alterações é um requisito fundamental do sistema.

Sem rollback, o usuário não possui garantias suficientes para confiar nas recomendações produzidas pela IA.

O mecanismo de restauração deve ser tratado como um componente de primeira classe da arquitetura e não como uma funcionalidade secundária.

---

# Fluxo Resumido

```text
Execução Concluída
↓
Snapshot
↓
Seleção pelo Usuário
↓
Validação
↓
Plano de Restauração
↓
Rollback
↓
Atualização do Índice
↓
Atualização do Grafo
↓
Estado Restaurado
```
