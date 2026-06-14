-- Marco 3 — tabelas de sugestões de organização (UC-005, ADR-025).

CREATE TABLE suggestions (
    id          TEXT PRIMARY KEY,
    tipo        TEXT NOT NULL
        CHECK (tipo IN ('agrupamento', 'mover_arquivo', 'renomear_arquivo', 'criar_pasta')),
    titulo      TEXT,
    descricao   TEXT,
    confianca   REAL,
    status      TEXT NOT NULL DEFAULT 'pendente'
        CHECK (status IN ('pendente', 'aceita', 'rejeitada', 'executada')),
    cluster_id  TEXT REFERENCES clusters(id),
    evidencias  TEXT, -- JSON array de evidências
    criado_em   TEXT NOT NULL
);

-- Operações concretas a executar quando a sugestão for aprovada (Marco 4).
CREATE TABLE suggestion_operations (
    id             TEXT PRIMARY KEY,
    suggestion_id  TEXT NOT NULL REFERENCES suggestions(id) ON DELETE CASCADE,
    tipo_operacao  TEXT NOT NULL,
    payload        TEXT NOT NULL -- JSON com detalhes da operação
);

CREATE INDEX IF NOT EXISTS idx_suggestions_status    ON suggestions(status);
CREATE INDEX IF NOT EXISTS idx_suggestions_cluster   ON suggestions(cluster_id);
CREATE INDEX IF NOT EXISTS idx_sugg_ops_suggestion   ON suggestion_operations(suggestion_id);
