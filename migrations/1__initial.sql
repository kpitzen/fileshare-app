CREATE TABLE IF NOT EXISTS files
(
    id SERIAL PRIMARY KEY NOT NULL,
    filename TEXT,
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    file_object BYTEA
);
ALTER TABLE files ALTER COLUMN file_object SET STORAGE EXTENDED;
