# Caso de Uso: Escanear Diretório

## Identificação

**ID:** UC-001

**Nome:** Escanear Diretório

**Categoria:** Indexação

**Prioridade:** Alta

---

# Objetivo

Permitir que o usuário selecione um diretório e inicie o processo de descoberta dos arquivos que serão posteriormente indexados e analisados pelo sistema.

---

# Atores

## Primário

Usuário

---

## Secundários

- Serviço de Indexação
- Sistema de Arquivos
- Banco de Dados

---

# Pré-condições

- Aplicação iniciada.
- Diretório acessível pelo sistema operacional.
- Usuário possui permissões de leitura.
- Banco de dados disponível.

---

# Pós-condições

## Sucesso

- Arquivos encontrados são registrados.
- Estrutura inicial é persistida.
- Arquivos ficam disponíveis para indexação.

---

## Falha

- Nenhuma alteração é aplicada ao sistema.
- Erros são registrados.

---

# Fluxo Principal

## Passo 1

O usuário acessa a funcionalidade de seleção de diretório.

---

## Passo 2

O sistema exibe o seletor de diretórios do sistema operacional.

---

## Passo 3

O usuário seleciona um diretório raiz.

Exemplo:

```text
D:\Documentos
```

---

## Passo 4

O sistema valida:

- existência do diretório
- permissões de leitura
- acessibilidade

---

## Passo 5

O sistema registra o diretório selecionado.

---

## Passo 6

O usuário inicia o escaneamento.

---

## Passo 7

O Serviço de Indexação inicia a descoberta recursiva.

---

## Passo 8

O sistema percorre:

- diretório raiz
- subdiretórios
- arquivos

---

## Passo 9

Para cada arquivo encontrado o sistema coleta:

- caminho absoluto
- caminho relativo
- nome
- extensão

---

## Passo 10

O sistema registra os arquivos descobertos.

Status inicial:

```text
DISCOVERED
```

---

## Passo 11

O sistema atualiza o progresso em tempo real.

---

## Passo 12

Ao final do processo o sistema apresenta:

- quantidade de diretórios encontrados
- quantidade de arquivos encontrados
- tempo total

---

## Passo 13

Os arquivos ficam disponíveis para indexação.

---

# Fluxos Alternativos

## FA-001 — Diretório Inexistente

### Condição

O diretório selecionado não existe.

### Ação

O sistema exibe mensagem de erro.

### Resultado

Escaneamento não iniciado.

---

## FA-002 — Permissão Negada

### Condição

O usuário não possui acesso ao diretório.

### Ação

O sistema registra o erro.

### Resultado

Escaneamento cancelado.

---

## FA-003 — Arquivo Inacessível

### Condição

Um arquivo específico não pode ser lido.

### Ação

O sistema ignora o arquivo.

### Resultado

O escaneamento continua normalmente.

---

## FA-004 — Diretório Vazio

### Condição

Nenhum arquivo encontrado.

### Resultado

Processo concluído sem erros.

---

## FA-005 — Cancelamento

### Condição

Usuário cancela a operação.

### Ação

O sistema interrompe o escaneamento.

### Resultado

Estado parcial é preservado.

---

# Regras de Negócio

## RN-001

O escaneamento deve ser recursivo.

---

## RN-002

Nenhum conteúdo deve ser analisado nesta etapa.

Apenas descoberta.

---

## RN-003

Nenhuma chamada à IA deve ocorrer.

---

## RN-004

Arquivos descobertos devem receber status:

```text
DISCOVERED
```

---

## RN-005

Falhas individuais não interrompem o processo.

---

## RN-006

O sistema deve registrar estatísticas da execução.

---

# Eventos Emitidos

## ScanStarted

Emitido quando o escaneamento inicia.

---

## DirectoryDiscovered

Emitido quando um diretório é encontrado.

---

## FileDiscovered

Emitido quando um arquivo é encontrado.

---

## ScanProgress

Emitido periodicamente durante o processo.

---

## ScanCompleted

Emitido quando o escaneamento termina.

---

## ScanFailed

Emitido em falhas críticas.

---

## ScanCancelled

Emitido quando o usuário cancela.

---

# Dados Produzidos

## Diretórios

- caminho
- profundidade

---

## Arquivos

- caminho absoluto
- caminho relativo
- nome
- extensão

---

## Estatísticas

- total de diretórios
- total de arquivos
- duração
- quantidade de erros

---

# Integrações

## Sistema de Arquivos

Leitura recursiva da estrutura física.

---

## Banco de Dados

Persistência dos registros descobertos.

---

# Critérios de Aceitação

CA-001

O usuário consegue selecionar um diretório.

---

CA-002

Todos os arquivos acessíveis são descobertos.

---

CA-003

O progresso é exibido durante a execução.

---

CA-004

Arquivos descobertos são persistidos.

---

CA-005

Falhas individuais não interrompem o escaneamento.

---

CA-006

O resultado final apresenta estatísticas da execução.

---

# Observações Arquiteturais

Esta etapa não realiza:

- extração de conteúdo
- OCR
- embeddings
- inferências
- construção de grafo
- sugestões

Sua única responsabilidade é descobrir e registrar a existência dos arquivos.

As etapas posteriores serão responsáveis por transformar os arquivos descobertos em conhecimento.
