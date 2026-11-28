# Repository

`/r/<name>/<path>` serves the tree of `docs/specs/repo.md`: `root/<N>.json[.minisig]`, `entry/latest`,
`entry/<N>.json[.minisig]`, `index/<sha256>.json`,
`revoked/<sha256>.json`, `authors/<sha256>.json`, `featured/<sha256>.json`, `previews/<sha256>.<ext>` and
`packages/<id>/<version>.wpak`.
`entry/latest` is `no-cache`; everything else is immutable. A package that moderators removed or
revoked answers `410`, and its preview is deleted unless a release still listed uses it. The share
URL is the `url` of the repository in the discovery document (`https://example.org/r/main/`): a plain
address, with no parameters; a client learns the root key on its first contact.

### Several repositories

One server hosts several repositories. Each has a name (1 to 32 lowercase letters, digits or
hyphens), its own root and online keys, its own tree and its own signed index, and lives at
`/r/<name>/<path>` with the same tree as above. The first repository is called `main`. Every one lives in
`repos/<name>/` with `keys/<name>.online.key`; nothing is special about the first. Every repository is signed on its own and a client verifies each one
against its own root key: the key of one never verifies another.

`repositories` of the discovery document lists all of them with the address of each. Package, version, vote and featured data
belong to a repository (`(repo, id)`): the same package id can exist in two. Catalogue, package
page, votes and featured have a route per repository, `/api/v1/r/<name>/catalog`,
`/api/v1/r/<name>/packages/{id}`, `…/votes`, `…/admin/featured`; there is no route without `r/<name>`
for them (it answers `404`). An upload must carry `repo` (a name; `422` without it).

Moderation stays per server: one team, one set of blocks, one queue. Accounts and tokens are
shared; the history of a case counts the author's other versions in every repository, and leaves out the
same package id of another repository.

| Method | Path | Role | Answer |
|--------|------|------|--------|
| GET | `/api/v1/admin/repos` | admin | `{items}`: name, repo id, root version, share URL and address of each |
| POST | `/api/v1/admin/repos` | admin | `{title, name?, require_author_signature?, online_authors?}` creates one and answers `{repository, at}`; `title` (up to 80 characters) is required when `name` is missing, and then the address name is derived from it; the answer never carries a secret (`409` if the name is taken). `repository.root_key_kept` tells whether the server holds the root key |
| GET | `/api/v1/admin/repos/{repo}` | admin | State of one |
| PATCH | `/api/v1/admin/repos/{repo}` | admin | `{private?, publishing?}`: `private` hides the repository from non-members; `publishing` is `open` (any account may upload for review, the default) or `members` |
| GET | `/api/v1/admin/repos/{repo}/members` | admin | `{items: [{user, handle, may_read, may_publish}]}` |
| PUT | `/api/v1/admin/repos/{repo}/members/{user}` | admin | `{may_read?, may_publish?}`; publishing implies reading; both false removes the member |
| DELETE | `/api/v1/admin/repos/{repo}/members/{user}` | admin | `204` |
| DELETE | `/api/v1/admin/repos/{repo}` | admin | `{confirm: true}` deletes the repository (see «Deleting» under «Moderation and administration») → `{name, deleted: true}` |

«Repositories» in the administrator's header opens `/admin/repos`: one field, «Name» (free text, up to 80
characters), and a button. The address name is derived from it: lowercase ASCII, accents folded, any other
character a hyphen, hyphens collapsed and trimmed, at most 32 characters, `repo` when nothing is left, and `-2`,
`-3`… when taken. The title is the name as typed. Author signature stays off and online authors off (change
them with `PATCH` below). The form posts to `/admin/repos` and goes back to the list with the new repository
highlighted, its address to copy and the controls «Who can see it» and «Who can publish»; a refusal shows as a message on
that page. The same exists in setup step 3 for the first one and in
`reiwand-server repo init|import|status --repo NAME` (default `main`; `status` without `--repo`
lists all, `repo publish` signs all). The background job signs each repository on its own; a
failure of one does not stop the others and turns the health row red.

### Root key kept by the server

`init_repo` (the form, the API, setup and `repo init`) seals the root secret key of the new repository in
`keys/<name>.root.sealed` with `keys/seal.key`; it is never shown, logged or returned. It is read back only when a
root rotation needs it. Both files are under `keys/`, so `backup` and `restore` carry them. A repository made
before the server kept the key has no such file and `/admin/repos` says so on its row; nothing else changes for
it. The sealing key lives next to the data it protects: this guards against a copy of a single file, not against
whoever reads the whole data folder.

### Explore repositories (public page)

`/` lists the server's repositories without JavaScript and with the look of the other public pages:
name, number of listed wallpapers, date of the last publication, the plain address
`https://server/r/<name>` (with "Copy the address" when there is JavaScript; the text can be
selected without it), the "Open in Reiwand" button and a link to the catalog. Private repositories
only appear, marked "Private", to whoever signed in on the web and has read rights (administrators
and moderators see all of them).

"Open in Reiwand" is a `reiwand://add?url=<address encoded as a URL component>` link: the desktop
application is the handler of the `reiwand` scheme (`x-scheme-handler/reiwand` in its `.desktop`
file): it opens Explore with the "Add a repository" box and the address filled in, adding nothing
until the person presses "Add". The `url` parameter is the address that box accepts. The application
never lists repositories from a pasted server address: that list lives here.

