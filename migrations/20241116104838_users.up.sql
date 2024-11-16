-- CITEXT Email Domain (HTML5-compatible)

CREATE EXTENSION IF NOT EXISTS citext;
CREATE DOMAIN email
    AS citext
    CHECK (value ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$');

-- User Table

CREATE TABLE users (
	id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
	email email UNIQUE NOT NULL,
	password_hash text NOT NULL,
	created_at timestamptz DEFAULT now() NOT NULL,
	updated_at timestamptz DEFAULT now() NOT NULL
);

-- UpdatedAt Trigger

CREATE TRIGGER users_updated_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Snippets User Relation

ALTER TABLE snippets
ADD COLUMN user_id uuid
REFERENCES users(id);

-- User Deletion Snippet Archival Function

CREATE OR REPLACE FUNCTION archive_user_snippets(input_user_id UUID)
RETURNS VOID AS $$
BEGIN
    UPDATE snippets
    SET archived = true, user_id = NULL
    WHERE user_id = input_user_id AND archived = false;
END;
$$ LANGUAGE 'plpgsql';

-- Auth Token Tables

CREATE TABLE access_tokens (
	id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
	user_id uuid REFERENCES users(id) ON DELETE CASCADE NOT NULL,
	jwt text UNIQUE NOT NULL,
	expires_at timestamptz DEFAULT now() NOT NULL
);

CREATE TABLE refresh_tokens (
	id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
	user_id uuid REFERENCES users(id) ON DELETE CASCADE NOT NULL,
	access_token_id uuid UNIQUE REFERENCES access_tokens(id) ON DELETE CASCADE NOT NULL,
	jwt text UNIQUE NOT NULL,
	expires_at timestamptz DEFAULT now() NOT NULL
);
