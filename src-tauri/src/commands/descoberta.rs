//! Commands do Marco 1 — Descoberta (UC-001, UC-002).

use std::path::PathBuf;

use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

use crate::core::state::AppState;
use crate::error::{AppError, AppResult};
use crate::services::indexacao::{indexing::IndexingService, scan::ScanService};

/// Inicia o escaneamento recursivo de um diretório (UC-001).
///
/// Retorna imediatamente um `scanId`; o progresso chega por eventos.
#[tauri::command]
pub async fn escanear_diretorio(
    app: AppHandle,
    state: State<'_, AppState>,
    root_path: String,
    ignore: Vec<String>,
) -> AppResult<serde_json::Value> {
    let path = PathBuf::from(&root_path);

    if !path.exists() || !path.is_dir() {
        return Err(AppError::Internal("caminho_invalido".to_string()));
    }

    let scan_id = Uuid::new_v4().to_string();
    let cancel_rx = state.register_cancel(&scan_id);
    let pool = state.db.clone();

    let app_clone = app.clone();
    let scan_id_clone = scan_id.clone();

    tauri::async_runtime::spawn(async move {
        let service = ScanService::new(app_clone.clone(), pool);
        let _ = service
            .escanear(&scan_id_clone, &path, &ignore, cancel_rx)
            .await;
        if let Some(s) = app_clone.try_state::<AppState>() {
            s.remove_cancel(&scan_id_clone);
        }
    });

    Ok(serde_json::json!({ "scanId": scan_id }))
}

/// Inicia a indexação dos arquivos descobertos em um scan (UC-002).
///
/// Retorna imediatamente um `indexingId`; o progresso chega por eventos.
#[tauri::command]
pub async fn indexar_arquivos(
    app: AppHandle,
    state: State<'_, AppState>,
    scan_id: String,
) -> AppResult<serde_json::Value> {
    let indexing_id = Uuid::new_v4().to_string();
    let cancel_rx = state.register_cancel(&indexing_id);
    let pool = state.db.clone();

    let app_clone = app.clone();
    let indexing_id_clone = indexing_id.clone();

    tauri::async_runtime::spawn(async move {
        let service = IndexingService::new(app_clone.clone(), pool);
        let _ = service
            .indexar(&indexing_id_clone, &scan_id, cancel_rx)
            .await;
        if let Some(s) = app_clone.try_state::<AppState>() {
            s.remove_cancel(&indexing_id_clone);
        }
    });

    Ok(serde_json::json!({ "indexingId": indexing_id }))
}

/// Cancela uma operação assíncrona em andamento.
#[tauri::command]
pub fn cancelar_operacao(
    state: State<'_, AppState>,
    operation_id: String,
) -> AppResult<serde_json::Value> {
    let cancelou = state.cancel(&operation_id);
    Ok(serde_json::json!({
        "operationId": operation_id,
        "status": if cancelou { "cancelando" } else { "nao_encontrada" }
    }))
}

/// Retorna o resultado de uma indexação concluída, se disponível (CA-HMR-001).
///
/// Permite que o frontend recupere o payload de `indexing://completed` mesmo
/// que o evento tenha sido perdido por hot-reload ou reconexão.
/// O resultado é consumido na primeira consulta (evita acúmulo de memória).
#[tauri::command]
pub async fn consultar_indexacao(
    state: State<'_, AppState>,
    indexing_id: String,
) -> AppResult<Option<serde_json::Value>> {
    Ok(state.take_resultado(&indexing_id))
}
