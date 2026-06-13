# Arquitetura: Esquema SQL

## Objetivo

Este documento define o **esquema físico** do banco de dados em SQLite.

Ele deriva do modelo conceitual descrito em `banco-de-dados.md` e o traduz em DDL concreto.

O Grafo de Conhecimento não é armazenado diretamente: é reconstruído a partir destas tabelas (ver ADR-004 e ADR-005).

---

# Convenções

## Identificadores

UUIDs são armazenados como `TEXT`.

---

## Datas

Datas são armazenadas como `TEXT` em formato ISO 8601 (UTC).

---

## Booleanos

Booleanos são armazenados como `INTEGER` (0 ou 1).

---

## Integridade

- Chaves estrangeiras habilitadas (`PRAGMA foreign_keys = ON`).
- Exclusões em cascata apenas onde o filho não tem valor sem o pai.

---

## Migrações

O esquema é versionado por migrações incrementais e numeradas.

A versão atual é registrada via `PRAGMA user_version`.

---

# Configuração Inicial

```sql
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
```

---

# Tabela: files

```sql
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
```

---

# Tabela: file_contents

```sql
CREATE TABLE file_contents (
    id             TEXT PRIMARY KEY,
    file_id        TEXT NOT NULL,
    content        TEXT,
    language       TEXT,
    content_length INTEGER,
    extracted_at   TEXT,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
);
```

---

# Tabela: entities

```sql
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
```

---

# Tabela: file_entities

```sql
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
```

---

# Tabela: relationships

```sql
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
```

---

# Tabela: embeddings

```sql
CREATE TABLE embeddings (
    id         TEXT PRIMARY KEY,
    file_id    TEXT NOT NULL,
    model      TEXT NOT NULL,
    vector     BLOB NOT NULL,
    created_at TEXT,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
);
```

---

# Tabela: clusters

```sql
CREATE TABLE clusters (
    id          TEXT PRIMARY KEY,
    name        TEXT,
    description TEXT,
    confidence  REAL,
    created_at  TEXT
);
```

---

# Tabela: cluster_members

```sql
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
```

---

# Tabela: suggestions

```sql
CREATE TABLE suggestions (
    id         TEXT PRIMARY KEY,
    type       TEXT NOT NULL
        CHECK (type IN (
            'move_file',
            'rename_file',
            'create_folder',
            'merge_cluster'
        )),
    title      TEXT,
    reason     TEXT,
    confidence REAL,
    status     TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN (
            'pending',
            'approved',
            'rejected',
            'executed'
        )),
    created_at TEXT
);
```

---

# Tabela: suggestion_operations

```sql
CREATE TABLE suggestion_operations (
    id             TEXT PRIMARY KEY,
    suggestion_id  TEXT NOT NULL,
    operation_type TEXT NOT NULL,
    payload        TEXT NOT NULL, -- JSON
    FOREIGN KEY (suggestion_id) REFERENCES suggestions(id) ON DELETE CASCADE
);
```

---

# Tabela: snapshots

Snapshot lógico, conforme ADR-010.

```sql
CREATE TABLE snapshots (
    id           TEXT PRIMARY KEY,
    execution_id TEXT NOT NULL,
    created_at   TEXT,
    description  TEXT
);
```

---

# Tabela: operations

Diário de operações reversíveis (ADR-010).

```sql
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
```

---

# Índices

```sql
CREATE INDEX idx_files_hash             ON files(hash);
CREATE INDEX idx_files_path             ON files(path);
CREATE INDEX idx_files_status           ON files(status);

CREATE INDEX idx_entities_name          ON entities(name);

CREATE INDEX idx_file_entities_file     ON file_entities(file_id);
CREATE INDEX idx_file_entities_entity   ON file_entities(entity_id);

CREATE INDEX idx_relationships_source   ON relationships(source_entity_id);
CREATE INDEX idx_relationships_target   ON relationships(target_entity_id);

CREATE INDEX idx_clusters_name          ON clusters(name);

CREATE INDEX idx_suggestions_status     ON suggestions(status);

CREATE INDEX idx_snapshots_execution    ON snapshots(execution_id);
CREATE INDEX idx_operations_snapshot    ON operations(snapshot_id);
CREATE INDEX idx_operations_execution   ON operations(execution_id);
```

---

# Reconstrução do Grafo

O grafo é reconstruído logicamente a partir de:

- files
- entities
- file_entities
- relationships
- clusters
- cluster_members

Nenhuma tabela representa o grafo diretamente.

---

# Evolução Futura

O esquema foi projetado para acomodar, via migrações, sem reescrever o domínio:

- coluna de estado de candidatura em `entities` (entidade candidata)
- tabela dedicada de busca vetorial
- particionamento de `file_contents` para conteúdos extensos
- histórico de versões de embeddings

---

# Observação

Este documento é a fonte única do esquema físico.

O modelo conceitual permanece em `banco-de-dados.md`; divergências de campo devem ser resolvidas em favor deste documento.
