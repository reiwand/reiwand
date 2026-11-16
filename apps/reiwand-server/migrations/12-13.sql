ALTER TABLE repos ADD COLUMN private INTEGER NOT NULL DEFAULT 0;
ALTER TABLE repos ADD COLUMN publishing TEXT NOT NULL DEFAULT 'open' CHECK (publishing IN ('open', 'members'));

CREATE TABLE repo_members (
  repo TEXT NOT NULL REFERENCES repos(name) ON DELETE CASCADE ON UPDATE CASCADE,
  user_id INTEGER NOT NULL REFERENCES users ON DELETE CASCADE,
  may_read INTEGER NOT NULL DEFAULT 1,
  may_publish INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY (repo, user_id)
) STRICT, WITHOUT ROWID;

CREATE INDEX repo_members_user ON repo_members(user_id);
