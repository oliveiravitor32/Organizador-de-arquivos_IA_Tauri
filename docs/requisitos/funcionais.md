# Requisitos Funcionais

## Objetivo

Este documento descreve as funcionalidades que o sistema deve fornecer aos usuários.

Os requisitos funcionais definem comportamentos observáveis do sistema, independentemente da implementação técnica.

---

# Módulo: Gerenciamento de Diretórios

## RF-001 — Selecionar Diretório Raiz

O sistema deve permitir que o usuário selecione um diretório raiz para análise.

### Critério de Aceitação

- O usuário pode selecionar um diretório local.
- O sistema valida a existência do diretório.
- O diretório selecionado é registrado no projeto atual.

---

## RF-002 — Gerenciar Diretórios Monitorados

O sistema deve permitir adicionar e remover diretórios monitorados.

### Critério de Aceitação

- O usuário pode visualizar todos os diretórios cadastrados.
- O usuário pode remover um diretório existente.
- A remoção não deve apagar arquivos físicos.

---

# Módulo: Indexação

## RF-003 — Escanear Diretórios

O sistema deve percorrer recursivamente os diretórios selecionados.

### Critério de Aceitação

- Todos os arquivos acessíveis são descobertos.
- O progresso é exibido ao usuário.
- Falhas individuais não interrompem o processo.

---

## RF-004 — Extrair Metadados

O sistema deve coletar metadados dos arquivos encontrados.

### Critério de Aceitação

Os seguintes dados devem ser obtidos quando disponíveis:

- Nome
- Caminho
- Extensão
- Tamanho
- Hash
- Data de criação
- Data de modificação

---

## RF-005 — Extrair Conteúdo

O sistema deve extrair conteúdo textual de formatos suportados.

### Critério de Aceitação

- O conteúdo extraído deve ser armazenado localmente.
- Arquivos não suportados continuam indexados através de metadados.

---

## RF-006 — Reindexação Incremental

O sistema deve detectar alterações em arquivos previamente indexados.

### Critério de Aceitação

O sistema deve identificar:

- Arquivos novos
- Arquivos modificados
- Arquivos removidos
- Arquivos movidos

---

# Módulo: Enriquecimento Semântico

## RF-007 — Gerar Embeddings

O sistema deve gerar representações vetoriais dos conteúdos indexados.

### Critério de Aceitação

- Embeddings devem ser associados aos arquivos correspondentes.
- O processamento deve ocorrer localmente.

---

## RF-008 — Extrair Entidades

O sistema deve identificar entidades relevantes presentes nos conteúdos.

### Critério de Aceitação

Exemplos de entidades:

- Pessoas
- Organizações
- Projetos
- Temas
- Conceitos

---

## RF-009 — Identificar Relações

O sistema deve identificar relações entre arquivos e entidades.

### Critério de Aceitação

- Relações devem possuir nível de confiança.
- Relações devem ser armazenadas no grafo.

---

## RF-010 — Construir Grafo de Conhecimento

O sistema deve construir e manter um grafo de conhecimento local.

### Critério de Aceitação

O grafo deve representar:

- Arquivos
- Entidades
- Relações
- Clusters

---

# Módulo: Descoberta de Conhecimento

## RF-011 — Agrupar Arquivos Relacionados

O sistema deve identificar agrupamentos semânticos de arquivos.

### Critério de Aceitação

- Arquivos relacionados podem pertencer ao mesmo cluster.
- O agrupamento deve possuir justificativa.

---

## RF-012 — Detectar Similaridade Semântica

O sistema deve identificar arquivos semanticamente semelhantes.

### Critério de Aceitação

- Similaridade não deve depender apenas de nomes ou extensões.
- A similaridade deve utilizar embeddings.

---

## RF-013 — Descobrir Contextos Compartilhados

O sistema deve identificar contextos compartilhados entre arquivos.

### Critério de Aceitação

Exemplos:

- Mesmo cliente
- Mesmo projeto
- Mesmo tema
- Mesmo assunto

---

# Módulo: Busca e Exploração

## RF-014 — Buscar Arquivos

O sistema deve permitir localizar arquivos indexados.

### Critério de Aceitação

A busca deve suportar:

- Nome
- Conteúdo
- Entidades
- Contexto

---

## RF-015 — Navegar pelo Grafo

O sistema deve permitir explorar relações entre arquivos e entidades.

### Critério de Aceitação

O usuário pode visualizar:

- Arquivos relacionados
- Entidades associadas
- Clusters
- Relações inferidas

---

## RF-016 — Visualizar Contexto de um Arquivo

O sistema deve apresentar o contexto conhecido de um arquivo.

### Critério de Aceitação

O usuário deve visualizar:

- Entidades encontradas
- Relações existentes
- Similaridades
- Clusters associados

---

# Módulo: Sugestões

## RF-017 — Gerar Sugestões

O sistema deve gerar sugestões de organização baseadas no conhecimento extraído.

### Critério de Aceitação

Cada sugestão deve possuir:

- Descrição
- Justificativa
- Confiança

---

## RF-018 — Explicar Sugestões

O sistema deve explicar como uma sugestão foi gerada.

### Critério de Aceitação

O usuário consegue compreender:

- Quais evidências foram utilizadas.
- Quais relações motivaram a sugestão.

---

## RF-019 — Revisar Sugestões

O sistema deve permitir revisão individual das sugestões.

### Critério de Aceitação

O usuário pode:

- Aprovar
- Rejeitar
- Ignorar

---

# Módulo: Organização Física

## RF-020 — Aplicar Alterações

O sistema deve aplicar alterações aprovadas pelo usuário.

### Critério de Aceitação

Alterações possíveis:

- Mover arquivos
- Renomear arquivos
- Criar diretórios

---

## RF-021 — Criar Snapshot

O sistema deve registrar o estado atual antes de executar alterações.

### Critério de Aceitação

Todo plano aprovado gera um snapshot.

---

## RF-022 — Desfazer Alterações

O sistema deve permitir rollback das alterações executadas.

### Critério de Aceitação

O estado anterior deve ser restaurado a partir do snapshot.

---

# Módulo: Observabilidade

## RF-023 — Exibir Progresso

O sistema deve informar o progresso de operações longas.

### Critério de Aceitação

Operações monitoradas:

- Escaneamento
- Indexação
- Análise
- Aplicação de alterações

---

## RF-024 — Exibir Histórico

O sistema deve registrar operações realizadas.

### Critério de Aceitação

O usuário pode consultar:

- Data
- Operação
- Resultado
- Arquivos afetados

---

# Módulo: Configuração

## RF-025 — Configurar Modelo de IA

O sistema deve permitir selecionar modelos de IA disponíveis localmente.

### Critério de Aceitação

O usuário pode definir:

- Modelo principal
- Modelo de embeddings
- Limites de processamento

---

## RF-026 — Configurar Regras de Indexação

O sistema deve permitir configurar exclusões.

### Critério de Aceitação

Exemplos:

- node_modules
- .git
- diretórios específicos
- extensões específicas

---

# Restrições Funcionais

## RF-027 — Nenhuma Alteração Automática

O sistema não deve realizar alterações físicas sem aprovação explícita do usuário.

---

## RF-028 — Funcionamento Offline

Todas as funcionalidades principais devem operar sem conexão com a internet.

---

# Resultado Esperado

Ao final do fluxo principal, o usuário deve ser capaz de:

1. Selecionar um diretório.
2. Indexar seus arquivos.
3. Construir um grafo de conhecimento.
4. Descobrir relações entre documentos.
5. Receber sugestões justificadas.
6. Aprovar alterações.
7. Organizar seus arquivos com segurança.
