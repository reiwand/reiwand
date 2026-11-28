# `io.github.reiwand.Publish1`, `io.github.reiwand.Carry1` and `io.github.reiwand.Accounts1`

On this page: `io.github.reiwand.publish1`, `io.github.reiwand.carry1`, `io.github.reiwand.accounts1`.

## `io.github.reiwand.Publish1`

Publishes to a repository of a server of your own (`reiwand-server`, see `server.md`) with an account
stored in `Accounts1`: uploads, own packages, author keys and votes. The target of an upload is the
repository passed to `UploadAs` (the account linked to it); votes go through the account of the
repository of each package (`repo_id/id`); the other calls receive the repository (`repo`: its address
or its `repo_id`) whose account they use. With `repo` empty they only work if there is a single
repository with an account; with several they fail with `Failed` and name the addresses to choose from. The token
comes from where secrets are stored (`account <id>`) and never appears in answers, signals,
job targets, errors or logs. The daemon only contacts the account's server, with the
fingerprint pin of its origin when one exists, does not follow redirects and sends the generic
`User-Agent`. An account without a session or rejected returns `SignedOut`; automatic synchronization no longer
exists: data travels only through export and import (`Carry1`).

### `UploadAs(s repo, s path, s visibility) → (t job)`

Uploads a `.wpak` package with the token of the account linked to the repository `repo` (its address
or its `repo_id`; empty: the only repository with an account, `Failed` naming the choices when there are several) (`POST /api/v1/uploads`, `PUT`,
`finalize`) in an `upload` job whose output is an `UploadOutput`; it fails with no account for that
repository and when the server refuses the package. It is published after review, or at once when
the server publishes uploads of the account's role. `visibility` is `public` or `unlisted`
(`server.md`, "Uploads"): an unlisted package stays out of the catalog and its search, and whoever
has its id installs it. A server that does not repeat `unlisted` when the upload opens gets it
cancelled, and the job fails without uploading.
The job reports the bytes sent as its progress. Cancel stops it at once, also in the middle of the
file or of `finalize`; it fails when nothing of the file is taken by the connection for a minute, or
no answer comes within five minutes of sending all of it ("nothing reached it for a minute, so the
upload stopped"). A job that fails or is cancelled after the upload opened gives it back (`DELETE
/api/v1/uploads/{id}`), so it holds neither one of the two open uploads the server allows nor
quota; when the server cannot be reached the daemon keeps the id in `uploads-abandoned.json`
(`explicit-values.md`), also across a restart, and gives it back before its next upload to that
server; the server drops an open upload after a day anyway.

### `ListPackages(s repo) → (s json)`

`{items: [{package, version, title, state, created_at, published_at, visibility, decision,
statement}]}`: the versions the account of the repository `repo` uploaded, from `/api/v1/me/packages`.

### `SetVisibility(s repo, s id, s version, s visibility)`

Makes every version of the package `id` of this account (`version` empty) or one version `public` or
`unlisted`; `InvalidArgs` for another value or an id that is not a package id.

### `WithdrawPackage(s repo, s id)`

Withdraws the package `id` of this account from the repository `repo` for good
(`DELETE /api/v1/r/{repo}/me/packages/{id}` with `confirm: true`, `server.md`): every published
version is revoked with the message «Withdrawn by its author.» and its files are deleted. It cannot
be undone; the caller asks first. `InvalidArgs` for an id that is not a package id; the server's
refusal for a package of another account.

### `ListAuthorKeys(s repo) → (s json)`

`{items: [{fingerprint, public_key, created_at}]}`: the keys this account registered for its online
author id, from `/api/v1/me/author-keys` (`server.md`).

### `AddAuthorKey(s repo, s public_key) → (s fingerprint)`

Registers a minisign public key (the text of a `.pub` file or its key line) for this account's online
author id and returns its fingerprint; `InvalidArgs` for text that is no public key, and the
server's refusal when the key is registered already or the repository binds its authors in the root.

### `RotateAuthorKey(s repo, s from, s public_key, s signature) → (s fingerprint)`

Hands the online author over from the registered key `from` (a fingerprint) to `public_key` with the
`signature` that `reiwand-ctl pack rotate-key` prints, and returns the fingerprint of the new key.
`InvalidArgs` for a `from` that is no fingerprint or a text that is no public key.

### `RemoveAuthorKey(s repo, s fingerprint)`

Removes a registered author key; `InvalidArgs` for a text that is no fingerprint.

### `ListUploads(s repo) → (s json)`

`{uploads: [{id, state, errors, created_at}]}`: the upload sessions of this account, finished or not,
from `/api/v1/me/packages`; `ListPackages` lists the versions they became.

### `CancelUpload(s repo, s id)`

Cancels an unfinished upload session (`DELETE /api/v1/uploads/{id}`); `InvalidArgs` for an id that
cannot be one.

### `Vote(s id, s vote) → (s json)`

Sends `up`, `down` or `none` for the package `id` with the token of the account linked to its repository
(`POST /api/v1/r/{repo}/packages/{id}/vote`) and answers `{id, up, down, mine}`. `id` is `repo_id/id`; a bare
package id goes to the first account with a session. The server keeps only the counts and the vote of this
account. `InvalidArgs` for another vote, an id that is not a package id, or a `repo_id` with no account
linked (nothing is sent).

### `GetVotes(as ids) → (s json)`

`{items: {id: {up, down, mine}}, shown, repos}` for at most 1000 package ids, each a package id or
`repo_id/id`, asked of every repository that has an account with a session, at most 100 ids at a time
(`mine` is the vote of the account: `up`, `down` or `none`). `items` is keyed by the ids as given; ids of
repositories with no account are never sent and get no item. `repos` lists the `repo_id` of the
repositories that can be voted on, so a client shows vote buttons only on their packages. `shown` is
`false`, with no items and without reaching the network, while no account is linked or `repos.vote_counts`
is off. Counts are hints the repository does not sign. `RepoPackage` carries `featured` for the packages
the repository features (`repo.md`).


## `io.github.reiwand.Carry1`

Exports this computer's configuration to a single `.reiwand` file and imports it on another (format
in `export.md`). Importing only adds: it never replaces anything that is already there. Export and import
run as jobs (`carry_export`, `carry_import`) and show in `Daemon1.ListJobs`. Paths are
absolute. Passwords and provider keys leave only if the person checks them, and
always encrypted with the file's password. Account sessions (`Accounts1`) leave only with the keys, encrypted; on import, each computer asks the server for its own copy of the session, and one that cannot be copied is left as an account that asks to sign in again.

### `Export(s path, s options_json) → (t job)`

Writes `path`, which must not exist (`Exists`). `options_json` is
`{"own": bool, "secrets": bool, "password": string}`: `own` includes the files of the own
wallpapers; `secrets` includes the provider keys and requires a non-empty `password` (`InvalidArgs`
if missing); the file is encrypted when there are `secrets` or a `password`. Job output:
`{path, sections, own, encrypted}`.

### `PreviewImport(s path, s password) → (s json)`

Changes nothing. Returns `{file, rows, lists, notes}`: `file` (program, version, date, whether it is encrypted, sections),
`rows` with `{section, held, new, skipped, conflicts}` per section relative to what is already here,
`lists` with the names of the lists in the file and `notes` with notices. `InvalidArgs` if the file
is encrypted and the password is missing, `AccessDenied` if it does not work.

### `ImportChosen(s path, s password, s choices_json) → (t job)`

Adds what was chosen. `choices_json` is `{"take": [sections] | null, "lists": "join" | "both",
"suffix": string}`; a null `take` takes everything. The sections are `repositories`, `providers`, `lists`,
`favourites`, `ratings`, `tags`, `installed`, `settings`, `own` and `secrets` (`InvalidArgs` for any other).
`join` adds to the own list the items it lacks; `both` keeps the imported one beside it with
`suffix` appended to its name. A repository that asks for confirmation is left in `pending` and does not
make the import fail. Output: `{added, pending, unresolved, notes}`. Importing the same twice
duplicates nothing in `join` mode.

## `io.github.reiwand.Accounts1`

Several accounts on the servers of added repositories, each linked to the repositories it is used
for. The list is in `accounts.json` of the data directory (id, server origin, user, label, whether
the session must be signed in again, and the linked repository addresses); it never holds a token.
The token of each account lives only where secrets are kept (see `SetSecretStore`), in one item
`account <id>`. The token is sent only to the origin of the repository the account is linked to and
under that repository's path, never to a mirror or to another origin. A repository that answers
`401 account_needed` fails the add with `AccountNeeded`, whose message lists the accounts known on
that server.

### `List() → (s json)`

`{"model": 1, "serial": n, "accounts": [{"id", "server", "user", "label", "pending", "repos": [{"url", "repo_id"}]}]}`.
`repo_id` is `null` until the repository is added.

### `SignIn(s repo_url) → (s user_code, s verification_uri, t job)`

Starts a device-code sign-in to the server of the repository and links the new account to it. The
person approves `user_code` at `verification_uri` in the browser; the account job finishes when the
server issues the token. The address follows the rules of repository addresses (pinned certificate
of a self-signed origin included).

### `Use(s repo, s account)`

Links the account to the repository, replacing the previous link. `NotFound` when the account is
unknown; `InvalidArgs` when the repository is not on the account's server.

### `Unlink(s repo)`

Removes the link of the repository; the account stays.

### `SignOut(s account)`

Deletes the token item of the account and marks it as needing a new sign-in (`pending`); its links
stay, and signing in again with the same user reuses it. `NotFound` when the account is unknown.

### `Remove(s account)`

Deletes the account, its links and its token item.
