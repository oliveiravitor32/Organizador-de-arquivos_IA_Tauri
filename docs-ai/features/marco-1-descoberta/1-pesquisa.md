# 1-pesquisa.md — Marco 1: Descoberta

## Feature

Transformar um diretório selecionado pelo usuário em arquivos indexados com metadados e conteúdo extraído, sem envolver IA.

## Casos de uso envolvidos

- [UC-001 — Escanear Diretório](../../../docs/casos-de-uso/01-descoberta/UC-001-escanear-diretorio.md)
- [UC-002 — Indexar Arquivos](../../../docs/casos-de-uso/01-descoberta/UC-002-indexar-arquivo.md)

## Decisões aplicáveis

| ADR | Relevância |
| --- | --- |
| ADR-002 | Tauri como plataforma — seletor de diretório via `tauri-plugin-dialog` |
| ADR-005 | SQLite + sqlx — persistência de arquivos, metadados e conteúdo |
| ADR-006 | Indexação independente da IA — nenhuma chamada ao modelo neste marco |
| ADR-011 | Stack técnica (Tauri v2, sqlx, Vite) |
| ADR-012 | UI com shadcn/ui + Tailwind, tokens semânticos |
| ADR-014 | Vitest + RTL (frontend) / cargo test (backend) |
| ADR-015 | TanStack Query para estado assíncrono IPC |
| ADR-016 | `pdf-extract` + `docx-rs` + `calamine` para extração de conteúdo |
| ADR-017 | OCR adiado para M6; imagens indexadas só com metadados |

## Contratos e dados

### Commands (contratos-tauri.md)

- `escanear_diretorio({ rootPath, ignore })` → `{ scanId }` (UC-001)
- `indexar_arquivos({ scanId })` → `{ indexingId }` (UC-002)
- `cancelar_operacao({ operationId })` (FA-005 de ambos os UCs)

### Events (catalogo-de-eventos.md)

**Escaneamento:** ScanStarted · DirectoryDiscovered · FileDiscovered · ScanProgress · ScanCompleted · ScanFailed · ScanCancelled

**Indexação:** IndexingStarted · FileIndexingStarted · IndexingProgress · IndexingCompleted · IndexingFailed · IndexingCancelled

### Tabelas afetadas (esquema-sql.md)

- `files` — registro de cada arquivo (caminho, hash, tamanho, datas, status, mime_type)
- `file_contents` — conteúdo extraído (raw_text, language, char_count)

Status do ciclo de vida neste marco:
```
DISCOVERED → INDEXED → PENDING_ANALYSIS
           → FAILED
```

## Onde mora o código

### Backend (Rust)

```
src-tauri/src/
├── commands/           # escanear_diretorio, indexar_arquivos, cancelar_operacao
├── services/
│   └── indexacao/      # ScanService, IndexingService (novo)
├── domain/             # FileRecord, ScanStats, IndexingStats (novo)
└── db/
    └── repositories/   # FileRepository (novo)
```

### Frontend (React)

```
src/
├── features/
│   └── descoberta/     # tela de Indexação, componentes de progresso (novo)
├── ipc/
│   ├── commands.ts     # wrappers escanear_diretorio, indexar_arquivos
│   └── events.ts       # listeners dos eventos de scan/indexing
└── stores/             # store de operação em andamento (Zustand)
```

## Critérios de aceitação

### UC-001

- CA-001: usuário consegue selecionar um diretório
- CA-002: todos os arquivos acessíveis são descobertos
- CA-003: progresso é exibido durante a execução
- CA-004: arquivos descobertos são persistidos
- CA-005: falhas individuais não interrompem o escaneamento
- CA-006: resultado final apresenta estatísticas

### UC-002

- CA-001: metadados são persistidos corretamente
- CA-002: conteúdo é extraído quando suportado
- CA-003: arquivos não suportados continuam indexados (só metadados)
- CA-004: falhas individuais não interrompem a execução
- CA-005: arquivos indexados ficam disponíveis para análise posterior
- CA-006: sistema registra estatísticas completas

## Riscos e questões em aberto

- **Performance em diretórios grandes:** escaneamento recursivo pode ser lento para pastas com muitos arquivos. Estratégia: processar em lote + emitir ScanProgress a cada N arquivos.
- **Hash de arquivos grandes:** SHA-256 em arquivo de GBs pode ser lento. Estratégia: streaming hash sem carregar o arquivo inteiro na memória.
- **`pdf-extract` em PDFs complexos:** extração pode ser incompleta. Aceitável no M1 — texto parcial é melhor que nenhum.
- **`tauri-plugin-dialog`:** precisa ser adicionado ao `Cargo.toml` e registrado no `lib.rs`.
