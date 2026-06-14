//! Serviço de indexação de arquivos descobertos (UC-002).

use std::path::Path;
use std::time::Instant;

use sha2::{Digest, Sha256};
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::watch;

use crate::db::repositories::files::FileRepository;
use crate::domain::arquivo::{FileContent, FileStatus};
use crate::domain::scan::{IndexingProgress, IndexingStats};
use crate::error::{AppError, AppResult};
use crate::events;
use crate::services::indexacao::extratores;

pub struct IndexingService {
    app: AppHandle,
    pool: sqlx::SqlitePool,
}

impl IndexingService {
    pub fn new(app: AppHandle, pool: sqlx::SqlitePool) -> Self {
        Self { app, pool }
    }

    /// Indexa todos os arquivos com status DISCOVERED para o scan_id dado.
    pub async fn indexar(
        &self,
        indexing_id: &str,
        scan_id: &str,
        cancel_rx: watch::Receiver<bool>,
    ) -> AppResult<IndexingStats> {
        let inicio = Instant::now();
        let repo = FileRepository::new(&self.pool);

        self.app
            .emit(
                events::INDEXING_STARTED,
                serde_json::json!({ "indexingId": indexing_id, "scanId": scan_id }),
            )
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let arquivos = repo.find_discovered_by_scan(scan_id).await?;
        let total = arquivos.len() as u64;
        let mut stats = IndexingStats::default();

        for arquivo in &arquivos {
            if *cancel_rx.borrow() {
                self.app
                    .emit(
                        events::INDEXING_CANCELLED,
                        serde_json::json!({ "indexingId": indexing_id }),
                    )
                    .ok();
                break;
            }

            self.app
                .emit(
                    events::INDEXING_FILE_STARTED,
                    serde_json::json!({ "indexingId": indexing_id, "fileId": arquivo.id }),
                )
                .ok();

            let path = Path::new(&arquivo.path);
            match self.processar_arquivo(&repo, &arquivo.id, path).await {
                Ok(_) => stats.processed += 1,
                Err(_) => {
                    stats.failed += 1;
                    let _ = repo.update_status(&arquivo.id, FileStatus::Failed).await;
                }
            }

            let progress = IndexingProgress {
                indexing_id: indexing_id.to_string(),
                processed: stats.processed + stats.failed,
                total,
                current_file: arquivo.path.clone(),
            };
            self.app.emit(events::INDEXING_PROGRESS, &progress).ok();
        }

        stats.duration_ms = inicio.elapsed().as_millis() as u64;

        let payload = serde_json::json!({
            "indexingId": indexing_id,
            "processados": stats.processed,
            "ignorados": stats.skipped,
            "falhos": stats.failed,
            "durationMs": stats.duration_ms,
        });

        // Persiste antes de emitir — garante que o frontend pode consultar
        // o resultado mesmo que o evento seja perdido por hot-reload (CA-HMR-001).
        if let Some(state) = self.app.try_state::<crate::core::state::AppState>() {
            state.store_resultado(indexing_id, payload.clone());
        }

        self.app.emit(events::INDEXING_COMPLETED, payload).ok();

        Ok(stats)
    }

    async fn processar_arquivo(
        &self,
        repo: &FileRepository<'_>,
        file_id: &str,
        path: &Path,
    ) -> AppResult<()> {
        // Lê os bytes para hash e tamanho.
        let bytes = std::fs::read(path).map_err(|e| AppError::Internal(e.to_string()))?;
        let size = bytes.len() as i64;

        // SHA-256 streaming sobre os bytes já lidos.
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let hash = format!("{:x}", hasher.finalize());

        // MIME type por extensão.
        let mime = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        repo.update_metadata(file_id, size, &hash, &mime).await?;

        // Extrai conteúdo textual (None para formatos não suportados — FA-001).
        if let Some(texto) = extratores::extrair(path) {
            let content_length = texto.chars().count() as i64;
            repo.upsert_content(&FileContent {
                file_id: file_id.to_string(),
                content: texto,
                language: None,
                content_length,
            })
            .await?;
        }

        // INDEXED → PENDING_ANALYSIS (UC-002 passos 9 e 10).
        repo.update_status(file_id, FileStatus::Indexed).await?;
        repo.update_status(file_id, FileStatus::PendingAnalysis)
            .await?;

        Ok(())
    }
}
