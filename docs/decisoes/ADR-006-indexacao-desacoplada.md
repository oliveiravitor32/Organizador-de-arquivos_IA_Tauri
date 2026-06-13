# ADR-006: Indexação Independente da IA

Status: Aceito

## Contexto

A indexação é responsável por transformar arquivos físicos em dados estruturados.

A dependência da IA durante a indexação tornaria o processo mais lento e menos resiliente.

## Decisão

Separar indexação e enriquecimento semântico.

Primeiro:

- Descoberta
- Metadados
- Conteúdo

Depois:

- Entidades
- Embeddings
- Relações

## Justificativa

- Maior robustez.
- Possibilidade de reprocessamento.
- Melhor experiência do usuário.

## Consequências

Positivas:

- Sistema continua funcionando sem IA.

Negativas:

- Necessidade de pipeline adicional.
