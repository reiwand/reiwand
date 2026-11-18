CREATE TABLE users_new (
  id INTEGER PRIMARY KEY,
  handle TEXT NOT NULL UNIQUE COLLATE NOCASE,
  password_hash TEXT,
  role TEXT NOT NULL CHECK (role IN ('user', 'trusted', 'moderator', 'admin')),
  status TEXT NOT NULL CHECK (status IN ('active', 'suspended')),
  quota_bytes INTEGER NOT NULL,
  created_at INTEGER NOT NULL
) STRICT;
INSERT INTO users_new (id, handle, password_hash, role, status, quota_bytes, created_at) SELECT id, handle, password_hash, role, status, quota_bytes, created_at FROM users;
DROP TABLE users;
ALTER TABLE users_new RENAME TO users;
CREATE TABLE device_codes_new (
  code_hash BLOB PRIMARY KEY,
  user_code TEXT NOT NULL UNIQUE,
  platform TEXT NOT NULL,
  user_id INTEGER REFERENCES users ON DELETE CASCADE,
  state TEXT NOT NULL CHECK (state IN ('pending', 'approved', 'denied')),
  interval_s INTEGER NOT NULL,
  polled_at INTEGER NOT NULL,
  expires_at INTEGER NOT NULL
, client TEXT NOT NULL, created_at INTEGER NOT NULL) STRICT, WITHOUT ROWID;
INSERT INTO device_codes_new (code_hash, user_code, platform, user_id, state, interval_s, polled_at, expires_at, client, created_at) SELECT code_hash, user_code, platform, user_id, state, interval_s, polled_at, expires_at, client, created_at FROM device_codes;
DROP TABLE device_codes;
ALTER TABLE device_codes_new RENAME TO device_codes;
CREATE TABLE repos_new (
  n INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  folder TEXT NOT NULL UNIQUE,
  online_key TEXT NOT NULL,
  repo_id TEXT,
  dirty INTEGER NOT NULL,
  signed_at INTEGER NOT NULL,
  failed_at INTEGER,
  failed_raw TEXT,
  created_at INTEGER NOT NULL,
  private INTEGER NOT NULL,
  publishing TEXT NOT NULL CHECK (publishing IN ('open', 'members'))
) STRICT;
INSERT INTO repos_new (n, name, folder, online_key, repo_id, dirty, signed_at, failed_at, failed_raw, created_at, private, publishing) SELECT n, name, folder, online_key, repo_id, dirty, signed_at, failed_at, failed_raw, created_at, private, publishing FROM repos;
DROP TABLE repos;
ALTER TABLE repos_new RENAME TO repos;
CREATE TABLE repo_members_new (
  repo TEXT NOT NULL REFERENCES repos(name) ON DELETE CASCADE ON UPDATE CASCADE,
  user_id INTEGER NOT NULL REFERENCES users ON DELETE CASCADE,
  may_read INTEGER NOT NULL,
  may_publish INTEGER NOT NULL,
  PRIMARY KEY (repo, user_id)
) STRICT, WITHOUT ROWID;
INSERT INTO repo_members_new (repo, user_id, may_read, may_publish) SELECT repo, user_id, may_read, may_publish FROM repo_members;
DROP TABLE repo_members;
ALTER TABLE repo_members_new RENAME TO repo_members;
CREATE INDEX repo_members_user ON repo_members(user_id);
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
  facets TEXT NOT NULL,
  listed INTEGER NOT NULL,
  signer TEXT,
  asks TEXT NOT NULL,
  band_sheet TEXT,
  UNIQUE (repo, package_id, version),
  FOREIGN KEY (repo, package_id) REFERENCES packages
) STRICT;
INSERT INTO versions_new (id, repo, package_id, version, sha256, size, kind, title, description, license, tags, authors, state, uploaded_by, created_at, published_at, content, photosensitive, facets, listed, signer, asks, band_sheet) SELECT id, repo, package_id, version, sha256, size, kind, title, description, license, tags, authors, state, uploaded_by, created_at, published_at, content, photosensitive, facets, listed, signer, asks, band_sheet FROM versions;
DROP TABLE versions;
ALTER TABLE versions_new RENAME TO versions;
CREATE INDEX versions_feed ON versions(state, published_at DESC, id DESC);
CREATE INDEX versions_uploader ON versions(uploaded_by);
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
CREATE TABLE uploads_new (
  id TEXT PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users ON DELETE CASCADE,
  size INTEGER NOT NULL,
  sha256 TEXT NOT NULL,
  state TEXT NOT NULL CHECK (state IN ('open', 'received', 'failed', 'done')),
  errors TEXT,
  version_id INTEGER REFERENCES versions ON DELETE SET NULL,
  created_at INTEGER NOT NULL,
  expires_at INTEGER NOT NULL
, listed INTEGER NOT NULL, repo TEXT NOT NULL) STRICT, WITHOUT ROWID;
INSERT INTO uploads_new (id, user_id, size, sha256, state, errors, version_id, created_at, expires_at, listed, repo) SELECT id, user_id, size, sha256, state, errors, version_id, created_at, expires_at, listed, repo FROM uploads;
DROP TABLE uploads;
ALTER TABLE uploads_new RENAME TO uploads;
CREATE INDEX uploads_user ON uploads(user_id, created_at);
CREATE TABLE cases_new (
  id INTEGER PRIMARY KEY,
  version_id INTEGER NOT NULL REFERENCES versions ON DELETE CASCADE,
  reason TEXT NOT NULL CHECK (reason IN ('upload', 'report', 'notice', 'counter')),
  detail TEXT NOT NULL,
  priority INTEGER NOT NULL,
  state TEXT NOT NULL CHECK (state IN ('open', 'actioned', 'dismissed')),
  decision TEXT,
  statement TEXT,
  decided_by INTEGER REFERENCES users ON DELETE SET NULL,
  created_at INTEGER NOT NULL,
  decided_at INTEGER
) STRICT;
INSERT INTO cases_new (id, version_id, reason, detail, priority, state, decision, statement, decided_by, created_at, decided_at) SELECT id, version_id, reason, detail, priority, state, decision, statement, decided_by, created_at, decided_at FROM cases;
DROP TABLE cases;
ALTER TABLE cases_new RENAME TO cases;
CREATE INDEX cases_queue ON cases(state, priority DESC, created_at);
CREATE TABLE audit_new (
  id INTEGER PRIMARY KEY,
  at INTEGER NOT NULL,
  actor_id INTEGER,
  action TEXT NOT NULL,
  subject TEXT NOT NULL,
  detail TEXT NOT NULL
) STRICT;
INSERT INTO audit_new (id, at, actor_id, action, subject, detail) SELECT id, at, actor_id, action, subject, detail FROM audit;
DROP TABLE audit;
ALTER TABLE audit_new RENAME TO audit;
CREATE INDEX audit_at ON audit(at);
CREATE TABLE mail_queue_new (
  id INTEGER PRIMARY KEY,
  topic TEXT NOT NULL CHECK (topic IN ('case', 'health', 'test')),
  case_id INTEGER REFERENCES cases(id) ON DELETE SET NULL,
  subject TEXT NOT NULL,
  body TEXT NOT NULL,
  state TEXT NOT NULL CHECK (state IN ('queued', 'sent', 'failed')),
  tries INTEGER NOT NULL,
  next_at INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  sent_at INTEGER
) STRICT;
INSERT INTO mail_queue_new (id, topic, case_id, subject, body, state, tries, next_at, created_at, sent_at) SELECT id, topic, case_id, subject, body, state, tries, next_at, created_at, sent_at FROM mail_queue;
DROP TABLE mail_queue;
ALTER TABLE mail_queue_new RENAME TO mail_queue;
CREATE INDEX mail_queue_due ON mail_queue (state, next_at);
CREATE INDEX mail_queue_case ON mail_queue (case_id);
