# ADR-010 — Estratégia de Snapshot e Rollback

## Status

Aceito

---

## Data

2026-06-13

---

## Contexto

O sistema possui uma etapa de execução (UC-006 Aplicar Alterações) que é a única do fluxo principal capaz de modificar fisicamente o sistema de arquivos do usuário.

Os ADRs anteriores estabeleceram pilares que tornam a reversibilidade obrigatória:

- ADR-004 — o Grafo de Conhecimento é a fonte da verdade e representa significado, não localização.
- ADR-005 — SQLite é a persistência principal e local-first.
- ADR-007 — toda alteração exige aprovação explícita do usuário.
- ADR-009 — a organização emerge do conhecimento e o usuário deve poder experimentar sugestões.

O UC-007 (Desfazer Alterações) define a reversibilidade como requisito de primeira classe: o usuário precisa poder reverter total ou parcialmente qualquer execução, sem medo de perder sua organização anterior.

É necessário decidir **como** o estado anterior será capturado (snapshot) e **como** ele será restaurado (rollback).

---

## Problema

Como capturar e restaurar o estado do sistema de arquivos antes de uma execução, de forma:

- segura
- auditável
- reversível
- eficiente em espaço e tempo
- compatível com o modelo local-first

sem copiar grandes volumes de dados e sem comprometer o conhecimento já armazenado no Grafo?

---

## Forças em Jogo

- Os arquivos do usuário podem ser grandes (mídia, documentos volumosos).
- As operações suportadas pelo sistema são predominantemente **estruturais e reversíveis por natureza**: criar diretório, mover arquivo, mover diretório, renomear, consolidar (ver UC-006).
- O sistema **não executa exclusões destrutivas** de conteúdo durante a organização.
- O banco já prevê as tabelas `snapshots` e `operations` (ver arquitetura de banco de dados).
- O histórico não pode ser apagado após a reversão (RN-004 do UC-007).
- O Grafo é derivado dos dados persistidos e deve refletir o estado restaurado (RN-007 do UC-007).

---

## Decisão

O sistema adotará uma estratégia de **snapshot lógico baseado em diário de operações (operation journal)**, e não em cópia integral do conteúdo dos arquivos.

Cada execução cria um snapshot que registra, de forma transacional, a **descrição reversível de cada operação** aplicada. O rollback é realizado pela aplicação das **operações inversas**, na ordem inversa.

A cópia física de conteúdo (snapshot de bytes) é reservada apenas para operações potencialmente destrutivas, tratadas como exceção e não como regra.

---

## Princípios

### Reversibilidade por Design

Toda operação registrada deve carregar informação suficiente para gerar sua operação inversa de maneira determinística.

---

### Snapshot Lógico Antes de Cópia Física

A reversão de operações estruturais (mover, renomear, criar) não exige duplicar conteúdo.

Basta registrar origem, destino e tipo da operação.

---

### Atomicidade Percebida

Uma execução deve se comportar como uma transação: ou o estado final é alcançado, ou o sistema é capaz de retornar ao estado inicial.

---

### Histórico Permanente

Snapshots e execuções permanecem registrados mesmo após o rollback.

A reversão é um novo evento, não um apagamento do anterior.

---

### Conhecimento Preservado

A reversão física deve sincronizar índices e Grafo, sem destruir o conhecimento semântico previamente aprendido.

---

## Modelo de Operações Reversíveis

Cada tipo de operação possui uma inversa bem definida:

| Operação | Inversa |
| --- | --- |
| Criar Diretório | Remover Diretório (se vazio) |
| Mover Arquivo (A → B) | Mover Arquivo (B → A) |
| Mover Diretório (A → B) | Mover Diretório (B → A) |
| Renomear (A → B) | Renomear (B → A) |
| Consolidar Estruturas | Desfazer movimentações que compuseram a consolidação |

A consolidação é decomposta em operações primitivas (mover + criar diretório), de modo que sua reversão é a soma das inversas primitivas.

---

## Estrutura do Snapshot

O snapshot é persistido no SQLite, reaproveitando as tabelas já previstas (`snapshots` e `operations`), associadas à execução.

```json
{
  "snapshotId": "snapshot-123",
  "executionId": "exec-001",
  "createdAt": "2026-06-13T12:00:00Z",
  "description": "Reorganização do Projeto Alpha",
  "operations": [
    {
      "order": 1,
      "type": "create_directory",
      "targetPath": "/Projeto Alpha"
    },
    {
      "order": 2,
      "type": "move",
      "sourcePath": "/Documentos/contrato.pdf",
      "targetPath": "/Projeto Alpha/contrato.pdf"
    },
    {
      "order": 3,
      "type": "rename",
      "sourcePath": "/Projeto Alpha/apresentacao.pptx",
      "targetPath": "/Projeto Alpha/apresentacao-final.pptx"
    }
  ]
}
```

Cada operação registra, no mínimo:

- ordem de aplicação
- tipo
- caminho de origem
- caminho de destino
- estado de execução (planejada, aplicada, revertida)

---

## Estratégia de Rollback

O rollback aplica as operações inversas em **ordem inversa** à da execução original.

```text
Execução:   create dir → move A → rename B
Rollback:   undo rename B → undo move A → remove dir
```

### Fluxo

```text
Execução Concluída
↓
Snapshot (diário de operações)
↓
Seleção pelo Usuário (UC-007)
↓
Validação de Integridade
↓
Geração das Operações Inversas
↓
Aplicação em Ordem Inversa
↓
Atualização do Índice
↓
Sincronização do Grafo
↓
Registro do Rollback
↓
Estado Restaurado
```

