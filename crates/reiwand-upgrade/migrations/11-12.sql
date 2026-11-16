CREATE TABLE repos (
  n INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  folder TEXT NOT NULL UNIQUE,
  online_key TEXT NOT NULL,
  repo_id TEXT,
  dirty INTEGER NOT NULL DEFAULT 1,
  signed_at INTEGER NOT NULL DEFAULT 0,
  failed_at INTEGER,
  failed_raw TEXT,
  created_at INTEGER NOT NULL
) STRICT;

INSERT INTO repos (name, folder, online_key, dirty, signed_at, failed_at, failed_raw, created_at)
  SELECT 'main', 'repos/main', 'keys/main.online.key', dirty, signed_at, failed_at, failed_raw, 0 FROM repo_state;

DROP TABLE repo_state;

CREATE TABLE packages_new (
  repo TEXT NOT NULL,
  id TEXT NOT NULL,
  owner_id INTEGER REFERENCES users ON DELETE SET NULL,
  created_at INTEGER NOT NULL,
  hidden TEXT,
  hidden_at INTEGER,
  PRIMARY KEY (repo, id)
) STRICT, WITHOUT ROWID;

INSERT INTO packages_new (repo, id, owner_id, created_at, hidden, hidden_at)
  SELECT 'main', id, owner_id, created_at, hidden, hidden_at FROM packages;

CREATE TABLE versions_new (
  id INTEGER PRIMARY KEY,
  repo TEXT NOT NULL,
  package_id TEXT NOT NULL,
  version TEXT NOT NULL,
  sha256 TEXT NOT NULL,
  size INTEGER NOT NULL,
  kind TEXT NOT NULL,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  license TEXT NOT NULL,
  tags TEXT NOT NULL,
  authors TEXT NOT NULL,
  state TEXT NOT NULL CHECK (state IN ('pending', 'published', 'rejected', 'removed', 'revoked')),
  uploaded_by INTEGER REFERENCES users ON DELETE SET NULL,
  created_at INTEGER NOT NULL,
  published_at INTEGER,
  content TEXT,
  photosensitive INTEGER,
  facets TEXT NOT NULL DEFAULT '[]',
  listed INTEGER NOT NULL DEFAULT 1,
  signer TEXT,
  asks TEXT NOT NULL DEFAULT '[]',
  band_sheet TEXT,
  UNIQUE (repo, package_id, version),
  FOREIGN KEY (repo, package_id) REFERENCES packages
) STRICT;

INSERT INTO versions_new (id, repo, package_id, version, sha256, size, kind, title, description, license, tags, authors,
    state, uploaded_by, created_at, published_at, content, photosensitive, facets, listed, signer, asks, band_sheet)
  SELECT id, 'main', package_id, version, sha256, size, kind, title, description, license, tags, authors,
    state, uploaded_by, created_at, published_at, content, photosensitive, facets, listed, signer, asks, band_sheet
  FROM versions;

CREATE TABLE votes_new (
  repo TEXT NOT NULL,
  package_id TEXT NOT NULL,
  user_id INTEGER NOT NULL REFERENCES users ON DELETE CASCADE,
  value INTEGER NOT NULL CHECK (value IN (-1, 1)),
  at INTEGER NOT NULL,
  PRIMARY KEY (repo, package_id, user_id),
  FOREIGN KEY (repo, package_id) REFERENCES packages ON DELETE CASCADE
) STRICT, WITHOUT ROWID;

INSERT INTO votes_new (repo, package_id, user_id, value, at)
  SELECT 'main', package_id, user_id, value, at FROM votes;

CREATE TABLE featured_new (
  repo TEXT NOT NULL,
  package_id TEXT NOT NULL,
  note TEXT,
  at INTEGER NOT NULL,
  PRIMARY KEY (repo, package_id),
  FOREIGN KEY (repo, package_id) REFERENCES packages ON DELETE CASCADE
) STRICT, WITHOUT ROWID;

INSERT INTO featured_new (repo, package_id, note, at)
  SELECT 'main', package_id, note, at FROM featured;

DROP TABLE votes;

DROP TABLE featured;

DROP TABLE versions;

DROP TABLE packages;

ALTER TABLE packages_new RENAME TO packages;

ALTER TABLE versions_new RENAME TO versions;

ALTER TABLE votes_new RENAME TO votes;

ALTER TABLE featured_new RENAME TO featured;

CREATE INDEX versions_feed ON versions(state, published_at DESC, id DESC);

CREATE INDEX versions_uploader ON versions(uploaded_by);

CREATE INDEX votes_user ON votes(user_id, at);

ALTER TABLE uploads ADD COLUMN repo TEXT NOT NULL DEFAULT '';

CREATE TRIGGER search_added AFTER INSERT ON versions WHEN new.state = 'published' BEGIN
  INSERT INTO search (rowid, package, title, description, tags)
    VALUES (new.id, new.package_id, new.title, new.description, new.tags);
END;

CREATE TRIGGER search_changed AFTER UPDATE ON versions BEGIN
  DELETE FROM search WHERE rowid = old.id;
  INSERT INTO search (rowid, package, title, description, tags)
    SELECT new.id, new.package_id, new.title, new.description, new.tags WHERE new.state = 'published';
END;

CREATE TRIGGER search_removed AFTER DELETE ON versions BEGIN
  DELETE FROM search WHERE rowid = old.id;
END;
