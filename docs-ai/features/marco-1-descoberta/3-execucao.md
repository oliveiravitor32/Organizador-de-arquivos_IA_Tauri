# 3-execucao.md — Marco 1: Descoberta

Derivado de `2-planejamento.md`. **Em andamento.**

## Checklist de implementação

### Backend

- [ ] **T1** — Adicionar `tauri-plugin-dialog`, `pdf-extract`, `docx-rs`, `calamine`, `sha2`, `mime_guess` ao `Cargo.toml`; registrar plugin no `lib.rs`.
- [ ] **T2** — `FileRepository` em `src-tauri/src/db/repositories/files.rs`: `insert`, `upsert_by_path`, `find_by_status`, `update_status`.
- [ ] **T3** — `ScanService` em `src-tauri/src/services/indexacao/scan.rs`: travessia recursiva, emite `ScanProgress` a cada 50 arquivos, persiste com status `DISCOVERED`, suporta cancelamento via `watch::Receiver`.
- [ ] **T4** — Command `escanear_diretorio`: valida path, inicia `ScanService` em `tauri::async_runtime::spawn`, retorna `{ scanId }`.
- [ ] **T5** — Extratores em `src-tauri/src/services/indexacao/extratores/`: `txt.rs`, `pdf.rs`, `docx.rs`, `xlsx.rs`, `imagem.rs` (metadados apenas). Trait `Extrator` comum.
- [ ] **T6** — `IndexingService` em `src-tauri/src/services/indexacao/indexing.rs`: itera `DISCOVERED`, chama extrator, persiste `file_contents`, atualiza status para `INDEXED` → `PENDING_ANALYSIS` ou `FAILED`.
- [ ] **T7** — Command `indexar_arquivos`: valida `scanId`, inicia `IndexingService` em task assíncrona, retorna `{ indexingId }`.
- [ ] **T8** — Command `cancelar_operacao`: sinaliza via `watch::Sender` compartilhado por `operationId`.
- [ ] **T9** — Testes unitários e de integração (ver plano de testes em `2-planejamento.md`).

### Frontend

- [ ] **T10** — Wrappers IPC: `escanearDiretorio`, `indexarArquivos`, `cancelarOperacao` em `src/ipc/commands.ts`; listeners de eventos em `src/ipc/events.ts`.
- [ ] **T11** — Store Zustand em `src/stores/descoberta.ts`: `scanId`, `indexingId`, `progresso`, `status`, `estatisticas`.
- [ ] **T12** — Tela `src/features/descoberta/Descoberta.tsx`: seletor de diretório → scan com barra de progresso → botão indexar → progresso → estatísticas finais. Usa só tokens semânticos.
- [ ] **T13** — Atualizar `src/App.tsx` ou criar roteamento mínimo para exibir a tela de Descoberta.
- [ ] **T14** — Testes de componente RTL: `src/features/descoberta/Descoberta.test.tsx`.

## Definition of Done

- [ ] Cada CA de UC-001 e UC-002 tem ≥ 1 teste (mapeamento em `2-planejamento.md`)
- [ ] Testes unitários Rust inline (`#[cfg(test)]`); integração em `src-tauri/tests/integracao.rs`
- [ ] Testes de componente frontend ao lado do arquivo testado
- [ ] Cobertura reportada (`npm run test -- --coverage` + `cargo llvm-cov` se disponível)
- [ ] Commands e events respeitam os contratos de `contratos-tauri.md` e `catalogo-de-eventos.md`
- [ ] Sem violação das regras inegociáveis (CLAUDE.md)
- [ ] UI usa só tokens de tema; nenhuma cor crua ou classe de cor direta do Tailwind
- [ ] Suíte passa offline e determinística (`npm run test`, `cargo test`)
- [ ] `cargo fmt --check` limpo; sem warnings no `cargo clippy`

## Verificação

1. `cargo test` — todos os testes unitários e de integração passam.
2. `npm run test` — todos os testes de componente passam.
3. `npm run tauri dev` — fluxo completo:
   - Tela de Descoberta visível.
   - Clicar "Selecionar diretório" abre o seletor nativo do SO.
   - Escolher um diretório inicia o scan e exibe progresso.
   - Ao término, botão "Indexar" fica disponível.
   - Indexação exibe progresso e estatísticas ao final.
   - Arquivos aparecem com status `PENDING_ANALYSIS` no banco (verificável via SQLite CLI).

## Registro

_A preencher após a implementação._
