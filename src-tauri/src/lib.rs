pub mod commands;
pub mod core;
pub mod db;
pub mod error;
pub mod events;

use tauri::Manager;

use crate::core::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();

            // Diretório de dados da aplicação (criado se necessário).
            let data_dir = handle
                .path()
                .app_data_dir()
                .expect("não foi possível resolver o diretório de dados");
            std::fs::create_dir_all(&data_dir).ok();
            let db_path = data_dir.join("organizador.db");

            // Inicializa o banco e aplica as migrações.
            let pool = tauri::async_runtime::block_on(async move {
                let pool = db::create_pool(&db_path)
                    .await
                    .expect("falha ao abrir o banco de dados");
                db::run_migrations(&pool)
                    .await
                    .expect("falha ao aplicar migrações");
                pool
            });

            app.manage(AppState { db: pool });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::ping,
            commands::announce_ready
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
