# ADR-017 — OCR para Imagens Adiado para M6

## Status

Aceito

## Contexto

O UC-002 lista PNG/JPG como formatos suportados e menciona que "OCR é opcional nesta etapa". As alternativas avaliadas para o M1 foram:

- **A** — Adiar OCR para M6: imagens indexadas apenas com metadados no M1.
- **B** — `tesseract-rs` agora: OCR no M1 com Tesseract, mas requer instalação do Tesseract no SO do usuário.
- **C** — Alternativa embutida: nenhuma lib Rust com OCR de qualidade sem dependência nativa existe hoje (2025).

## Decisão

**Adiar OCR para M6** (Opção A).

No M1, imagens (PNG/JPG/JPEG) são indexadas apenas com metadados (tamanho, hash, datas, tipo MIME). Nenhum texto é extraído.

O OCR entra no Marco 6 (Robustez e Acabamento), onde a configuração `ocrHabilitado` já está prevista no contrato `obter_configuracao` / `atualizar_configuracao`.

## Consequências

- Zero dependência de sistema no M1; instalação da aplicação permanece simples.
- Arquivos de imagem com texto não terão conteúdo extraído até o M6.
- O extrator de imagens no M1 deve implementar o FA-001 do UC-002 (formato não suportado para conteúdo → indexar só metadados), o que é o comportamento correto.
- A decisão sobre qual engine OCR usar (Tesseract, alternativa embutida) fica para o Gate de Marco 6.
