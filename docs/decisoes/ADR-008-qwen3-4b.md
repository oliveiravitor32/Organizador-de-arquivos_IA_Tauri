# ADR-008: Qwen 3 4B como Modelo Inicial

Status: Aceito

## Contexto

O hardware alvo inicial inclui máquinas com aproximadamente 8 GB de RAM.

O projeto necessita executar inferência local.

## Decisão

Adotar Qwen 3 4B como modelo padrão inicial.

Execução via Ollama.

## Justificativa

- Compatível com hardware modesto.
- Boa capacidade de raciocínio.
- Baixo consumo de memória.
- Boa relação desempenho/qualidade.

## Consequências

Positivas:

- Maior acessibilidade.
- Menor barreira para adoção.

Negativas:

- Inferência menos poderosa que modelos maiores.
- Algumas análises complexas podem exigir modelos alternativos.
