-- Migração inicial — esquema completo (ver docs/arquitetura/esquema-sql.md)
-- Marco 0: cria todas as tabelas e índices, mesmo que ainda não utilizados.

CREATE TABLE files (
    id            TEXT PRIMARY KEY,
    path          TEXT NOT NULL,
    relative_path TEXT NOT NULL,
    name          TEXT NOT NULL,
    extension     TEXT,
    size          INTEGER,
    hash          TEXT,
    created_at    TEXT,
    modified_at   TEXT,
    indexed_at    TEXT,
    status        TEXT NOT NULL DEFAULT 'discovered'
        CHECK (status IN (
            'discovered',
            'indexed',
            'pending_analysis',
            'analyzed',
            'failed'
        ))
);

CREATE TABLE file_contents (
    id             TEXT PRIMARY KEY,
    file_id        TEXT NOT NULL,
    content        TEXT,
    language       TEXT,
    content_length INTEGER,
    extracted_at   TEXT,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
);

CREATE TABLE entities (
    id         TEXT PRIMARY KEY,
    name       TEXT NOT NULL,
    type       TEXT NOT NULL
        CHECK (type IN (
            'person',
            'organization',
            'project',
            'topic',
            'document'
        )),
    confidence REAL,
    created_at TEXT
);

CREATE TABLE file_entities (
    id                TEXT PRIMARY KEY,
    file_id           TEXT NOT NULL,
    entity_id         TEXT NOT NULL,
    relationship_type TEXT
        CHECK (relationship_type IN (
            'mentions',
            'references',
            'belongs_to'
        )),
    confidence        REAL,
    FOREIGN KEY (file_id)   REFERENCES files(id)    ON DELETE CASCADE,
    FOREIGN KEY (entity_id) REFERENCES entities(id) ON DELETE CASCADE
);

CREATE TABLE relationships (
    id                TEXT PRIMARY KEY,
    source_entity_id  TEXT NOT NULL,
    target_entity_id  TEXT NOT NULL,
    relationship_type TEXT
        CHECK (relationship_type IN (
            'related_to',
            'parent_of',
            'derived_from'
        )),
    confidence        REAL,
    created_at        TEXT,
    FOREIGN KEY (source_entity_id) REFERENCES entities(id) ON DELETE CASCADE,
    FOREIGN KEY (target_entity_id) REFERENCES entities(id) ON DELETE CASCADE
);

CREATE TABLE embeddings (
    id         TEXT PRIMARY KEY,
    file_id    TEXT NOT NULL,
    model      TEXT NOT NULL,
    vector     BLOB NOT NULL,
    created_at TEXT,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
);

CREATE TABLE clusters (
    id          TEXT PRIMARY KEY,
    name        TEXT,
    description TEXT,
    confidence  REAL,
    created_at  TEXT
);

CREATE TABLE cluster_members (
    id         TEXT PRIMARY KEY,
    cluster_id TEXT NOT NULL,
    file_id    TEXT,
    entity_id  TEXT,
    confidence REAL,
    FOREIGN KEY (cluster_id) REFERENCES clusters(id) ON DELETE CASCADE,
    FOREIGN KEY (file_id)    REFERENCES files(id)    ON DELETE CASCADE,
    FOREIGN KEY (entity_id)  REFERENCES entities(id) ON DELETE CASCADE,
    CHECK (file_id IS NOT NULL OR entity_id IS NOT NULL)
);

-- Tabelas de sugestões criadas na migration 0004 (Marco 3).

CREATE TABLE snapshots (
    id           TEXT PRIMARY KEY,
    execution_id TEXT NOT NULL,
    created_at   TEXT,
    description  TEXT
);

CREATE TABLE operations (
    id             TEXT PRIMARY KEY,
    snapshot_id    TEXT NOT NULL,
    execution_id   TEXT NOT NULL,
    sequence       INTEGER NOT NULL,
    operation_type TEXT NOT NULL
        CHECK (operation_type IN (
            'create_directory',
            'move',
            'rename',
            'consolidate'
        )),
    source_path    TEXT,
    target_path    TEXT,
    reversal_state TEXT NOT NULL DEFAULT 'planned'
        CHECK (reversal_state IN (
            'planned',
            'applied',
            'reverted',
            'failed'
        )),
    executed_at    TEXT,
    reverted_at    TEXT,
    FOREIGN KEY (snapshot_id) REFERENCES snapshots(id) ON DELETE CASCADE
);

CREATE INDEX idx_files_hash             ON files(hash);
CREATE INDEX idx_files_path             ON files(path);
CREATE INDEX idx_files_status           ON files(status);
CREATE INDEX idx_entities_name          ON entities(name);
CREATE INDEX idx_file_entities_file     ON file_entities(file_id);
CREATE INDEX idx_file_entities_entity   ON file_entities(entity_id);
CREATE INDEX idx_relationships_source   ON relationships(source_entity_id);
CREATE INDEX idx_relationships_target   ON relationships(target_entity_id);
CREATE INDEX idx_clusters_name          ON clusters(name);
CREATE INDEX idx_snapshots_execution    ON snapshots(execution_id);
CREATE INDEX idx_operations_snapshot    ON operations(snapshot_id);
CREATE INDEX idx_operations_execution   ON operations(execution_id);
