# ADR-005: SQLite como Persistência Principal

Status: Aceito

## Contexto

O sistema necessita persistir arquivos, entidades, embeddings, relacionamentos e sugestões.

## Decisão

Utilizar SQLite como banco principal.

O grafo será reconstruído logicamente a partir dos dados persistidos.

## Justificativa

- Local-first.
- Sem necessidade de servidor.
- Fácil distribuição.
- Excelente integração com Tauri.

## Consequências

Positivas:

- Menor complexidade.
- Menor consumo de recursos.

Negativas:

- Algumas consultas complexas exigirão processamento adicional.
