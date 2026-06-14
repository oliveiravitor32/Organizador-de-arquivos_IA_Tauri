//! Commands do Marco 2 — Conhecimento (UC-003).

use std::sync::Arc;

use tauri::{AppHandle, Emitter, State};
use uuid::Uuid;

use crate::core::state::AppState;
use crate::error::AppResult;
use crate::services::conhecimento::analise::AnaliseService;
use crate::services::ia::ollama::{OllamaService, DEFAULT_EMBED_MODEL};

/// Inicia o pipeline de análise semântica dos arquivos com status pending_analysis (UC-003).
///
/// Retorna imediatamente um `analysisId`; o progresso chega por eventos.
#[tauri::command]
pub async fn analisar_arquivos(
    app: AppHandle,
    state: State<'_, AppState>,
    file_ids: Option<Vec<String>>,
) -> AppResult<serde_json::Value> {
    let analysis_id = Uuid::new_v4().to_string();
    let pool = state.db.clone();
    let app_clone = app.clone();
    let analysis_id_clone = analysis_id.clone();

    tauri::async_runtime::spawn(async move {
        let ia: Arc<dyn crate::services::ia::ServicoIa> = Arc::new(OllamaService::default());
        let svc = AnaliseService::new(pool, ia, DEFAULT_EMBED_MODEL);
        let result = svc.analisar(&app_clone, file_ids, &analysis_id_clone).await;
        if let Err(e) = result {
            let _ = app_clone.emit(
                "analysis://failed",
                serde_json::json!({ "analysisId": analysis_id_clone, "error": e.to_string() }),
            );
        }
    });

    Ok(serde_json::json!({ "analysisId": analysis_id }))
}
