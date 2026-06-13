# Visão do Produto

# Nome Provisório

AI File Knowledge Manager

---

# Visão

O AI File Knowledge Manager é uma aplicação desktop que utiliza inteligência artificial local para compreender, relacionar e organizar informações contidas em arquivos.

Diferentemente de organizadores tradicionais, o sistema não depende exclusivamente de nomes de arquivos, extensões ou regras pré-definidas. Seu objetivo é analisar o conteúdo dos arquivos, extrair conhecimento relevante e construir uma representação semântica capaz de identificar relações entre documentos, projetos, pessoas, clientes, temas e conceitos.

A partir desse entendimento, o sistema gera sugestões explicáveis de organização, agrupamento e estruturação do conhecimento armazenado pelo usuário.

A organização física de pastas é tratada como uma possível representação desse conhecimento, e não como sua fonte principal.

---

# Problema

Usuários acumulam milhares de arquivos ao longo do tempo.

Esses arquivos frequentemente ficam distribuídos em diretórios criados em momentos diferentes, seguindo padrões inconsistentes de nomenclatura e organização.

Como consequência:

- Arquivos relacionados ficam separados.
- Projetos perdem contexto.
- Informações importantes tornam-se difíceis de localizar.
- Estruturas de pastas deixam de representar o conhecimento real existente nos documentos.

Soluções tradicionais dependem de regras rígidas e classificações baseadas em extensões, não sendo capazes de compreender o significado dos arquivos.

---

# Objetivo

Construir uma camada inteligente de conhecimento sobre o sistema de arquivos do usuário.

O sistema deve ser capaz de:

- Compreender o conteúdo dos arquivos.
- Identificar entidades relevantes.
- Descobrir relações entre documentos.
- Construir um grafo de conhecimento local.
- Sugerir agrupamentos semânticos.
- Propor reorganizações explicáveis.
- Permitir exploração do conhecimento armazenado.

---

# Princípios Fundamentais

## Conteúdo acima da estrutura

O conteúdo dos arquivos possui maior importância que sua localização atual no sistema de pastas.

## Relações acima de extensões

A relação semântica entre arquivos é mais importante do que seu formato.

## IA como mecanismo de descoberta

A inteligência artificial deve ser utilizada para identificar conhecimento e relações que não seriam facilmente encontradas por regras estáticas.

## Controle total do usuário

Nenhuma alteração física deve ser aplicada sem aprovação explícita do usuário.

## Processamento local

Sempre que possível, análises e inferências devem ocorrer localmente para preservar privacidade e reduzir dependência de serviços externos.

---

# Escopo Inicial (MVP)

O MVP deve ser capaz de:

1. Selecionar um diretório raiz.
2. Escanear arquivos recursivamente.
3. Extrair metadados dos arquivos.
4. Extrair conteúdo textual de formatos suportados.
5. Armazenar informações em banco local.
6. Utilizar IA local para identificar entidades e contexto.
7. Construir um grafo de conhecimento básico.
8. Gerar sugestões de agrupamento.
9. Exibir justificativas para cada sugestão.
10. Aplicar alterações aprovadas pelo usuário.
11. Permitir desfazer alterações realizadas.

---

# Fora do Escopo Inicial

Não fazem parte da primeira versão:

- Sincronização em nuvem.
- Compartilhamento entre dispositivos.
- Colaboração multiusuário.
- Exclusão automática de arquivos.
- Aplicação automática de sugestões.
- Dependência obrigatória de APIs externas.
- Indexação em tempo real do sistema inteiro.

---

# Arquitetura Conceitual

Sistema de Arquivos
↓
Scanner
↓
Extração de Conteúdo
↓
Extração de Entidades
↓
Embeddings
↓
Grafo de Conhecimento
↓
Motor de Sugestões
↓
Interface de Revisão
↓
Aplicação das Alterações

---

# Visão de Longo Prazo

Evoluir de um organizador de arquivos para um assistente inteligente de conhecimento pessoal.

No futuro, o sistema poderá responder perguntas como:

- "Quais arquivos pertencem ao Projeto Alpha?"
- "Quais documentos estão relacionados ao Cliente XPTO?"
- "Quais arquivos possuem conteúdo semelhante?"
- "Quais informações estão duplicadas?"
- "Quais documentos não são utilizados há mais de dois anos?"

O objetivo final é transformar arquivos isolados em uma rede navegável de conhecimento.
