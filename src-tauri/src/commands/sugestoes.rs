//! Commands do Marco 3 — Sugestões de organização (UC-005, UC-012).

use std::sync::Arc;

use tauri::{AppHandle, State};
use uuid::Uuid;

use crate::core::state::AppState;
use crate::db::repositories::files::FileRepository;
use crate::db::repositories::sugestoes::SugestoesRepository;
use crate::error::{AppError, AppResult};
use crate::events;
use crate::services::ia::ollama::OllamaService;
use crate::services::sugestoes::motor::SugestaoMotorService;

/// Inicia a geração de sugestões a partir dos clusters existentes (UC-005).
///
/// Retorna imediatamente um `suggestionGenerationId`; o progresso chega por eventos.
#[tauri::command]
pub async fn gerar_sugestoes(
    app: AppHandle,
    state: State<'_, AppState>,
) -> AppResult<serde_json::Value> {
    let generation_id = Uuid::new_v4().to_string();
    let pool = state.db.clone();
    let gen_id_clone = generation_id.clone();

    tauri::async_runtime::spawn(async move {
        let ia: Arc<dyn crate::services::ia::ServicoIa> = Arc::new(OllamaService::default());
        let svc = SugestaoMotorService::new(pool, ia);
        let result = svc.processar(&app, &gen_id_clone).await;
        if let Err(e) = result {
            let _ = tauri::Emitter::emit(
                &app,
                events::SUGGESTION_FAILED,
                serde_json::json!({ "suggestionGenerationId": gen_id_clone, "error": e.to_string() }),
            );
        }
    });

    Ok(serde_json::json!({ "suggestionGenerationId": generation_id }))
}

/// Retorna a explicação detalhada de uma sugestão (UC-012).
#[tauri::command]
pub async fn explicar_sugestao(
    state: State<'_, AppState>,
    suggestion_id: String,
) -> AppResult<serde_json::Value> {
    let repo = SugestoesRepository::new(&state.db);

    let sugestao = repo
        .find_suggestion_by_id(&suggestion_id)
        .await?
        .ok_or_else(|| AppError::Internal(format!("sugestão {suggestion_id} não encontrada")))?;

    let operacoes = repo.find_operations_by_suggestion(&suggestion_id).await?;

    // Extrai file_ids dos payloads das operações e busca dados de cada arquivo
    let files_repo = FileRepository::new(&state.db);
    let mut arquivos: Vec<serde_json::Value> = Vec::new();
    let mut ids_vistos: std::collections::HashSet<String> = std::collections::HashSet::new();

    for op in &operacoes {
        if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&op.payload) {
            if let Some(fids) = payload.get("file_ids").and_then(|v| v.as_array()) {
                for fid_val in fids {
                    if let Some(fid) = fid_val.as_str() {
                        if !ids_vistos.insert(fid.to_string()) {
                            continue;
                        }
                        if let Some(f) = files_repo.find_by_id(fid).await? {
                            arquivos.push(serde_json::json!({
                                "id": f.id,
                                "nome": f.name,
                                "caminho": f.path,
                            }));
                        }
                    }
                }
            }
        }
    }

    // Evidências armazenadas inline como JSON
    let evidencias: Vec<serde_json::Value> = sugestao
        .evidencias
        .as_deref()
        .and_then(|e| serde_json::from_str(e).ok())
        .unwrap_or_default();

    // Justificativa é a descrição da sugestão
    let justificativa = sugestao
        .descricao
        .clone()
        .unwrap_or_else(|| "Nenhuma justificativa disponível.".to_string());

    let confianca = sugestao.confianca.unwrap_or(0.0);

    // Sinaliza desatualizada se o cluster_id não existe mais
    let desatualizada = if let Some(cid) = &sugestao.cluster_id {
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM clusters WHERE id = ?")
                .bind(cid)
                .fetch_one(&state.db)
                .await
                .unwrap_or(0);
        count == 0
    } else {
        false
    };

    Ok(serde_json::json!({
        "suggestionId": suggestion_id,
        "tipo": sugestao.tipo,
        "titulo": sugestao.titulo,
        "justificativa": justificativa,
        "evidencias": evidencias,
        "confianca": confianca,
        "desatualizada": desatualizada,
        "operacoes": operacoes.len(),
        "arquivos": arquivos,
    }))
}

/// Lista sugestões, opcionalmente filtradas por status (UC-005 CA-004).
#[tauri::command]
pub async fn listar_sugestoes(
    state: State<'_, AppState>,
    status: Option<String>,
) -> AppResult<serde_json::Value> {
    let repo = SugestoesRepository::new(&state.db);
    let sugestoes = repo.find_suggestions(status.as_deref()).await?;

    let lista: Vec<serde_json::Value> = sugestoes
        .iter()
        .map(|s| {
            serde_json::json!({
                "id": s.id,
                "tipo": s.tipo,
                "titulo": s.titulo,
                "confianca": s.confianca,
                "status": s.status,
            })
        })
        .collect();

    Ok(serde_json::json!({ "sugestoes": lista }))
}

// ─── Testes ───────────────────────────────────────────────────────────────────
// Os commands dependem do contexto Tauri (State<AppState>) — testados via
// repositório diretamente (lógica de negócio coberta em sugestoes.rs).

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use crate::db::repositories::sugestoes::SugestoesRepository;

    async fn setup() -> sqlx::SqlitePool {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn explicar_sugestao_logica_evidencias_e_justificativa() {
        let pool = setup().await;
        let repo = SugestoesRepository::new(&pool);

        let evidencias_json = r#"[{"tipo":"arquivos_no_cluster","valor":"3"}]"#;
        let id = repo
            .insert_suggestion(
                "agrupamento",
                Some("Grupo Fiscal"),
                Some("3 arquivos com similaridade 0.88"),
                0.88,
                None,
                Some(evidencias_json),
            )
            .await
            .unwrap();

        let s = repo.find_suggestion_by_id(&id).await.unwrap().unwrap();

        // Verifica que justificativa e evidências são recuperáveis
        assert!(!s.descricao.unwrap_or_default().is_empty());
        let ev: Vec<serde_json::Value> =
            serde_json::from_str(s.evidencias.unwrap().as_str()).unwrap();
        assert_eq!(ev.len(), 1);
        assert_eq!(ev[0]["tipo"], "arquivos_no_cluster");
    }

    #[tokio::test]
    async fn listar_sugestoes_filtra_por_status() {
        let pool = setup().await;
        let repo = SugestoesRepository::new(&pool);

        repo.insert_suggestion("agrupamento", Some("A"), None, 0.9, None, None)
            .await
            .unwrap();

        let pendentes = repo.find_suggestions(Some("pendente")).await.unwrap();
        assert_eq!(pendentes.len(), 1);

        let aceitas = repo.find_suggestions(Some("aceita")).await.unwrap();
        assert!(aceitas.is_empty());
    }
}
