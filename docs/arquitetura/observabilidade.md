# Arquitetura: Observabilidade

## Objetivo

Este documento define como o sistema registra logs, métricas e histórico para diagnóstico — atendendo ao RNF-019.

Decisão de base em ADR-013: **logs locais estruturados, sem telemetria**.

---

# Princípios

## Local e Privado

Toda observabilidade é local. Nenhum dado sai da máquina (ADR-003, RNF-003).

---

## Sem Conteúdo Sensível

Logs e métricas nunca contêm o conteúdo dos arquivos. Apenas identificadores, caminhos e metadados.

---

## Estruturado

Logs são estruturados (campos consistentes), facilitando filtro e leitura.

---

## Suficiente para Diagnóstico

O conjunto logs + métricas + histórico deve permitir entender o que aconteceu sem reproduzir o problema.

---

# Logging

## Destino

Arquivo no diretório de logs da aplicação fornecido pelo sistema operacional (via Tauri).

Em desenvolvimento, também no console.

---

## Níveis

- **error** — falha que impede uma operação.
- **warn** — situação anômala recuperável (ex.: arquivo ignorado, inferência de baixa confiança).
- **info** — marcos de operação (início/fim de escaneamento, indexação, análise, execução).
- **debug** — detalhes para desenvolvimento (desativado por padrão).

---

## Campos Padrão

```text
timestamp   data/hora ISO 8601
level       error | warn | info | debug
module      serviço de origem (indexacao, ia, grafo, …)
operation   operationId quando aplicável
message     descrição
context     metadados (ids, contagens) — nunca conteúdo
```

---

## O que NUNCA registrar

- conteúdo textual de arquivos
- embeddings ou trechos de documentos
- qualquer dado que viole RNF-003

---

## Rotação

Logs são rotacionados por tamanho/idade para não crescer indefinidamente.

---

# Métricas

Métricas já previstas nos casos de uso, consolidadas para diagnóstico:

## Indexação

- arquivos descobertos / indexados / falhos
- tempo médio de indexação

---

## Análise (IA)

- entidades extraídas
- relações criadas
- confiança média
- tempo médio por arquivo
- taxa de falhas

---

## Grafo

- nós e relações
- densidade
- entidades órfãs

---

## Execução

- operações aplicadas / revertidas
- snapshots criados

---

# Histórico

Persistido no SQLite (não em log):

- execuções (`snapshots`, `operations`)
- estado dos arquivos (`files.status`)

Permite auditoria (RNF-011) e é a base do rollback (UC-007).

---

# Correlação

O `operationId` emitido nos events (`catalogo-de-eventos.md`) aparece também nos logs, permitindo correlacionar o que o usuário viu com o que o backend registrou.

---

# Critérios de Aceitação

- CA-001: logs são gravados em arquivo local, estruturados e com níveis.
- CA-002: nenhum log contém conteúdo de arquivos.
- CA-003: métricas de indexação, análise, grafo e execução estão disponíveis.
- CA-004: o histórico de execução sobrevive a reinicializações.
- CA-005: logs podem ser correlacionados a operações via `operationId`.
- CA-006: nenhum dado de diagnóstico é transmitido externamente.

---

# Observação

Observabilidade é transversal. O Marco 0 estabelece o logger estruturado base; cada marco seguinte adiciona suas métricas específicas.
