# ADR-003: IA Local como Estratégia Principal

Status: Aceito

## Contexto

O sistema analisa conteúdo potencialmente sensível armazenado pelo usuário.

Enviar arquivos para APIs externas comprometeria privacidade e aumentaria custos.

## Decisão

Utilizar modelos executados localmente como estratégia principal de inferência.

## Justificativa

- Privacidade.
- Funcionamento offline.
- Custo operacional zero.
- Maior controle sobre os dados.

## Consequências

Positivas:

- Nenhuma dependência obrigatória de nuvem.
- Melhor aceitação por usuários preocupados com privacidade.

Negativas:

- Inferência limitada pelo hardware local.
- Necessidade de otimizações para máquinas mais simples.
