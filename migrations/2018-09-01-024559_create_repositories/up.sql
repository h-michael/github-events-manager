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

CREATE TABLE IF NOT EXISTS pull_request_event_conditions (
  id            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  repository_id INTEGER NOT NULL,
  open          INTEGER NOT NULL DEFAULT 0 CHECK(open IN (0, 1)),
  closed        INTEGER NOT NULL DEFAULT 0 CHECK(closed IN (0, 1)),
  merged        INTEGER NOT NULL DEFAULT 0 CHECK(merged IN (0, 1))
);

CREATE INDEX IF NOT EXISTS repository_id ON pull_request_event_conditions(repository_id);

CREATE TABLE IF NOT EXISTS issue_event_conditions (
  id            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  repository_id INTEGER NOT NULL,
  open          INTEGER NOT NULL DEFAULT 0 CHECK(open IN (0, 1)),
  closed        INTEGER NOT NULL DEFAULT 0 CHECK(closed IN (0, 1))
);

CREATE INDEX IF NOT EXISTS repository_id ON issue_event_conditions(repository_id);

/**CREATE TABLE IF NOT EXISTS pull_requests (
/**  id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
/**  created_at      TEXT    NOT NULL,
/**  updated_at      TEXT    NOT NULL,
/**  edited_at       TEXT    NOT NULL,
/**  pull_request_id TEXT    NOT NULL UNIQUE,
/**  repository_id   INTEGER NOT NULL,
/**  state           TEXT    NOT NULL CHECK(state IN ("OPEN", "CLOSED", "MERGED")),
/**  title           TEXT,
/**  body            TEXT,
/**)
/**
/**CREATE TABLE IF NOT EXISTS issues (
/**  id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
/**  created_at      TEXT    NOT NULL,
/**  updated_at      TEXT    NOT NULL,
/**  issue_id        TEXT    NOT NULL UNIQUE,
/**  repository_id   INTEGER NOT NULL,
/**  state           TEXT    NOT NULL CHECK(state IN ("OPEN", "CLOSED")),
/**  title           TEXT,
/**  body            TEXT,
/**)
/**
/**CREATE TABLE IF NOT EXISTS comments (
/**  id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
/**  created_at      TEXT    NOT NULL,
/**  updated_at      TEXT    NOT NULL,
/**  comment_id      TEXT    NOT NULL UNIQUE,
/**  resource_type   TEXT    NOT NULL CHECK(resource_type IN ("PULL_REQUEST", "ISSUE")),
/**  repource_id     INTEGER NOT NULL,
/**  title           TEXT,
/**  body            TEXT,
/**)
