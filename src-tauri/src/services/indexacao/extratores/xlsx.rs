//! Extrator de XLSX/XLS via `calamine` (ADR-016).

use calamine::{open_workbook_auto, Reader};
use std::path::Path;

pub fn extrair(path: &Path) -> Option<String> {
    let mut wb = open_workbook_auto(path).ok()?;
    let mut linhas: Vec<String> = Vec::new();

    for nome in wb.sheet_names().to_owned() {
        if let Ok(range) = wb.worksheet_range(&nome) {
            for row in range.rows() {
                let celulas: Vec<String> = row
                    .iter()
                    .map(|c| c.to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                if !celulas.is_empty() {
                    linhas.push(celulas.join("\t"));
                }
            }
        }
    }

    if linhas.is_empty() {
        None
    } else {
        Some(linhas.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retorna_none_para_xlsx_invalido() {
        let resultado = extrair(Path::new("/inexistente.xlsx"));
        assert!(resultado.is_none());
    }
}
