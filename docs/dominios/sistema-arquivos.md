# Domínio: Sistema de Arquivos

## Objetivo

Este documento define como o sistema enxerga, representa e manipula o sistema de arquivos do usuário.

O sistema de arquivos é a origem de todo o conhecimento e o destino final das alterações aprovadas.

Ele é tratado como uma **projeção** do conhecimento, e não como a fonte da verdade.

---

# Filosofia

## Estrutura Física como Projeção

A organização física dos diretórios é apenas uma representação possível do conhecimento.

A fonte da verdade é o Grafo de Conhecimento (ver ADR-004).

---

## Leitura Livre, Escrita Controlada

A leitura do sistema de arquivos é livre e ocorre durante a indexação.

A escrita só ocorre após aprovação explícita do usuário (ver ADR-007) e sempre com snapshot prévio (ver ADR-010).

---

## Não Destrutividade

Nenhuma operação do sistema deve destruir conteúdo de forma irreversível.

---

# Conceitos

## Diretório Raiz

Diretório selecionado pelo usuário como ponto de partida da indexação.

Define o escopo de atuação do sistema.

---

## Arquivo

Unidade básica de informação física.

Cada arquivo é representado internamente por um registro com identidade própria.

---

## Caminho

Localização do arquivo no sistema de arquivos.

O sistema distingue:

- caminho absoluto
- caminho relativo ao diretório raiz

---

## Identidade do Arquivo

A identidade de um arquivo não depende apenas do caminho.

É determinada pela combinação de:

- hash de conteúdo
- metadados

Isso permite reconhecer arquivos movidos ou renomeados.

---

# Operações Suportadas

## Leitura

- percorrer diretórios
- ler metadados
- extrair conteúdo

---

## Escrita

- criar diretório
- mover arquivo
- mover diretório
- renomear arquivo
- renomear diretório
- consolidar estruturas

Todas reversíveis, conforme ADR-010.

---

## Operações Proibidas por Padrão

- exclusão definitiva de conteúdo
- sobrescrita destrutiva

Quando inevitáveis, devem passar por área de quarentena (ver ADR-010).

---

# Metadados Coletados

- nome
- extensão
- tamanho
- hash
- tipo MIME
- data de criação
- data de modificação

---

# Formatos

## Com Extração de Conteúdo

- TXT
- Markdown
- PDF
- DOCX
- XLSX
- Imagens (OCR opcional)

---

## Sem Extração de Conteúdo

Arquivos não suportados continuam sendo indexados.

Participam do sistema apenas através de metadados.

---

# Detecção de Alterações

Um arquivo deve ser reprocessado quando:

- o conteúdo muda
- o hash muda
- o arquivo é renomeado
- o arquivo é movido

---

# Relação com o Grafo

O sistema de arquivos alimenta o grafo durante a indexação.

O grafo, por sua vez, orienta as alterações aplicadas ao sistema de arquivos.

```text
Sistema de Arquivos
↓ (indexação)
Grafo de Conhecimento
↓ (sugestões aprovadas)
Sistema de Arquivos
```

A estrutura física pode mudar completamente sem alterar o conhecimento armazenado.

---

# Segurança e Integridade

## Snapshot Antes de Alterar

Toda alteração exige snapshot prévio.

---

## Operações Auditáveis

Toda operação física é registrada.

---

## Verificação de Acesso

O sistema valida existência e permissões antes de qualquer escrita.

---

## Tolerância a Falhas

A falha em um arquivo não interrompe o processamento dos demais.

---

# Restrições

## Escopo Limitado ao Diretório Raiz

O sistema não atua fora do diretório selecionado pelo usuário.

---

## Itens Ignorados

O usuário pode configurar arquivos e diretórios a serem ignorados.

---

# Estados de um Arquivo

Coerentes com o domínio de Indexação:

- discovered
- indexed
- pending_analysis
- analyzed
- failed

---

# Resultado Esperado

O sistema deve ser capaz de:

- descobrir arquivos dentro do escopo
- reconhecer arquivos movidos ou renomeados
- extrair conteúdo quando possível
- aplicar alterações de forma segura e reversível
- preservar a integridade dos dados do usuário

mantendo o sistema de arquivos sempre como uma projeção do conhecimento, nunca como sua fonte.
