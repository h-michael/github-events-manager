-- Your SQL goes here
CREATE TABLE repositories (
  id                INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  node_id           TEXT    NOT NULL UNIQUE,
  owner             TEXT    NOT NULL,
  name              TEXT    NOT NULL,
  url               TEXT    NOT NULL UNIQUE,
  last_pr_cursor    TEXT             UNIQUE,
  last_issue_cursor TEXT             UNIQUE
);

CREATE UNIQUE INDEX node_id ON repositories(node_id);
CREATE UNIQUE INDEX owner_and_name ON repositories(owner, name);

/** 2bit flag **/
/** 1 => open, 2 => assign, 4 => review request, 8 => closed, 16 => merged **/
/** 1 => merged, 2 => closed **/
CREATE TABLE pull_request_event_conditions (
  id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  repository_id   INTEGER NOT NULL UNIQUE,
  start_condition INTEGER NOT NULL DEFAULT 1 CHECK(start_condition > 0 AND start_condition < 32 AND start_condition != stop_condition),
  stop_condition  INTEGER NOT NULL DEFAULT 1 CHECK(stop_condition > 0 AND stop_condition < 4 AND stop_condition != start_condition),
  listen_status   INTEGER NOT NULL DEFAULT 1 CHECK(listen_status IN (0, 1))
);
CREATE UNIQUE INDEX pr_req_event_cond_repository_id ON pull_request_event_conditions(repository_id);

/** 2bit flag **/
/** 1 => open, 2 => assign, 4 => closed **/
/** 1 => closed **/
CREATE TABLE issue_event_conditions (
  id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  repository_id   INTEGER NOT NULL UNIQUE,
  start_condition INTEGER NOT NULL DEFAULT 1 CHECK(start_condition > 0 AND start_condition < 8 AND start_condition != stop_condition),
  stop_condition  INTEGER NOT NULL DEFAULT 1 CHECK(stop_condition > 0 AND stop_condition < 2 AND stop_condition != start_condition),
  listen_status   INTEGER NOT NULL DEFAULT 1 CHECK(listen_status IN (0, 1))
);
CREATE UNIQUE INDEX issue_event_cond_repository_id ON issue_event_conditions(repository_id);

CREATE TABLE IF NOT EXISTS pull_requests (
  id                        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  created_at                TEXT    NOT NULL,
  updated_at                TEXT    NOT NULL,
  edited_at                 TEXT,
  closed_at                 TEXT,
  merged_at                 TEXT,
  node_id                   TEXT    NOT NULL UNIQUE,
  number                    INTEGER NOT NULL,
  repository_id             INTEGER NOT NULL,
  state                     TEXT    NOT NULL CHECK(state IN ("OPEN", "CLOSED", "MERGED")),
  title                     TEXT    NOT NULL,
  body_text                 TEXT    NOT NULL,
  closed                    INTEGER NOT NULL CHECK(closed IN (0, 1)),
  merged                    INTEGER NOT NULL CHECK(merged IN (0, 1)),
  last_event_cursor         TEXT
);
CREATE INDEX pr_req_repository_id ON pull_requests(repository_id);

CREATE TABLE IF NOT EXISTS issues (
  id                INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  created_at        TEXT    NOT NULL,
  updated_at        TEXT    NOT NULL,
  edited_at         TEXT,
  closed_at         TEXT,
  node_id           TEXT    NOT NULL UNIQUE,
  number            INTEGER NOT NULL,
  repository_id     INTEGER NOT NULL,
  state             TEXT    NOT NULL CHECK(state IN ("OPEN", "CLOSED")),
  title             TEXT,
  body_text         TEXT    NOT NULL,
  closed            INTEGER NOT NULL CHECK(closed IN (0, 1)),
  last_event_cursor TEXT
);
CREATE INDEX issues_repository_id ON issues(repository_id);

CREATE TABLE IF NOT EXISTS commits (
  id                INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  resource_id       INTEGER NOT NULL,
  node_id           TEXT    NOT NULL UNIQUE,
  oid               TEXT    NOT NULL,
  message_body      TEXT    NOT NULL,
  message_headline  TEXT    NOT NULL,
  commit_url        TEXT    NOT NULL,
  committed_date    TEXT    NOT NULL,
  pushed_date       TEXT
);

CREATE TABLE IF NOT EXISTS comments (
  id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  created_at      TEXT    NOT NULL,
  last_edited_at  TEXT,
  published_at    TEXT,
  resource_id     INTEGER NOT NULL,
  resource_type   TEXT    NOT NULL CHECK(resource_type IN ("PULL_REQUEST", "ISSUE")),
  node_id         TEXT    NOT NULL UNIQUE,
  body_text       TEXT
);

/** CREATE TABLE IF NOT EXISTS closed_events (
/**   id                INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
/**   resource_id       INTEGER NOT NULL,
/**   node_id           TEXT    NOT NULL UNIQUE,
/**   actor_id          INTEGER NOT NULL,
/**   url               TEXT    NOT NULL,
/**   resource_path     TEXT    NOT NULL,
/**   created_at        TEXT    NOT NULL
/** );

/** CREATE TABLE IF NOT EXISTS actors (
/** 
/** );
/** 
/** CREATE TABLE IF NOT EXISTS (
/** 
/** );

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
