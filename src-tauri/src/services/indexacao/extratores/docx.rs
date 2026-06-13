//! Extrator de DOCX via `docx-rs` (ADR-016).

use std::path::Path;

pub fn extrair(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    let docx = docx_rs::read_docx(&bytes).ok()?;

    let mut partes: Vec<String> = Vec::new();

    for child in &docx.document.children {
        if let docx_rs::DocumentChild::Paragraph(p) = child {
            let linha: String = p
                .children
                .iter()
                .filter_map(|c| {
                    if let docx_rs::ParagraphChild::Run(r) = c {
                        let texto: String = r
                            .children
                            .iter()
                            .filter_map(|rc| {
                                if let docx_rs::RunChild::Text(t) = rc {
                                    Some(t.text.clone())
                                } else {
                                    None
                                }
                            })
                            .collect();
                        Some(texto)
                    } else {
                        None
                    }
                })
                .collect();
            if !linha.trim().is_empty() {
                partes.push(linha);
            }
        }
    }

    if partes.is_empty() {
        None
    } else {
        Some(partes.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retorna_none_para_docx_invalido() {
        let resultado = extrair(Path::new("/inexistente.docx"));
        assert!(resultado.is_none());
    }
}
