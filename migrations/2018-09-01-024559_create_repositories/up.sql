-- Your SQL goes here
CREATE TABLE repositories (
  id            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  repository_id TEXT    NOT NULL UNIQUE,
  owner         TEXT    NOT NULL,
  name          TEXT    NOT NULL,
  url           TEXT    NOT NULL UNIQUE
);

CREATE INDEX repository_id ON repositories(repository_id);
CREATE UNIQUE INDEX owner_and_name ON repositories(owner, name);

/** 2bit flag **/
/** 1 => open, 2 => assign, 4 => review request, 8 => closed, 16 => merged **/
/** 1 => merged, 2 => closed **/
CREATE TABLE pull_request_event_conditions (
  id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  repository_id   INTEGER NOT NULL UNIQUE,
  start_condition INTEGER NOT NULL DEFAULT 1 CHECK(start_condition > 0 AND start_condition < 32),
  stop_condition  INTEGER NOT NULL DEFAULT 1 CHECK(stop_condition > 0 AND stop_condition < 4)
);

/** 2bit flag **/
/** 1 => open, 2 => assign, 4 => closed **/
/** 1 => closed **/
CREATE TABLE issue_event_conditions (
  id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  repository_id   INTEGER NOT NULL UNIQUE,
  start_condition INTEGER NOT NULL DEFAULT 1 CHECK(start_condition > 0 AND start_condition < 8),
  stop_condition  INTEGER NOT NULL DEFAULT 1 CHECK(stop_condition > 0 AND stop_condition < 2)
);

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
