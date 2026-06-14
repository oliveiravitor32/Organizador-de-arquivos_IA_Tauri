# ADR-025 — Algoritmo do Motor de Sugestões (Marco 3)

**Status:** Aceito  
**Data:** 2026-06-14  
**Marco:** M3 — Inteligência

---

## Contexto

UC-005 define que o Motor de Sugestões deve analisar o Grafo de Conhecimento
e identificar oportunidades de organização. UC-012 exige que toda sugestão
possua justificativa e evidências.

Ao final do Marco 2, o sistema produz:
- **Embeddings** por arquivo (nomic-embed-text, 768 dimensões)
- **Clusters** semânticos (union-find, threshold 0,75)

Entidades e relações não são populadas no Marco 2 (ADR-023), portanto o motor
do Marco 3 opera exclusivamente sobre embeddings e clusters.

---

## Decisão

O motor de sugestões do Marco 3 adota uma abordagem em **três etapas**:

### Etapa 1 — Detecção de oportunidades (determinístico)

Para cada cluster com `confiança ≥ 0,50` (ADR-024) e `≥ 2 arquivos membros`:
- Se os arquivos estão em **diretórios diferentes** → oportunidade de
  `agrupamento` identificada.
- Confiança da sugestão = `cluster.confidence`.
- Evidências calculadas:
  - número de arquivos no cluster
  - similaridade média dos pares
  - lista de diretórios distintos representados

Tipo único implementado no Marco 3: **`agrupamento`**.  
Outros tipos (separação, renomeação, estruturação, consolidação) ficam para
marcos futuros quando entidades e relações forem populadas.

### Etapa 2 — Nomeação via LLM (1 chamada por cluster)

O LLM recebe apenas os nomes dos arquivos do cluster, com cap de **15 arquivos**:
- Se o cluster tiver ≤ 15 membros: todos são enviados.
- Se tiver > 15: os **15 arquivos mais próximos do centróide semântico** são
  selecionados. O centróide é calculado como a média dos vetores de embedding
  dos membros; cada arquivo recebe um score de similaridade cosseno com o
  centróide e os 15 de maior score são enviados.

Prompt enviado ao LLM:
```
Você recebe nomes de arquivos de um grupo semântico (mostrando até 15 de N total).
Gere um nome curto (máximo 5 palavras) que descreva o tema comum deste grupo.
Responda apenas com o nome, sem explicações.

Arquivos:
- relatorio_alpha.pdf
- cronograma_alpha.xlsx
- apresentacao_alpha.pptx
```

**Por que cap de 15?**  
200 nomes × ~40 chars = ~8.000 chars de input, aumentando latência e
reduzindo qualidade do resultado em modelos menores. 15 nomes representativos
produzem resultado equivalente com input ~10× menor.

O LLM devolve o `titulo` da sugestão. Se o LLM estiver indisponível,
o sistema usa fallback por template: `"Grupo semântico com N arquivos"`.

Constante nomeada no código:
```rust
pub const MAX_FILES_LLM_NAMING: usize = 15;
```

### Etapa 3 — Justificativa por template (determinístico)

A justificativa textual é gerada sem LLM:

```
"N arquivos com similaridade semântica média de X foram identificados
no mesmo agrupamento e estão distribuídos em Y diretórios distintos."
```

---

## Alternativas consideradas

| Alternativa | Motivo da rejeição |
|---|---|
| LLM gera toda a justificativa | Latência adicional por sugestão; templates são suficientes e rastreáveis |
| Título por frequência de palavras nos nomes | Heurística frágil; LLM produz resultado melhor com mesma quantidade de chamadas |
| Tipos adicionais (separação, renomeação) | Inviáveis sem entidades/relações; adiados para marcos futuros |
| LLM por arquivo para detectar oportunidades | Inviável — mesmo problema de latência do Marco 2 (ADR-023) |

---

## Consequências

- O `ServicoIa::gerar_nome_cluster(nomes_arquivos)` será adicionado ao trait.
- O sistema funciona **sem Ollama**: nomeação cai para template, sugestões
  continuam sendo geradas (apenas com título genérico).
- Quando entidades forem retomadas (Marco futuro), o motor pode ser estendido
  sem quebrar a interface atual.
- Apenas `agrupamento` é suportado no Marco 3; o campo `tipo` no banco já
  admite outros valores para expansão futura.

---

## Schema resultante (migration 0004)

Segue `docs/arquitetura/esquema-sql.md` (fonte única da verdade para o banco):

```sql
CREATE TABLE suggestions (
    id          TEXT PRIMARY KEY,
    tipo        TEXT NOT NULL CHECK (tipo IN ('agrupamento', 'mover_arquivo', 'renomear_arquivo', 'criar_pasta')),
    titulo      TEXT,
    descricao   TEXT,
    confianca   REAL,
    status      TEXT NOT NULL DEFAULT 'pendente'
                CHECK (status IN ('pendente', 'aceita', 'rejeitada', 'executada')),
    cluster_id  TEXT REFERENCES clusters(id),
    evidencias  TEXT,   -- JSON array inline
    criado_em   TEXT
);

-- Operações concretas a executar na aprovação (usada no Marco 4)
CREATE TABLE suggestion_operations (
    id             TEXT PRIMARY KEY,
    suggestion_id  TEXT NOT NULL REFERENCES suggestions(id) ON DELETE CASCADE,
    tipo_operacao  TEXT NOT NULL,
    payload        TEXT NOT NULL  -- JSON
);
```

Marco 3 popula `suggestions` com `tipo = 'agrupamento'` e registra em
`suggestion_operations` as movimentações propostas (para Marco 4 executar).

---

## Referências

- UC-005 — Gerar Sugestões
- UC-012 — Explicar Sugestões
- ADR-009 — Motor de Sugestões Orientado a Conhecimento
- ADR-023 — LLM usado somente na geração de sugestões
- ADR-024 — Limiar de confiança mínima para sugestões
