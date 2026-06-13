//! Extrator de texto puro e código-fonte.

use std::path::Path;

pub fn extrair(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn extrai_conteudo_de_txt() {
        let mut f = NamedTempFile::with_suffix(".txt").unwrap();
        writeln!(f, "linha um\nlinha dois").unwrap();
        let texto = extrair(f.path()).unwrap();
        assert!(texto.contains("linha um"));
    }

    #[test]
    fn retorna_none_para_arquivo_invalido() {
        let resultado = extrair(Path::new("/caminho/inexistente.txt"));
        assert!(resultado.is_none());
    }
}
