# 2-planejamento.md — Marco 1: Descoberta

Derivado de `1-pesquisa.md`.

## Abordagem

Implementar em duas camadas sequenciais:

1. **Backend Rust:** serviços de scan e indexação, repositório de arquivos, extratores de conteúdo.
2. **Frontend React:** tela de Descoberta com seletor de diretório, progresso em tempo real e estatísticas finais.

Operações longas (scan, indexing) retornam imediatamente um ID e comunicam progresso via events — conforme o contrato de `contratos-tauri.md`.

## Tarefas

### Backend

| # | Tarefa | Depende de |
| --- | --- | --- |
| T1 | Adicionar `tauri-plugin-dialog` ao `Cargo.toml` e `lib.rs` | — |
| T2 | `FileRepository`: insert, upsert por caminho, query por status | — |
| T3 | `ScanService`: travessia recursiva, emite eventos, persiste com status `DISCOVERED` | T2 |
| T4 | Command `escanear_diretorio`: valida path, inicia `ScanService` em task assíncrona, retorna `scanId` | T3 |
| T5 | Extratores de conteúdo: TXT/MD/JSON/YAML/XML/código (std::fs), PDF (`pdf-extract`), DOCX (`docx-rs`), XLSX (`calamine`), imagens (metadados apenas) | — |
| T6 | `IndexingService`: itera arquivos `DISCOVERED`, chama extrator, persiste em `file_contents`, atualiza status | T2, T5 |
| T7 | Command `indexar_arquivos`: valida `scanId`, inicia `IndexingService` em task assíncrona, retorna `indexingId` | T6 |
| T8 | Command `cancelar_operacao`: sinaliza cancelamento via channel/flag | T4, T7 |
| T9 | Testes unitários do backend (extratores, regras de negócio, repositório) | T2–T8 |

### Frontend

| # | Tarefa | Depende de |
| --- | --- | --- |
| T10 | Wrappers IPC: `escanearDiretorio`, `indexarArquivos`, `cancelarOperacao` em `src/ipc/commands.ts` | T4, T7, T8 |
| T11 | Listeners de eventos em `src/ipc/events.ts`: scan e indexing | — |
| T12 | Store Zustand: estado da operação em andamento (scanId, indexingId, progresso, status) | T10, T11 |
| T13 | Tela `Descoberta`: botão seletor de diretório → inicia scan → progresso → botão indexar → progresso → estatísticas | T10–T12 |
| T14 | Testes de componente (RTL): tela de Descoberta com mocks IPC | T13 |

## Pontos de integração

- `tauri-plugin-dialog`: novo plugin Tauri para seletor de diretório nativo.
- `AppState`: pool SQLite já existente; `FileRepository` consome o pool.
- Eventos: `src/ipc/events.ts` expande os listeners além do `app://ready` atual.
- `src/App.tsx`: rota ou renderização condicional para a tela de Descoberta (estrutura de navegação mínima).

## Plano de testes

### UC-001

| CA | Teste | Nível | Arquivo |
| --- | --- | --- | --- |
| CA-001 | seletor abre e retorna path | componente (RTL) | `src/features/descoberta/Descoberta.test.tsx` |
| CA-002 | scan recursivo encontra todos os arquivos acessíveis | integração (Rust) | `src-tauri/tests/integracao.rs` |
| CA-003 | evento ScanProgress chega ao frontend durante o scan | componente (RTL) | `src/features/descoberta/Descoberta.test.tsx` |
| CA-004 | arquivos descobertos têm status DISCOVERED no banco | integração (Rust) | `src-tauri/tests/integracao.rs` |
| CA-005 | arquivo inacessível não interrompe o scan (FA-003) | unidade (Rust) | inline em `services/indexacao/scan.rs` |
| CA-006 | ScanCompleted carrega totalArquivos e totalDiretorios | integração (Rust) | `src-tauri/tests/integracao.rs` |

### UC-002

| CA | Teste | Nível | Arquivo |
| --- | --- | --- | --- |
| CA-001 | metadados (hash, tamanho, datas) persistidos corretamente | integração (Rust) | `src-tauri/tests/integracao.rs` |
| CA-002 | conteúdo extraído de TXT, PDF, DOCX, XLSX | unidade (Rust) | inline em `services/indexacao/extratores/` |
| CA-003 | arquivo de formato não suportado → só metadados, status INDEXED | unidade (Rust) | inline em `services/indexacao/extratores/` |
| CA-004 | arquivo corrompido → status FAILED, demais continuam | unidade (Rust) | inline em `services/indexacao/` |
| CA-005 | após indexação, status é PENDING_ANALYSIS | integração (Rust) | `src-tauri/tests/integracao.rs` |
| CA-006 | estatísticas finais (processados, ignorados, falhos, tempo) | integração (Rust) | `src-tauri/tests/integracao.rs` |

## Decisões tomadas

| Decisão | Escolha | Por quê |
| --- | --- | --- |
| Extração PDF/DOCX/XLSX | `pdf-extract` + `docx-rs` + `calamine` (ADR-016) | Rust puro, sem deps nativas, suficiente para M1 |
| OCR em imagens | Adiado para M6 (ADR-017) | UC-002 marca como opcional; evita dep de sistema |
| Seletor de diretório | `tauri-plugin-dialog` | Único mecanismo suportado pelo Tauri v2 para diálogo nativo |
| Cancelamento | channel `tokio::sync::watch` por operação | Permite sinalizar cancelamento a tasks em andamento sem matar threads |
| Progresso do scan | emitir `ScanProgress` a cada 50 arquivos | Equilíbrio entre granularidade e overhead de IPC |
| Hash | SHA-256 via `sha2` crate, streaming | Sem carregar arquivo inteiro na memória |