---

## Modos de Restauração

Coerente com o UC-007:

- **Completa** — todas as operações da execução são revertidas.
- **Parcial** — apenas operações selecionadas são revertidas.
- **Arquivo Específico** — reverte operações que afetam um único arquivo.
- **Diretório Específico** — reverte operações que afetam um diretório.

Na reversão parcial, o sistema valida dependências entre operações (por exemplo, não remover um diretório que ainda contém arquivos não revertidos).

---

## Tratamento de Conflitos e Inconsistências

Alinhado aos fluxos alternativos do UC-007:

- **Snapshot ausente ou corrompido (FA-001):** rollback indisponível; a validação de integridade ocorre antes de qualquer alteração.
- **Arquivo alterado manualmente após a execução (FA-002):** detectado por comparação de hash; o sistema solicita confirmação antes de sobrescrever.
- **Conflito de caminho de destino (FA-003):** o destino da restauração já está ocupado; o sistema solicita resolução.
- **Falha durante o rollback (FA-004):** a restauração é interrompida; as operações já revertidas permanecem registradas, garantindo que nenhuma alteração parcial fique sem rastreamento.

---

## Operações Potencialmente Destrutivas

Caso uma operação futura possa **sobrescrever** ou **eliminar** conteúdo (cenário hoje fora do fluxo principal), o snapshot lógico é insuficiente.

Para esses casos, a estratégia prevê um mecanismo complementar:

- mover o conteúdo afetado para uma **área de quarentena** interna ao invés de excluí-lo;
- ou registrar uma **cópia física** do conteúdo no snapshot.

A exclusão definitiva da quarentena só ocorre por ação explícita do usuário ou por política de retenção.

Isso preserva o princípio de que nenhuma operação do sistema é irreversível por padrão.

---

## Integridade e Validação

- Toda execução exige snapshot prévio (RN-001 do UC-006 e UC-007).
- A integridade do snapshot é validada antes do rollback.
- Hashes de arquivo são usados para detectar alterações manuais.
- A consistência entre sistema de arquivos, índice e Grafo é verificada ao final do rollback.

---

## Consequências Positivas

### Eficiência de Espaço

Não há duplicação de conteúdo para operações estruturais.

---

### Eficiência de Tempo

Snapshot e rollback operam sobre metadados, não sobre bytes.

---

### Auditabilidade

Cada operação e cada reversão é um evento rastreável e permanente.

---

### Compatibilidade Local-First

Toda a estratégia reside no SQLite local, sem dependência de servidor.

---

### Reversão Parcial

O modelo de diário permite reverter operações isoladas.

---

## Consequências Negativas

### Dependência de Determinismo

A inversa de cada operação precisa ser bem definida; novas operações exigem definição explícita de sua inversa.

---

### Sensibilidade a Alterações Externas

Mudanças manuais do usuário após a execução podem invalidar parcialmente o rollback, exigindo confirmação.

---

### Complexidade na Reversão Parcial

A validação de dependências entre operações adiciona complexidade.

---

### Necessidade de Quarentena para Casos Destrutivos

Operações destrutivas futuras exigem mecanismo adicional de proteção de conteúdo.

---

## Alternativas Consideradas

### Cópia Integral do Diretório (Snapshot Físico Completo)

Copiar todos os arquivos afetados antes da execução.

#### Motivo da Rejeição

Alto custo de espaço e tempo; inviável para grandes volumes; redundante para operações estruturais reversíveis.

---

### Snapshot do Sistema de Arquivos (ex.: cópia em nível de SO / COW)

Usar recursos de cópia-em-escrita do sistema operacional.

#### Motivo da Rejeição

Dependente de plataforma e sistema de arquivos; quebra a portabilidade e o princípio local-first multiplataforma do Tauri.

---

### Versionamento Estilo Git

Manter um repositório versionado do diretório do usuário.

#### Motivo da Rejeição

Excesso de complexidade; inadequado para arquivos binários grandes; experiência intrusiva sobre os dados do usuário.

---

## Impacto nos Casos de Uso

- **UC-006 Aplicar Alterações** — cria o snapshot lógico antes de executar.
- **UC-007 Desfazer Alterações** — consome o diário de operações e aplica as inversas.

---

## Impacto no Banco de Dados

Reaproveita as tabelas existentes:

- `snapshots` — uma entrada por execução.
- `operations` — uma entrada por operação reversível, com `operation_type`, `source_path`, `target_path` e `executed_at`.

Sugere-se acrescentar a essas operações um marcador de **estado de reversão** e a referência à execução, para suportar rollback parcial e auditoria.

---

## Impacto no Grafo de Conhecimento

O rollback altera apenas a **localização física** dos arquivos.

Como o Grafo representa significado e não localização (ADR-004), o conhecimento é preservado; apenas os caminhos indexados são ressincronizados após a restauração.

---

## Decisão Final

O sistema adotará **snapshot lógico baseado em diário de operações reversíveis**, com rollback por aplicação das operações inversas em ordem inversa.

A cópia física de conteúdo é reservada exclusivamente a operações potencialmente destrutivas, tratadas via quarentena.

Esta decisão garante reversibilidade obrigatória com mínimo custo de armazenamento e é considerada um componente de primeira classe da arquitetura, conforme exigido pelo UC-007.
