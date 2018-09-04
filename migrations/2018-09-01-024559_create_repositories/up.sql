-- Your SQL goes here
CREATE TABLE IF NOT EXISTS repositories (
  id            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  repository_id TEXT    NOT NULL UNIQUE,
  owner         TEXT    NOT NULL,
  name          TEXT    NOT NULL,
  url           TEXT    NOT NULL UNIQUE
);

CREATE INDEX repository_id ON repositories(repository_id);
CREATE UNIQUE INDEX owner_and_name ON repositories(owner, name);
