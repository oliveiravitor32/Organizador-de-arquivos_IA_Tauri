//! Testes de integração contra um SQLite real e efêmero
//! (ver docs/requisitos/convencoes-de-teste.md).

use organizador_de_arquivos_tauri_lib::db;

/// CA-2 do Marco 0: a migração cria todas as tabelas do esquema.
#[tokio::test]
async fn migracao_cria_todas_as_tabelas() {
    let tmp = tempfile::tempdir().expect("dir temporário");
    let db_path = tmp.path().join("teste.db");

    let pool = db::create_pool(&db_path).await.expect("cria pool");
    db::run_migrations(&pool).await.expect("aplica migrações");

    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master \
         WHERE type = 'table' \
         AND name NOT LIKE 'sqlite_%' \
         AND name NOT LIKE '_sqlx%'",
    )
    .fetch_all(&pool)
    .await
    .expect("consulta sqlite_master");

    let tabelas: Vec<String> = rows.into_iter().map(|r| r.0).collect();

    let esperadas = [
        "files",
        "file_contents",
        "entities",
        "file_entities",
        "relationships",
        "embeddings",
        "clusters",
        "cluster_members",
        "suggestions",
        "suggestion_operations",
        "snapshots",
        "operations",
    ];

    for tabela in esperadas {
        assert!(
            tabelas.contains(&tabela.to_string()),
            "tabela ausente após a migração: {tabela}"
        );
    }
}
