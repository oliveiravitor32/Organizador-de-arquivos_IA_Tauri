# Requisitos Não Funcionais

## Objetivo

Este documento define os requisitos de qualidade, desempenho, segurança, confiabilidade e experiência do usuário para o sistema.

Os requisitos não funcionais estabelecem restrições e expectativas que devem ser respeitadas durante todo o desenvolvimento.

---

# RNF-001 — Funcionamento Offline

O sistema deve funcionar integralmente sem conexão com a internet.

## Critério de Aceitação

- Todas as funcionalidades principais devem operar offline.
- A ausência de conexão não deve impedir indexação, análise ou organização.
- Serviços externos devem ser opcionais.

---

# RNF-002 — Processamento Local

Toda análise deve ocorrer localmente por padrão.

## Critério de Aceitação

- Nenhum arquivo deve ser enviado para servidores externos sem autorização explícita do usuário.
- O processamento de IA deve ser executado localmente sempre que possível.

---

# RNF-003 — Privacidade

Os dados do usuário devem permanecer sob controle do próprio usuário.

## Critério de Aceitação

- Nenhum conteúdo deve ser compartilhado automaticamente.
- Logs não devem conter conteúdo sensível dos arquivos.
- Dados devem permanecer armazenados localmente.

---

# RNF-004 — Responsividade da Interface

A interface deve permanecer utilizável durante operações longas.

## Critério de Aceitação

Durante:

- Escaneamento
- Indexação
- Geração de embeddings
- Construção do grafo

A interface não deve bloquear completamente a interação do usuário.

---

# RNF-005 — Escalabilidade de Arquivos

O sistema deve suportar grandes volumes de arquivos.

## Meta Inicial

Suportar:

- 100.000 arquivos indexados

Sem degradação severa da experiência.

---

# RNF-006 — Escalabilidade do Grafo

O sistema deve suportar crescimento progressivo do conhecimento armazenado.

## Meta Inicial

Suportar:

- Centenas de milhares de entidades
- Milhões de relacionamentos

Sem necessidade de alteração arquitetural.

---

# RNF-007 — Uso de Memória

O sistema deve operar em computadores com recursos limitados.

## Hardware Alvo Inicial

- 8 GB RAM
- SSD recomendado
- CPU com pelo menos 4 núcleos

## Critério de Aceitação

A aplicação não deve exigir mais de 2 GB de RAM durante operações normais.

---

# RNF-008 — Inicialização

A aplicação deve iniciar rapidamente.

## Meta

Tempo de inicialização inferior a:

- 5 segundos

Em hardware compatível.

---

# RNF-009 — Persistência Confiável

Nenhuma informação indexada deve ser perdida em encerramentos inesperados.

## Critério de Aceitação

- Dados persistidos devem sobreviver a reinicializações.
- Operações devem ser transacionais sempre que possível.

---

# RNF-010 — Integridade dos Arquivos

O sistema não deve corromper arquivos do usuário.

## Critério de Aceitação

- Operações devem ser validadas antes da execução.
- Alterações devem possuir rollback disponível.

---

# RNF-011 — Auditabilidade

Toda operação relevante deve ser rastreável.

## Critério de Aceitação

Registrar:

- Data
- Operação
- Resultado
- Arquivos afetados

---

# RNF-012 — Recuperação

O sistema deve permitir recuperação após falhas.

## Critério de Aceitação

Antes de alterações físicas:

- Criar snapshot.
- Registrar plano de execução.

Após falha:

- Permitir rollback.

---

# RNF-013 — Explicabilidade

As inferências realizadas pela IA devem ser explicáveis.

## Critério de Aceitação

Toda sugestão deve apresentar:

- Evidências utilizadas
- Relações identificadas
- Nível de confiança

---

# RNF-014 — Transparência

O usuário deve compreender o estado atual do sistema.

## Critério de Aceitação

O sistema deve informar:

- Quantidade de arquivos indexados
- Progresso atual
- Operações em andamento
- Erros encontrados

---

# RNF-015 — Extensibilidade

A arquitetura deve permitir adicionar novos formatos de arquivo.

## Critério de Aceitação

Novos extratores devem poder ser adicionados sem alterar o restante do pipeline.

---

# RNF-016 — Extensibilidade de IA

O sistema não deve depender de um modelo específico.

## Critério de Aceitação

Modelos devem ser substituíveis.

Exemplos:

- Qwen
- Llama
- Mistral
- Gemma

---

# RNF-017 — Independência de Plataforma

A aplicação deve ser multiplataforma.

## Plataformas Alvo

- Windows
- Linux

## Plataforma Futura

- macOS

---

# RNF-018 — Consistência de Estado

O banco de dados e o sistema de arquivos devem permanecer sincronizados.

## Critério de Aceitação

Mudanças físicas devem atualizar o estado interno.

Mudanças detectadas externamente devem ser reindexadas.

---

# RNF-019 — Observabilidade

O sistema deve fornecer informações suficientes para diagnóstico.

## Critério de Aceitação

Disponibilizar:

- Logs estruturados
- Métricas básicas
- Histórico de execução

---

# RNF-020 — Modularidade

Os componentes principais devem ser desacoplados.

## Módulos Esperados

- Scanner
- Extratores
- Persistência
- IA
- Grafo
- Sugestões
- Aplicação de alterações

Mudanças em um módulo não devem exigir reescrita dos demais.

---

# RNF-021 — Reprocessamento Incremental

O sistema deve evitar trabalho desnecessário.

## Critério de Aceitação

Arquivos já processados não devem ser reanalisados sem necessidade.

A detecção deve utilizar:

- Hash
- Metadados
- Histórico

---

# RNF-022 — Tolerância a Falhas

Falhas individuais não devem interromper o processamento global.

## Critério de Aceitação

Um arquivo com erro não deve impedir:

- Escaneamento
- Indexação
- Análise dos demais arquivos

---

# RNF-023 — Configurabilidade

O usuário deve poder ajustar limites operacionais.

## Exemplos

- Quantidade de threads
- Diretórios ignorados
- Limite de processamento
- Modelos utilizados

---

# RNF-024 — Experiência do Usuário

O sistema deve priorizar clareza e segurança.

## Critério de Aceitação

- Nenhuma alteração automática.
- Confirmações explícitas.
- Feedback visual contínuo.
- Possibilidade de desfazer alterações.

---

# Restrições Arquiteturais

## RA-001

Utilizar SQLite como persistência principal.

---

## RA-002

Utilizar Tauri como plataforma desktop.

---

## RA-003

Utilizar React no frontend.

---

## RA-004

Utilizar IA local como estratégia principal.

---

## RA-005

Considerar o Grafo de Conhecimento como fonte da verdade do sistema.

---

# Metas para o MVP

O MVP será considerado aceitável quando:

- Funcionar totalmente offline.
- Indexar pelo menos 10.000 arquivos.
- Construir o grafo de conhecimento local.
- Executar em máquinas com 8 GB de RAM.
- Permitir rollback de alterações.
- Gerar sugestões explicáveis.
- Não exigir serviços externos.
