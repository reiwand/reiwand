# Moderation and administration

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/v1/mod/queue` | moderator | Open cases, highest priority first: `{items: [{id, reason, detail, priority, created_at, package, version, title, state, sha256, uploader}]}` |
| POST | `/api/v1/mod/cases/{id}/decision` | moderator | `{decision, statement, reason?, strike?}` |
| GET, POST | `/api/v1/mod/blocklist` | moderator | List; add `{sha256, reason}` |
| DELETE | `/api/v1/mod/blocklist/{sha256}` | moderator | Remove |
| GET | `/api/v1/mod/hidden` | moderator | Hidden packages: `{items: [{repo, id, reason, hidden_at}]}, every repository` |
| POST | `/api/v1/r/{repo}/mod/hidden` | moderator | `{id, reason}` (at least 10 characters) hides a package → `{id, hidden: true}` |
| DELETE | `/api/v1/r/{repo}/mod/hidden/{id}` | moderator | Shows it again → `{id, hidden: false}` |
| POST | `/api/v1/r/{repo}/mod/removed` | moderator | `{id, statement}` (at least 10 characters) withdraws every published version (below) → `{id, removed: ["id@version"]}`; `404` without a published version |
| DELETE | `/api/v1/r/{repo}/admin/packages/{id}` | admin | `{confirm: true}` deletes a package (below) → `{id, deleted: true}` |
| GET | `/api/v1/mod/audit` | admin | Last 500 actions |
| GET, POST | `/api/v1/admin/invites` | admin | List; create `{role, uses?, days?}` → `201 {id, code, url, …}` |
| DELETE | `/api/v1/admin/invites/{id}` | admin | Revoke |
| GET | `/api/v1/admin/users` | admin | Accounts |
| PATCH | `/api/v1/admin/users/{id}` | admin | `{role?, status?: active\|suspended, quota_mib?}` |
| GET, PATCH | `/api/v1/admin/settings` | admin | Stored settings `{registration, auto_publish, network, setup_done, public_url}`; change `{registration?, auto_publish?}` |
| GET | `/api/v1/admin/repo` | admin | Repository state |
| POST | `/api/v1/admin/repo/publish` | admin | Sign now → `{entry_version, expires, packages}` |
| GET, POST | `/api/v1/r/{repo}/admin/featured` | admin | List; feature `{id, note?}` (a published package, at most 64, a note of at most 200 characters on one line) |
| DELETE | `/api/v1/r/{repo}/admin/featured/{id}` | admin | Stop featuring it; both are audited, signed into the repository (`repo.md`) and listed on `/admin/featured` |
| GET | `/api/v1/admin/transparency` | admin | CSV counts of notices, cases and decisions, without personal data |

Decisions: `approve` (pending → published), `reject` (pending → rejected, the file and its
thumbnail are deleted), `remove` (published → removed: revoked with reason `takedown`, downloads
answer `410`), `revoke` (published → revoked with `reason` `malware`, `license`, `takedown`,
`broken` or `other`), `restore` (removed or revoked → published again, for a counter-notice:
the revocation is withdrawn with `reiwand_format::unrevoke`) and `dismiss`. Removing or revoking
moves the package file, and the thumbnail unless another pending or published version uses it,
out of the public tree into `held/`, kept for `REIWAND_TAKEDOWN_DAYS` (also in backups); `restore`
answers `409 deleted` after that. Hiding takes every version of a package out of the catalog, its search and the web site's lists,
without revoking anything: `/packages/{id}` and the package API still answer with the reason, the
repository keeps serving the files and marks the releases `hidden` in the second-version shards, so
clients leave the package out of browsing unless it is installed and keep installing and updating
it for whoever has it. Versions uploaded later stay hidden. `/mod` lists hidden packages and hides
or shows them, `reiwand-ctl moderate --server URL hide ID --reason …`, `unhide ID` and `hidden` do the
same with a personal token that has the `mod` scope (`REIWAND_SERVER_TOKEN`). Every change is audited.
Nobody approves, restores or dismisses a case of their own
upload (`403`); restrictive decisions on one's own upload are allowed. Restrictive decisions need a `statement` of at least 10
characters, which the uploader sees in `/api/v1/me/packages`. `strike: true` adds a strike;
three strikes within a year suspend uploads. Every decision is audited. Approving or rejecting a
pending version answers `409 superseded` once a newer version of the package was uploaded;
dismissing such a case is still accepted (it only closes the case, the version stays pending). Cases come from
uploads (priority 0), reports (1), copyright notices and counter-notices (2) and notices of
illegal content (3).

### Deleting

Three actions take a package away for good, and one a whole repository. Each needs `confirm: true`
in the request (a form sends it from the coral button of its question; without it the answer is
`422` and nothing changes), each is audited (`withdraw`, `delete-package`, `repo-delete`) and none
can be undone.

- **An author withdraws** one of their packages (`DELETE /api/v1/r/{repo}/me/packages/{id}`), and
  **an administrator deletes** any package (`DELETE /api/v1/r/{repo}/admin/packages/{id}`). Both do
  the same: every published version is revoked in the signed index with reason `other` and the
  message «Withdrawn by its author.» or «Deleted by the administrator of this server.», and becomes
  `removed`; pending versions become `rejected`; open cases of its versions are closed with the
  decision `withdraw` or `delete-package`; it stops being featured; its votes are deleted; its files are
  deleted at once from the tree, `blobs/`, `held/` and `thumbs/` (a file another pending or
  published version still uses stays). Nothing is kept for a counter-notice, so `restore` answers
  `409 deleted`. The `packages` row stays, so the id still belongs to its owner (who may upload a
  newer version) and every version number used stays used. Its files answer `410`, its page and
  package API `404`.
- **A moderator withdraws from the repository** (`POST /api/v1/r/{repo}/mod/removed`): for each
  published version the server opens a case (reason `report`, the detail «Removed by a moderator
  from the page of the package.») and decides `remove` on it with the statement, exactly as from
  `/mod`: revoked with reason `takedown`, the files kept in `held/` for `REIWAND_TAKEDOWN_DAYS`, and
  restorable from that case after a counter-notice. The author reads the statement in
  `/api/v1/me/packages`.
- **An administrator deletes a repository** (`DELETE /api/v1/admin/repos/{repo}`): its rows
  (repository, members, packages, versions with their cases, votes, featured and uploads, open
  ones included, with their files in `tmp/`) and its tree `repos/<name>/`, online key and kept root
  key are deleted, and the files in `blobs/`, `held/` and `thumbs/` that no version of another
  repository uses. An upload of that repository still being checked is refused (`409`) when it
  finishes. Then `/r/<name>/…` answers `404` and the repository leaves the discovery document and
  `/`. Backups made before keep their copy.

What clients see (`docs/specs/repo.md`): a revocation reaches every client at its next refresh,
through the next signed entry, and a revoked installed package is never applied again; nothing
else can take a package off a device, so deleting a package always revokes it first. The files of
the tree are served as immutable, so a cache in front of the server may keep a deleted `.wpak` for
a while, but clients resolve every release against the verified index and refuse a revoked one
wherever it comes from. A deleted repository publishes nothing more: clients keep what they
installed (as with any repository that cannot be reached), see its last entry go stale when it
expires, and install nothing new from it. A repository created later under the same name gets a
new root, which those clients refuse because every refresh must chain from the root they anchored
(`repo_id`). Mirrors are listed in the root by the operator and the server cannot reach them: they
keep serving their copy until the operator removes it there, and clients that fall back to them
still see the revocations it holds and its entry go stale. So the question before deleting a
repository says that devices keep what they installed, and to take a package off devices it must
be withdrawn before its repository is deleted.

On the page of a package (`/r/<name>/packages/{id}`) a section «Manage» appears for whoever may
act there: the author sees «Withdraw this wallpaper for good»; a moderator sees «Catalog» (listed,
or out of the catalog with its reason) with «Withdraw from the catalog» (a reason, the hiding
above) or «Return to the catalog», and «Withdraw from the repository» (a statement); an
administrator also sees «Delete this wallpaper». «Withdraw…» and «Delete…» are coral words that
lead to the question (`?ask=withdraw` or `?ask=delete`): the row lights, says what is lost and what
happens on devices, and offers the coral button and «Cancel». `/admin/repos` does the same under
each repository with «Delete this repository…» (`/admin/repos?delete=<name>`).

### Mail

The server sends mail only after the administrator sets it up in the fold of the health row
«Mail» (`POST /admin/mail`, administrators only; anyone else is sent to `/login` and nothing is
stored). While it is off nothing is sent and nothing is queued. The settings live in the
`settings` table under `mail_host`, `mail_port`, `mail_security` (`starttls`, `tls` or `none`),
`mail_user`, `mail_from`, `mail_to` and `mail_lang` (the language of the administrator who saved
them, used for every mail); the password goes to `keys/mail.secret` (0600), is never shown again,
never logged and an empty password field keeps the stored one. «Turn the mail off»
first asks in the row of the mail (`/admin?mail=off`): the question says what is lost, and only
its danger button (`action=off`) drops the keys, the password and every mail still waiting;
«Cancel» goes back to `/admin` and keeps everything. There is a single
notice address (`mail_to`) for who moderates and who administers: accounts carry no address.

Mails wait in the table `mail_queue` (`topic` `case`, `health` or `test`, `case_id`, `subject`,
`body`, `state` `queued`, `sent` or `failed`, `tries`, `next_at`, `created_at`, `sent_at`). A
background job sends what is due and retries a failure after 1 min, 5 min, 30 min, 2 h and 8 h;
after the sixth try the mail is `failed`. The last result is kept in `mail_sent_at`,
`mail_failed_at`, `mail_failed_why` (`password`, `reach`, `secure`, `address`, `other`, said in a
person's words on the row) and `mail_failed_raw` (the raw answer of the mail server, under
"Details"). Another part of the server asks for a mail with
`mail::enqueue(app, topic, case, |setup| (subject, body))`: the closure runs in the language of
the mail and nothing is queued when mail is off.

What is sent: each case of priority 1 or more (reports, notices, counter-notices) sends one mail
with a link to the case and nothing about who sent it; the line of the case in `/mod` says whether
the notice went out, is on its way or did not go out. Every 15 minutes, and at once after an
automatic signing fails, the rows certificate, index signature, last backup and disk are compared
with the tone last seen (`health_seen`): a row that turns red or near sends one mail per change,
not per check. Subjects carry only the name of the server and of the row. "Send a test mail" (`POST /admin/mail/test`) sends at once and says whether it went out.

A failed automatic signing is stored in the `repos` row (`failed_at` and `failed_raw`): the row "Index
signature" turns red with when and why until a signing works again.

On the health page (`/admin`) the row "Index signature" appears only once the index was signed
for the first time (after the publish delay of the first upload). Under "Repository address" the
address one can copy is the plain repository address for the desktop app and the phone. On a Spanish page, a
detail the server gives only in English is kept under the fold "Details" of the error notice; on
an English page it is shown as it is.
