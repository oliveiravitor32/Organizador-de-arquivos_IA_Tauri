//! Extrator de PDF via `pdf-extract` (ADR-016).

use std::path::Path;

pub fn extrair(path: &Path) -> Option<String> {
    pdf_extract::extract_text(path).ok().map(|t| normalizar(&t))
}

fn normalizar(texto: &str) -> String {
    texto
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retorna_none_para_pdf_invalido() {
        let resultado = extrair(Path::new("/inexistente.pdf"));
        assert!(resultado.is_none());
    }

    #[test]
    fn normalizar_remove_linhas_vazias_e_espacos() {
        let entrada = "  linha um  \n\n  linha dois  \n   \n  linha três  ";
        let saida = normalizar(entrada);
        assert_eq!(saida, "linha um\nlinha dois\nlinha três");
    }

    #[test]
    fn normalizar_retorna_string_vazia_para_so_espacos() {
        let saida = normalizar("   \n\n   ");
        assert_eq!(saida, "");
    }

    #[test]
    fn normalizar_preserva_conteudo_de_linha_unica() {
        let saida = normalizar("texto único");
        assert_eq!(saida, "texto único");
    }
}
