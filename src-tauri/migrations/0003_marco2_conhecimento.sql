-- Marco 2 — índices únicos necessários para upserts do pipeline de IA.
-- As tabelas já existem desde a migração 0001.

-- Entidade única por nome+tipo (deduplicação — UC-008 CA-002).
CREATE UNIQUE INDEX IF NOT EXISTS idx_entities_name_type
    ON entities(name, type);

-- Um link arquivo→entidade por par (idempotência).
CREATE UNIQUE INDEX IF NOT EXISTS idx_file_entities_pair
    ON file_entities(file_id, entity_id);

-- Um embedding por arquivo por modelo (ADR-020).
CREATE UNIQUE INDEX IF NOT EXISTS idx_embeddings_file_model
    ON embeddings(file_id, model);

-- Relação única por par de entidades + tipo (deduplicação — UC-010 CA-002).
CREATE UNIQUE INDEX IF NOT EXISTS idx_relationships_pair_type
    ON relationships(source_entity_id, target_entity_id, relationship_type);

-- Membro único por cluster+arquivo e cluster+entidade.
CREATE UNIQUE INDEX IF NOT EXISTS idx_cluster_members_file
    ON cluster_members(cluster_id, file_id) WHERE file_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_cluster_members_entity
    ON cluster_members(cluster_id, entity_id) WHERE entity_id IS NOT NULL;

-- Índice de suporte para carregar todos os embeddings de um arquivo.
CREATE INDEX IF NOT EXISTS idx_embeddings_file_id
    ON embeddings(file_id);

-- Cluster único por nome (deduplicação no upsert_cluster).
CREATE UNIQUE INDEX IF NOT EXISTS idx_clusters_name_unique
    ON clusters(name);
