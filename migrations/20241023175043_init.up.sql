-- Utility Function

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = NOW();
   RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

-- Snippet Variants Enum

CREATE TYPE snippet_variant AS ENUM ('TEXT', 'URL');

-- Snippet Tables

CREATE TABLE snippets (
	id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
	variant snippet_variant NOT NULL,
    archived bool DEFAULT false NOT NULL,
	created_at timestamptz DEFAULT now() NOT NULL,
	updated_at timestamptz DEFAULT now() NOT NULL
);

CREATE TABLE snippets_data_text (
    snippet_id uuid PRIMARY KEY,
    text TEXT NOT NULL,

    CONSTRAINT fk_snippets_data_text_snippet_id FOREIGN KEY (snippet_id)
        REFERENCES snippets(id) ON DELETE CASCADE
);

CREATE TABLE snippets_data_url (
    snippet_id uuid PRIMARY KEY,
    url TEXT NOT NULL,

    CONSTRAINT fk_snippets_data_url_snippet_id FOREIGN KEY (snippet_id)
        REFERENCES snippets(id) ON DELETE CASCADE
);

-- UpdatedAt Trigger

CREATE TRIGGER snippets_updated_at
BEFORE UPDATE ON snippets
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Soft-Deletion Trigger

CREATE RULE snippets_archive
AS ON DELETE TO snippets
DO INSTEAD (
    UPDATE snippets
    SET archived = true
    WHERE id = old.id AND NOT archived
);
