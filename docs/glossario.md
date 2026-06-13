# Glossário

## Objetivo

Este documento define os conceitos fundamentais utilizados pelo sistema.

Todos os documentos, implementações e discussões técnicas devem utilizar os termos aqui definidos.

---

# Arquivo

Representação de um arquivo físico existente no sistema operacional.

Um arquivo é a principal fonte de informação analisada pelo sistema.

Exemplos:

- contrato.pdf
- proposta.docx
- reuniao.txt
- orcamento.xlsx

Atributos comuns:

- caminho
- nome
- extensão
- tamanho
- hash
- data de criação
- data de modificação

---

# Arquivo Indexado

Arquivo que já passou pelo processo de escaneamento e possui informações armazenadas no banco de dados.

Um arquivo indexado pode possuir:

- metadados
- conteúdo extraído
- entidades associadas
- embeddings
- relações

---

# Metadado

Informação obtida sem analisar o conteúdo interno do arquivo.

Exemplos:

- nome
- extensão
- tamanho
- datas
- localização

---

# Conteúdo Extraído

Representação textual obtida a partir do conteúdo interno de um arquivo.

Exemplos:

- texto de PDFs
- texto de DOCX
- conteúdo de TXT
- OCR de imagens

O conteúdo extraído é utilizado como entrada para os modelos de IA.

---

# Entidade

Conceito identificado dentro do conteúdo de um ou mais arquivos.

Uma entidade representa algo que possui significado dentro do domínio do usuário.

Exemplos:

- Cliente XPTO
- Projeto Alpha
- João Silva
- Contrato
- Financeiro

Entidades podem ser compartilhadas por múltiplos arquivos.

---

# Tipo de Entidade

Categoria utilizada para classificar entidades.

Exemplos:

- Pessoa
- Organização
- Projeto
- Documento
- Tema
- Localização
- Data

---

# Embedding

Representação vetorial de um conteúdo textual.

Embeddings são utilizados para calcular similaridade semântica entre arquivos, entidades e grupos de informação.

O sistema utiliza embeddings para identificar relações que não são evidentes através de palavras exatas.

---

# Similaridade Semântica

Medida que representa o quão próximos dois conteúdos são em significado.

Arquivos semanticamente similares podem estar relacionados mesmo que possuam nomes completamente diferentes.

---

# Relação

Conexão identificada entre dois elementos do sistema.

Uma relação pode existir entre:

- arquivo ↔ arquivo
- arquivo ↔ entidade
- entidade ↔ entidade

Exemplos:

- menciona
- pertence_a
- relacionado_com
- similar_a

---

# Grafo de Conhecimento

Estrutura principal utilizada pelo sistema para representar conhecimento.

O grafo é composto por:

- nós
- relações

O grafo representa o entendimento construído pelo sistema sobre os arquivos analisados.

---

# Nó

Elemento individual do grafo.

Tipos de nós:

- Arquivo
- Entidade
- Cluster

---

# Aresta

Ligação entre dois nós do grafo.

Toda aresta possui:

- origem
- destino
- tipo
- nível de confiança

---

# Confiança

Valor numérico que representa o grau de certeza de uma inferência realizada pelo sistema.

Exemplo:

0.95 = alta confiança

0.42 = baixa confiança

Valores de confiança são utilizados para priorizar sugestões.

---

# Cluster

Agrupamento lógico de arquivos ou entidades que apresentam forte relação semântica.

Um cluster não corresponde necessariamente a uma pasta física.

Exemplos:

- Projeto Alpha
- Cliente XPTO
- Estudos de Redes
- Imposto de Renda 2025

---

# Contexto

Conjunto de informações utilizadas para compreender um arquivo.

O contexto pode incluir:

- conteúdo textual
- entidades extraídas
- relações existentes
- histórico de organização

---

# Extração de Entidades

Processo de identificar entidades relevantes dentro de um conteúdo.

Exemplo:

Texto:

"Contrato firmado entre Empresa XPTO e João Silva."

Entidades extraídas:

- Empresa XPTO
- João Silva
- Contrato

---

# Descoberta de Relações

Processo de identificar conexões entre arquivos e entidades.

Exemplo:

Dois documentos mencionam o mesmo projeto.

O sistema pode inferir uma relação entre eles.

---

# Sugestão

Proposta gerada pelo sistema para melhorar a organização ou representação do conhecimento.

Uma sugestão nunca é aplicada automaticamente.

Toda sugestão deve possuir:

- justificativa
- nível de confiança
- impacto esperado

---

# Operação

Alteração física realizada no sistema de arquivos.

Exemplos:

- mover arquivo
- renomear arquivo
- criar diretório

Operações somente podem ser executadas após aprovação do usuário.

---

# Plano de Organização

Conjunto de sugestões aprovadas pelo usuário.

Representa uma proposta de reorganização que poderá ser aplicada ao sistema de arquivos.

---

# Snapshot

Registro do estado atual dos arquivos antes da execução de alterações.

Snapshots permitem rollback.

---

# Rollback

Processo de desfazer alterações realizadas anteriormente.

O rollback deve restaurar o estado registrado no snapshot correspondente.

---

# Indexação

Processo responsável por transformar arquivos físicos em conhecimento utilizável pelo sistema.

Etapas:

1. Escaneamento
2. Extração de metadados
3. Extração de conteúdo
4. Geração de embeddings
5. Construção do grafo

---

# Motor de Inferência

Componente responsável por gerar conhecimento a partir dos dados indexados.

Suas responsabilidades incluem:

- extrair entidades
- identificar relações
- gerar clusters
- criar sugestões

---

# Organização Física

Estrutura de diretórios e arquivos existente no sistema operacional.

A organização física é considerada uma representação do conhecimento, não sua fonte principal.

---

# Fonte da Verdade

Representação considerada oficial pelo sistema.

Neste projeto, a fonte da verdade é o Grafo de Conhecimento.

A estrutura física de diretórios é apenas uma projeção possível desse conhecimento.
