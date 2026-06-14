//! Serviço de escaneamento recursivo de diretório (UC-001).

use std::path::Path;
use std::time::Instant;

use tauri::{AppHandle, Emitter};
use tokio::sync::watch;
use walkdir::{DirEntry, WalkDir};

use crate::db::repositories::files::FileRepository;
use crate::domain::arquivo::NewFile;
use crate::domain::scan::{ScanProgress, ScanStats};
use crate::error::{AppError, AppResult};
use crate::events;

const PROGRESS_INTERVAL: u64 = 50;

/// Diretórios ignorados por padrão (pruned antes de entrar — zero custo).
/// Configurável pelo usuário no M6 via Configurações.
const DEFAULT_IGNORE: &[&str] = &[
    // Gerenciadores de pacotes / artefatos de build
    "node_modules",
    "target",
    "dist",
    "build",
    ".next",
    ".nuxt",
    ".vite",
    ".turbo",
    "__pycache__",
    ".venv",
    "venv",
    ".gradle",
    ".m2",
    "vendor",
    // VCS e ferramentas de desenvolvimento
    ".git",
    ".svn",
    ".hg",
    ".idea",
    ".vscode",
    ".cache",
    // Windows / OS
    "$RECYCLE.BIN",
    "System Volume Information",
    "Windows",
    "Program Files",
    "Program Files (x86)",
    "AppData",
];

pub struct ScanService {
    app: AppHandle,
    pool: sqlx::SqlitePool,
}

impl ScanService {
    pub fn new(app: AppHandle, pool: sqlx::SqlitePool) -> Self {
        Self { app, pool }
    }

    /// Percorre `root_path` recursivamente com poda de diretórios ignorados.
    /// O `ignore` do usuário é somado aos defaults; cancela via `cancel_rx`.
    pub async fn escanear(
        &self,
        scan_id: &str,
        root_path: &Path,
        ignore: &[String],
        cancel_rx: watch::Receiver<bool>,
    ) -> AppResult<ScanStats> {
        let inicio = Instant::now();
        let repo = FileRepository::new(&self.pool);
        let mut stats = ScanStats::default();

        self.app
            .emit(
                events::SCAN_STARTED,
                serde_json::json!({ "scanId": scan_id }),
            )
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // filter_entry poda o diretório ANTES de entrar — sem custo de travessia interna.
        let walker = WalkDir::new(root_path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| !deve_ignorar(e, ignore));

        for entry in walker {
            if *cancel_rx.borrow() {
                self.app
                    .emit(
                        events::SCAN_CANCELLED,
                        serde_json::json!({ "scanId": scan_id }),
                    )
                    .ok();
                return Ok(stats);
            }

            let entry = match entry {
                Ok(e) => e,
                Err(_) => {
                    stats.total_errors += 1;
                    continue;
                }
            };

            let path = entry.path();

            if entry.file_type().is_dir() {
                // O root em si não conta como diretório descoberto.
                if path != root_path {
                    stats.total_dirs += 1;
                    self.app
                        .emit(
                            events::SCAN_DIRECTORY_DISCOVERED,
                            serde_json::json!({
                                "scanId": scan_id,
                                "path": path.to_string_lossy()
                            }),
                        )
                        .ok();
                }
                continue;
            }

            if !entry.file_type().is_file() {
                continue;
            }

            stats.total_files += 1;

            let abs_path = path.to_string_lossy().to_string();
            let rel_path = path
                .strip_prefix(root_path)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| abs_path.clone());
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let extension = path
                .extension()
                .map(|e| e.to_string_lossy().to_lowercase().to_string());

            let new_file = NewFile {
                scan_id: scan_id.to_string(),
                path: abs_path.clone(),
                relative_path: rel_path,
                name,
                extension,
            };

            if repo.upsert_discovered(&new_file).await.is_err() {
                stats.total_errors += 1;
            }

            self.app
                .emit(
                    events::SCAN_FILE_DISCOVERED,
                    serde_json::json!({ "scanId": scan_id, "path": abs_path }),
                )
                .ok();

            if stats.total_files % PROGRESS_INTERVAL == 0 {
                let progress = ScanProgress {
                    scan_id: scan_id.to_string(),
                    files_found: stats.total_files,
                    dirs_found: stats.total_dirs,
                    current_path: abs_path,
                };
                self.app.emit(events::SCAN_PROGRESS, &progress).ok();
            }
        }

        stats.duration_ms = inicio.elapsed().as_millis() as u64;

        self.app
            .emit(
                events::SCAN_COMPLETED,
                serde_json::json!({
                    "scanId": scan_id,
                    "totalArquivos": stats.total_files,
                    "totalDiretorios": stats.total_dirs,
                    "totalErros": stats.total_errors,
                    "durationMs": stats.duration_ms,
                }),
            )
            .ok();

        Ok(stats)
    }
}

/// Retorna true se a entrada deve ser ignorada (poda via filter_entry).
/// Compara o nome do componente final do path — não o caminho completo —
/// para evitar falsos positivos (ex.: "/home/user/builds/relatorio.txt").
fn deve_ignorar(entry: &DirEntry, user_ignore: &[String]) -> bool {
    let name = match entry.file_name().to_str() {
        Some(n) => n,
        None => return false,
    };

    // Só poda diretórios; arquivos individuais não são podados aqui.
    if !entry.file_type().is_dir() {
        return false;
    }

    // Lista padrão (case-insensitive no Windows).
    if DEFAULT_IGNORE.iter().any(|p| name.eq_ignore_ascii_case(p)) {
        return true;
    }

    // Lista do usuário.
    user_ignore
        .iter()
        .any(|p| name.eq_ignore_ascii_case(p.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir_entry_mock(name: &str) -> bool {
        // Testa a lógica de nome diretamente, sem DirEntry real.
        DEFAULT_IGNORE.iter().any(|p| name.eq_ignore_ascii_case(p))
    }

    #[test]
    fn ignora_node_modules_por_padrao() {
        assert!(dir_entry_mock("node_modules"));
    }

    #[test]
    fn ignora_git_por_padrao() {
        assert!(dir_entry_mock(".git"));
    }

    #[test]
    fn ignora_target_por_padrao() {
        assert!(dir_entry_mock("target"));
    }

    #[test]
    fn nao_ignora_diretorio_comum() {
        assert!(!dir_entry_mock("documentos"));
        assert!(!dir_entry_mock("projetos"));
        assert!(!dir_entry_mock("fotos"));
    }

    #[test]
    fn ignora_case_insensitive() {
        assert!(dir_entry_mock("NODE_MODULES"));
        assert!(dir_entry_mock("Target"));
        assert!(dir_entry_mock(".GIT"));
    }

    #[test]
    fn nao_ignora_arquivo_com_nome_de_pasta_proibida() {
        // Arquivos individuais não são podados — apenas diretórios.
        // (Testado indiretamente: deve_ignorar só poda is_dir())
        // Este teste valida a lógica de nome para garantir que
        // "node_modules.txt" não seria afetado se fosse arquivo.
        assert!(!dir_entry_mock("node_modules.txt"));
    }
}