Each repository has its catalog at `/r/<name>/` (search, filters and pagination as before) and its
package pages at `/r/<name>/packages/{id}`, with votes and reports of the repository itself. The
catalog of the first repository is `/r/<name>/` like that of the others; there is no other route. A
private repository answers 401 to whoever is not a member. Discovery adds `repositories_page` (the
server's address, `<url>/`).

### Private repositories and rights

A repository is public or private (`repos.private`). Members are rows of `repo_members (repo,
user_id, may_read, may_publish)`; the admin sets them per account in the People page
(`/admin/people`, section "Rights in repositories") or with the API above, and marks a repository
private in `/admin/repos`.
Administrators and moderators read every repository; only administrators publish everywhere.

- Reading a private repository (`/r/<name>/…` and its catalogue,
  package, versions, votes and featured routes) needs a bearer token or session of a member.
  Anyone else, with or without a token, gets `401` with the JSON error code `account_needed`.
  Any token scope reads it (the device token an app keeps works). Files of a private repository
  carry `Cache-Control: private, no-cache`. Public repositories need no token, as before.
- Publishing: an upload names `repo` (API field or web form field). It needs `may_publish` in a
  private repository or in a public one set to `publishing: members`; otherwise `403` with code
  `not_allowed`. The default for a public repository is `open`: any account may upload and the
  upload waits for review or is published as the server's publishing policy says.
- Discovery: `repositories` lists only the repositories the caller may read (so anonymous callers
  never see private ones) and each item adds `private` and `publishing`. `GET
  /.well-known/reiwand-server?repo=<name>` (also `r/<name>`, `/r/<name>/` or `repo` for the first)
  answers one repository: `{name, private, publishing, registration}`, plus `url` (the repository
  address) and `repo_id` when the caller may read it, so a client learns that it must ask for an
  account before it adds a private repository. An unknown name is `404`.
- `POST /api/v1/auth/session/copy` (body optional `{name}`): from a valid token or session it
  issues a new, independent token of the same account (`201`, `{id, name, token, scopes,
  expires_at}`), with only the `catalog:read` and `upload` scopes the source has, and the same
  expiry, at most a year (a year for a browser session or a device token, which have no end of their
  own). Each device that imports an account gets its own copy;
  revoking one (`DELETE /api/v1/me/tokens/{id}` from the account page) leaves the others alone.
  A copy is not tied to the device it was made from: revoking that device leaves its copies until
  they expire (a year at most) or are revoked one by one on the account page.

### Database versions and backups

The database carries a version number (`user_version`, 16). The server knows only that version:
it refuses to start on any other, says so and leaves the database untouched. It contains no code for
older databases or layouts and nothing migrates them: a change that breaks the data is a major version.
`reiwand-server restore` takes a backup of the current version only. After restoring,
every repository is marked to sign again. Backups include the `repos/` trees and their keys.

### Explicit values in the database

No table has a `DEFAULT`: every insert in the server writes every column, and the schema (`apps/reiwand-server/schema.sql`) lists
none. Saved settings are complete too: the setup assistant writes `registration`, `auto_publish`, `network`, `cert_path`,
`key_path`, `repo_name` and `admin` before it marks `setup` as done, and a database that says it is set up but lacks one of
them is refused naming the key.

A repository address with `?fingerprint=` or `?repo_id=` is refused with a message that says to use the plain
address.

Only the online key signs on the server, and only entries and their details: an approval adds the
package with `reiwand_format::add`, a removal or revocation calls `reiwand_format::revoke`, and a
background job calls `reiwand_format::publish` with a lifetime of `REIWAND_ENTRY_DAYS` after changes and
at least every `REIWAND_RESIGN_HOURS`. Packages are either unsigned (when the root does not require
author signatures) or signed by an author key that the uploader registered and that the root lists
or, in a repository created with `--online-authors`, that the online authors file lists.
`--online-authors` alone does not make signatures required: without `--require-author-signature`
a package that was never signed is accepted and clients install it as unsigned, whatever the
uploading client checked. Once a version of a package is published, `reiwand_format::add` refuses a
later version with another author or without a signature (the upload stays in review and the
repository keeps serving the signed versions), and clients that have it refuse an unsigned release.

### Online authors

With `--online-authors` the root sets `policy.authors = "online"`. Every account is the author
`u<account id>`; the keys it registers at `/api/v1/me/author-keys` go into the operator catalog and
into `authors/<sha256>.json`, which the next entry pins (`docs/specs/repo.md`). A new key for
packages already published needs a rotation signed by an earlier key of the account:
`reiwand-ctl pack rotate-key OLD.key NEW.pub --author u<id>` prints the signature, and
`POST /api/v1/me/author-keys/rotate` (or the form of `/account`) stores it with the new key. The
desktop does the same without the web page: `reiwand-ctl server author-key list|add|rotate|remove`, or
*Author keys* in *Account* (see `dbus.md`), and `reiwand-ctl server uploads [--cancel ID]` or *Uploads that did not finish*
in *Account* list and cancel the upload sessions that `/api/v1/me/packages` returns in `uploads`. Clients
trust, for each package, the key that signed the first version they installed and accept a later
version only from that key or a key that a chain of rotations reaches; authors that the root lists
keep priority over online ones with the same id. So whoever steals the online key can list new
authors and new packages, but not update a package that a client already installed.

Clients must treat the catalog API as hints and resolve `(id, version, sha256, size)` against
the verified shards before installing.
