# ADR-002: Tauri como Plataforma Desktop

Status: Aceito

## Contexto

O sistema necessita acesso ao sistema de arquivos, execução local de IA e distribuição multiplataforma.

As alternativas consideradas foram:

- Tauri
- Electron
- Aplicação nativa completa

## Decisão

Utilizar Tauri.

## Justificativa

- Menor consumo de memória.
- Binários menores.
- Backend em Rust.
- Excelente integração com React.

## Consequências

Positivas:

- Aplicação leve.
- Melhor desempenho.

Negativas:

- Ecossistema menor que Electron.
- Necessidade de conhecimento em Rust.
