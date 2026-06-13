# Caso de Uso: Aplicar Alterações

## Identificação

**ID:** UC-006

**Nome:** Aplicar Alterações

**Categoria:** Execução

**Prioridade:** Crítica

---

# Objetivo

Executar no sistema de arquivos as alterações aprovadas pelo usuário durante a revisão das sugestões.

Esta é a primeira etapa do sistema que modifica efetivamente arquivos e diretórios físicos.

---

# Atores

## Primário

Usuário

## Secundários

- Serviço de Execução
- Sistema de Arquivos
- Banco de Dados
- Serviço de Snapshot
- Grafo de Conhecimento

---

# Pré-condições

- Existe pelo menos uma sugestão aprovada.
- O usuário confirmou a execução.
- Um snapshot foi criado com sucesso.
- Os caminhos envolvidos continuam acessíveis.

---

# Pós-condições

## Sucesso

As alterações aprovadas foram aplicadas.

O sistema atualiza:

- estrutura física
- índice de arquivos
- conhecimento associado
- histórico de execução

---

## Falha

Nenhuma alteração parcial deve permanecer sem rastreamento.

O sistema deve permitir recuperação através do snapshot.

---

# Objetivo de Negócio

Transformar sugestões aprovadas em alterações reais na organização dos arquivos.

Garantir segurança, rastreabilidade e reversibilidade durante todo o processo.

---

# Filosofia

## Usuário no Controle

O sistema nunca executa alterações sem aprovação explícita.

---

## Segurança Antes da Velocidade

Toda operação deve ser reversível.

---

## Conhecimento Preservado

A reorganização física não deve causar perda de conhecimento armazenado.

---

# Fluxo Principal

## Passo 1

O usuário seleciona sugestões aprovadas.

---

## Passo 2

O sistema valida todas as operações pendentes.

---

## Passo 3

O sistema verifica:

- existência dos arquivos
- permissões necessárias
- conflitos de nomes
- caminhos de destino

---

## Passo 4

O sistema cria um snapshot da operação.

---

## Passo 5

O sistema gera um plano de execução.

---

## Exemplo

```text
1. Criar diretório Projeto Alpha
2. Mover contrato.pdf
3. Mover cronograma.xlsx
4. Renomear apresentacao.pptx
```

---

## Passo 6

O sistema executa as operações.

---

## Passo 7

O progresso é atualizado em tempo real.

---

## Passo 8

Ao final da execução:

- índices são atualizados
- caminhos são sincronizados
- referências são corrigidas

---

## Passo 9

O Grafo de Conhecimento é atualizado.

---

## Passo 10

O histórico da execução é persistido.

---

## Passo 11

O sistema informa o resultado ao usuário.

---

# Tipos de Operação

## Criar Diretório

Criação de novas estruturas físicas.

---

## Mover Arquivo

Alteração de localização.

---

## Mover Diretório

Alteração de localização de estruturas completas.

---

## Renomear Arquivo

Alteração de nome mantendo identidade.

---

## Renomear Diretório

Alteração estrutural.

---

## Consolidar Estruturas

União de diretórios relacionados.

---

# Fluxos Alternativos

## FA-001 — Arquivo Não Encontrado

### Condição

Arquivo removido após a geração da sugestão.

### Ação

Ignorar operação.

### Resultado

Registrar inconsistência.

---

## FA-002 — Conflito de Nome

### Condição

Já existe arquivo com mesmo nome no destino.

### Ação

Solicitar resolução.

### Resultado

Operação pausada.

---

## FA-003 — Permissão Negada

### Condição

Sistema não possui acesso.

### Resultado

Operação falha.

---

## FA-004 — Falha Durante Execução

### Condição

Erro inesperado.

### Ação

Interromper execução.

### Resultado

Snapshot disponível para restauração.

---

## FA-005 — Cancelamento

### Condição

Usuário cancela execução.

### Resultado

Operações não iniciadas são abortadas.

---

# Regras de Negócio

## RN-001

Toda execução exige snapshot prévio.

---

## RN-002

Nenhuma alteração pode ocorrer sem aprovação explícita.

---

## RN-003

Toda operação deve ser registrada.

---

## RN-004

Toda execução deve possuir identificador único.

---

## RN-005

O histórico deve permanecer disponível para auditoria.

---

## RN-006

O sistema deve preservar a integridade dos arquivos.

---

## RN-007

A execução não pode apagar conhecimento armazenado.

---

## RN-008

O sistema deve atualizar referências internas após alterações.

---

# Estrutura de Execução

## Exemplo

```json
{
  "executionId": "exec-001",
  "snapshotId": "snapshot-123",
  "operations": [
    {
      "type": "move",
      "from": "/Documentos/contrato.pdf",
      "to": "/Projeto Alpha/contrato.pdf"
    }
  ]
}
```

---

# Eventos Emitidos

## ExecutionStarted

Execução iniciada.

---

## SnapshotCreated

Snapshot concluído.

---

## OperationStarted

Operação iniciada.

---

## OperationCompleted

Operação concluída.

---

## ExecutionProgress

Atualização de progresso.

---

## ExecutionCompleted

Execução finalizada.

---

## ExecutionFailed

Falha crítica.

---

## ExecutionCancelled

Execução cancelada.

---

# Dados Consumidos

## Sugestões Aprovadas

Lista de alterações aprovadas.

---

## Snapshot

Estado anterior do sistema.

---

## Estrutura Atual

Estado físico dos arquivos.

---

# Dados Produzidos

## Histórico de Execução

Registro completo da operação.

---

## Log de Alterações

Auditoria detalhada.

---

## Atualização do Índice

Novos caminhos.

---

## Atualização do Grafo

Sincronização do conhecimento.

---

# Integrações

## Sistema de Arquivos

Execução das operações.

---

## Banco de Dados

Persistência.

---

## Serviço de Snapshot

Proteção e recuperação.

---

## Grafo de Conhecimento

Atualização pós-execução.

---

# Critérios de Aceitação

## CA-001

Somente sugestões aprovadas podem ser executadas.

---

## CA-002

Snapshot é criado antes da execução.

---

## CA-003

Operações são registradas.

---

## CA-004

O progresso é exibido ao usuário.

---

## CA-005

O índice é atualizado após execução.

---

## CA-006

O Grafo de Conhecimento permanece consistente.

---

## CA-007

É possível restaurar o estado anterior através do snapshot.

---

# Dependências

## Pré-requisitos

- Escanear Diretório
- Indexar Arquivos
- Analisar Arquivos
- Construir Grafo
- Gerar Sugestões
- Revisar Sugestões

## Casos Relacionados

- Criar Snapshot
- Desfazer Alterações

---

# Observações Arquiteturais

Esta é a única etapa do fluxo principal que modifica o sistema de arquivos.

Por esse motivo, todas as operações devem ser:

- seguras
- auditáveis
- reversíveis

O conhecimento armazenado no sistema deve sobreviver às alterações físicas.

O Grafo de Conhecimento representa significado e não localização.

Mudanças na estrutura física não devem destruir contexto previamente aprendido.

---

# Fluxo Resumido

```text
Sugestões Aprovadas
↓
Validação
↓
Snapshot
↓
Plano de Execução
↓
Operações
↓
Atualização do Índice
↓
Atualização do Grafo
↓
Histórico
↓
Execução Concluída
```
