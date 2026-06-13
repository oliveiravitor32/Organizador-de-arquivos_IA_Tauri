//! Extratores de conteúdo textual de arquivos (ADR-016).
//!
//! Cada extrator recebe um caminho e retorna o texto extraído, ou None
//! quando o formato não suporta extração de conteúdo (FA-001 do UC-002).

pub mod docx;
pub mod pdf;
pub mod texto;
pub mod xlsx;

use std::path::Path;

/// Extrai texto de um arquivo com base na extensão/MIME.
/// Retorna None para formatos sem suporte de conteúdo (imagens, binários).
pub fn extrair(path: &Path) -> Option<String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    match ext.as_deref() {
        // Texto puro e código-fonte — leitura direta.
        Some(
            "txt" | "md" | "markdown" | "json" | "yaml" | "yml" | "xml" | "toml" | "csv" | "ts"
            | "tsx" | "js" | "jsx" | "rs" | "py" | "java" | "cs" | "cpp" | "c" | "h" | "go" | "rb"
            | "php" | "sh" | "sql" | "html" | "css" | "scss",
        ) => texto::extrair(path),

        Some("pdf") => pdf::extrair(path),
        Some("docx") => docx::extrair(path),
        Some("xlsx" | "xls") => xlsx::extrair(path),

        // Imagens e formatos sem suporte de conteúdo — apenas metadados (ADR-017).
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn extrai_texto_de_arquivo_txt() {
        let mut f = NamedTempFile::with_suffix(".txt").unwrap();
        writeln!(f, "conteúdo de teste").unwrap();
        let resultado = extrair(f.path());
        assert!(resultado.is_some());
        assert!(resultado.unwrap().contains("conteúdo de teste"));
    }

    #[test]
    fn retorna_none_para_formato_sem_suporte() {
        let f = NamedTempFile::with_suffix(".png").unwrap();
        let resultado = extrair(f.path());
        assert!(resultado.is_none());
    }
}
