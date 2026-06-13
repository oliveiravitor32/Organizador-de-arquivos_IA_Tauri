-- Marco 1: adiciona colunas necessárias para descoberta e indexação.

ALTER TABLE files ADD COLUMN scan_id   TEXT;
ALTER TABLE files ADD COLUMN mime_type TEXT;

-- UNIQUE em path: necessário para o upsert ON CONFLICT(path) no FileRepository.
CREATE UNIQUE INDEX idx_files_path_unique ON files(path);

-- Garante unicidade de conteúdo por arquivo (necessário para ON CONFLICT upsert).
CREATE UNIQUE INDEX idx_file_contents_file_id ON file_contents(file_id);

CREATE INDEX idx_files_scan_id ON files(scan_id);
