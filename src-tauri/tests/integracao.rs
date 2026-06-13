//! Testes de integração contra um SQLite real e efêmero
//! (ver docs/requisitos/convencoes-de-teste.md).

use std::io::Write;

use organizador_de_arquivos_tauri_lib::db;
use organizador_de_arquivos_tauri_lib::db::repositories::files::FileRepository;
use organizador_de_arquivos_tauri_lib::domain::arquivo::{FileStatus, NewFile};
use organizador_de_arquivos_tauri_lib::services::indexacao::extratores;

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

async fn pool_teste() -> sqlx::SqlitePool {
    let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
    db::run_migrations(&pool).await.unwrap();
    pool
}

/// UC-001 CA-004: arquivos descobertos são persistidos com status DISCOVERED.
#[tokio::test]
async fn arquivos_descobertos_sao_persistidos() {
    let pool = pool_teste().await;
    let repo = FileRepository::new(&pool);

    let file = NewFile {
        scan_id: "scan-1".to_string(),
        path: "/tmp/doc.txt".to_string(),
        relative_path: "doc.txt".to_string(),
        name: "doc.txt".to_string(),
        extension: Some("txt".to_string()),
    };

    repo.upsert_discovered(&file).await.unwrap();

    let status: String = sqlx::query_scalar("SELECT status FROM files WHERE path = '/tmp/doc.txt'")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(status, "discovered");
}

/// UC-002 CA-001: metadados são persistidos corretamente após indexação.
#[tokio::test]
async fn metadados_sao_persistidos_apos_indexacao() {
    let pool = pool_teste().await;
    let repo = FileRepository::new(&pool);

    let file = NewFile {
        scan_id: "scan-1".to_string(),
        path: "/tmp/meta.txt".to_string(),
        relative_path: "meta.txt".to_string(),
        name: "meta.txt".to_string(),
        extension: Some("txt".to_string()),
    };

    let id = repo.upsert_discovered(&file).await.unwrap();
    repo.update_metadata(&id, 1024, "abc123", "text/plain")
        .await
        .unwrap();
    repo.update_status(&id, FileStatus::Indexed).await.unwrap();

    let row: (i64, String, String) =
        sqlx::query_as("SELECT size, hash, mime_type FROM files WHERE id = ?")
            .bind(&id)
            .fetch_one(&pool)
            .await
            .unwrap();

    assert_eq!(row.0, 1024);
    assert_eq!(row.1, "abc123");
    assert_eq!(row.2, "text/plain");
}

/// UC-002 CA-005: após indexação completa, status é PENDING_ANALYSIS.
#[tokio::test]
async fn status_pending_analysis_apos_indexacao() {
    let pool = pool_teste().await;
    let repo = FileRepository::new(&pool);

    let file = NewFile {
        scan_id: "scan-1".to_string(),
        path: "/tmp/pending.txt".to_string(),
        relative_path: "pending.txt".to_string(),
        name: "pending.txt".to_string(),
        extension: Some("txt".to_string()),
    };

    let id = repo.upsert_discovered(&file).await.unwrap();
    repo.update_metadata(&id, 512, "hash-x", "text/plain")
        .await
        .unwrap();
    repo.update_status(&id, FileStatus::Indexed).await.unwrap();
    repo.update_status(&id, FileStatus::PendingAnalysis)
        .await
        .unwrap();

    let status: String = sqlx::query_scalar("SELECT status FROM files WHERE id = ?")
        .bind(&id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(status, "pending_analysis");
}

/// UC-002 CA-002: conteúdo extraído de arquivo TXT é persistido.
#[tokio::test]
async fn conteudo_txt_e_extraido_e_persistido() {
    let pool = pool_teste().await;
    let repo = FileRepository::new(&pool);

    // Cria um arquivo temporário real.
    let mut tmp = tempfile::NamedTempFile::with_suffix(".txt").unwrap();
    writeln!(tmp, "conteúdo de integração").unwrap();

    let path = tmp.path();
    let texto = extratores::extrair(path);
    assert!(texto.is_some(), "extrator deve retornar conteúdo para TXT");

    let file = NewFile {
        scan_id: "scan-1".to_string(),
        path: path.to_string_lossy().to_string(),
        relative_path: "tmp.txt".to_string(),
        name: "tmp.txt".to_string(),
        extension: Some("txt".to_string()),
    };

    let id = repo.upsert_discovered(&file).await.unwrap();
    repo.upsert_content(
        &organizador_de_arquivos_tauri_lib::domain::arquivo::FileContent {
            file_id: id.clone(),
            content: texto.unwrap(),
            language: None,
            content_length: 22,
        },
    )
    .await
    .unwrap();

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM file_contents WHERE file_id = ?")
        .bind(&id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(count, 1);
}

/// UC-002 CA-003: arquivo de formato não suportado é indexado apenas com metadados.
#[tokio::test]
async fn formato_nao_suportado_indexado_apenas_com_metadados() {
    let tmp = tempfile::NamedTempFile::with_suffix(".png").unwrap();
    let resultado = extratores::extrair(tmp.path());
    assert!(resultado.is_none(), "PNG não deve ter conteúdo extraído");
}

/// UC-002 CA-004: falha em arquivo individual não impede processamento dos demais.
#[tokio::test]
async fn falha_individual_nao_impede_demais() {
    let pool = pool_teste().await;
    let repo = FileRepository::new(&pool);

    // Arquivo válido.
    let file_ok = NewFile {
        scan_id: "scan-2".to_string(),
        path: "/tmp/ok.txt".to_string(),
        relative_path: "ok.txt".to_string(),
        name: "ok.txt".to_string(),
        extension: Some("txt".to_string()),
    };
    let id_ok = repo.upsert_discovered(&file_ok).await.unwrap();

    // Arquivo com falha.
    let file_fail = NewFile {
        scan_id: "scan-2".to_string(),
        path: "/tmp/fail.txt".to_string(),
        relative_path: "fail.txt".to_string(),
        name: "fail.txt".to_string(),
        extension: Some("txt".to_string()),
    };
    let id_fail = repo.upsert_discovered(&file_fail).await.unwrap();
    repo.update_status(&id_fail, FileStatus::Failed)
        .await
        .unwrap();

    // O arquivo OK pode continuar sendo atualizado normalmente.
    repo.update_status(&id_ok, FileStatus::Indexed)
        .await
        .unwrap();

    let status_ok: String = sqlx::query_scalar("SELECT status FROM files WHERE id = ?")
        .bind(&id_ok)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status_ok, "indexed");
}
