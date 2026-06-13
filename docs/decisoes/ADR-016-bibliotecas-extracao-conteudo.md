# ADR-016 — Bibliotecas de Extração de Conteúdo (M1)

## Status

Aceito

## Contexto

O UC-002 (Indexar Arquivos) exige extração de texto de múltiplos formatos: PDF, DOCX, XLSX, além de texto puro, Markdown, JSON, código-fonte e imagens. A extração deve ser desacoplada da IA (ADR-006) e funcionar offline.

Três abordagens foram avaliadas para os formatos binários (PDF/DOCX/XLSX):

- **A** — `pdf-extract` + `docx-rs` + `calamine`: Rust puro, sem dependências nativas.
- **B** — `lopdf` + `docx-rs` + `calamine`: mais controle sobre PDF, mas extração de texto manual.
- **C** — `pdfium-render` + `docx-rs` + `calamine`: qualidade profissional, mas binário PDFium nativo (~30 MB) e CI complexo.

## Decisão

Adotar **Opção A**: `pdf-extract` + `docx-rs` + `calamine`.

| Formato | Biblioteca |
| --- | --- |
| PDF | `pdf-extract` |
| DOCX | `docx-rs` |
| XLSX | `calamine` |
| TXT / MD / JSON / YAML / XML / código | leitura direta (`std::fs`) |
| PNG / JPG | metadados apenas (OCR adiado para M6 — ver ADR-017) |

## Consequências

- Sem dependências nativas; build CI simples.
- PDFs com layout complexo (colunas, tabelas) podem ter extração imperfeita. Aceitável no M1 — o conteúdo é insumo para a IA, não exibição fiel.
- Limitações de `pdf-extract` em PDFs complexos serão endereçadas no M6 (robustez), se necessário.
- Formatos não suportados são indexados apenas com metadados (FA-001 do UC-002 — comportamento correto).
