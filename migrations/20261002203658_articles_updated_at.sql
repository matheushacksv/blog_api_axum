-- Add migration script here
ALTER TABLE articles
ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT now();

UPDATE articles SET updated_at = created_at;
