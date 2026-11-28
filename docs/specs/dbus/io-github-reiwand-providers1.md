# `io.github.reiwand.Providers1`

Providers download wallpapers from online services into a cache folder,
`~/.cache/reiwand/providers/<id>/` (`0700`), that is a library source; playlists use their
items through the library (for example `Playlists1.Create(name, "source:<source uid>")`). Kinds
are `bing`, `apod`, `wikimedia`, `wallhaven`, `webdav`, `nextcloud` and `immich`, plus `plugin:<id>`
for each installed provider plugin (`InstallPlugin`); their options, defaults, contacted hosts and
secrets are listed in `GetSchema().providers`. Local folders are
`Library1` sources, not providers. Provider ids match `^[a-z][a-z0-9_-]{0,31}$`.

Secrets (passwords and API keys) are written only by `Add` and `SetSecret`, to the Secret
Service (`org.freedesktop.secrets`, default collection, falling back to `login`) as an item
labelled `reiwand: <id>` with the attributes `xdg:schema = io.github.reiwand.Provider`
and `provider = <id>`. They are never stored anywhere else and never appear in replies, signals,
job errors, logs or `config.toml`. A secret has at most 4096 bytes and no control characters.
Only `Add` with a secret, `SetSecret`, `ForgetSecret`, `Set` of `url`, `server` or `user`,
`Remove` and an explicit `Sync` may show the keyring's unlock prompt; they wait at most 20 s and
then return `KeyringUnavailable` (`unlock the keyring and try again`) without storing or
deleting anything. Clients call `Add` and `SetSecret` with a timeout of 120 s. A Secret Service
that does not answer gives `KeyringUnavailable` (`the keyring did not answer in time`) after
10 s for lookups and after 100 s for writes, deletions and reads that may prompt; after a lookup
times out, the lookups of the next 60 s fail at once, so scheduled syncs never wait for it again
and wait for their next due time. `List` waits at most 1 s for the keyring: a provider whose
lookup has not answered by then is listed with the last answer the daemon saw in this session,
or with `has_secret = false` and state `locked` when there is none, and `Changed("providers")`
follows if the late answer differs.

The `wallhaven` options go to the official API v1 (`/search`, `/collections/<user>/<id>`) as
follows; empty ones are not sent and `GetSchema().providers` lists the allowed values:

| Option | Kind | Query parameter |
| --- | --- | --- |
| `query` | text, or a pasted `https://wallhaven.cc/search?...` address | `q` (tags, `-tag`, `+tag`, `type:png`); a pasted address fills `query`, `similar`, `tag`, `categories`, `purity`, `sorting`, `order`, `top_range`, `atleast`, `resolutions`, `ratios`, `colors` and `page` from its parameters |
| `categories` | flags (general, anime, people) | `categories`, 3 digits |
| `purity` | flags (sfw, sketchy, nsfw) | `purity`, 3 digits; sketchy and nsfw need the API key |
| `sorting` | enum: date_added, relevance, random, views, favorites, toplist, hot | `sorting` |
| `order` | enum: desc, asc | `order` |
| `top_range` | enum: 1d, 3d, 1w, 1M, 3M, 6M, 1y | `topRange`, only with `sorting = toplist` |
| `atleast` | `WIDTHxHEIGHT` | `atleast` |
| `resolutions` | list of `WIDTHxHEIGHT` | `resolutions` |
| `ratios` | list: `16x9`, `16x10`, `21x9`, `32x9`, `48x9`, `9x16`, `10x16`, `9x18`, `1x1`, `3x2`, `4x3`, `5x4`, `landscape`, `portrait` | `ratios` |
| `colors` | palette of the 29 Wallhaven colors, hex without `#` | `colors` |
| `similar` | wallpaper id | `q=like:<id>` |
| `tag` | tag number | `q=id:<number>` |
| `uploader` | user name | `q=@<user>` added to `query` |
| `collection` | `<user>/<id>` | path `/collections/<user>/<id>`; only `purity` and `page` are sent |
| `page`, `pages`, `max_items` | whole numbers (1 to 1000, 1 to 10, 1 to 240) | `page`, first page and how many pages are read per sync; the list stops at `max_items` or the last page |

`similar` and `tag` cannot be combined with each other or with `query` and `uploader`, and a
`collection` takes no search terms (`InvalidArgs`). With `sorting = random` the daemon picks a new
`seed` for every sync and sends it with every page, so pages never repeat pictures and each
sync draws a new selection. One sync makes
`pages` requests, never more than needed; the API key travels only in the `X-API-Key` header, never in a
URL, the configuration or the logs. A `429` is retried after `Retry-After` or 2, 4 and 8 s and
then fails the sync, which is retried later like any failed sync.

URL options (`url`, `server`) must be `https://` or `http://`: the address of a person's own
server may use plain http, always allowed, and `Provider.plain_http` is `true` for an `http://`
address that is not this computer so that clients show a permanent notice that the password or key
and the pictures travel unencrypted. URLs with user
information, a query or a fragment return `InvalidArgs` (`put credentials in the secret, not in
the URL`). Requests that carry a secret never follow a redirect to another origin or from HTTPS
to HTTP.

A `plugin:<id>` provider takes the plugin's settings as options (with `refresh`, one day by default,
and `cache_mb`), each one as declared (`text`, `int` with its bounds, `bool`, `choice` as `enum`),
and the secret the plugin declares (`GetSchema().providers[x].secret.name` is its label), stored in
the keyring under the provider id like any other. A sync calls the plugin's `list` in its sandboxed
process (`reiwandd --provider-plugin-host`) for at most 5 pages and 500 items, and downloads the
files only over https from the hosts the plugin declares, refusing names with only private,
loopback or reserved addresses and following redirects only to those hosts (only to
`secret.hosts` when the request carries the secret, which goes only to `secret.hosts`), with the
same cache limits as below. A fault of the plugin fails the sync with its reason in `last_error`.

A sync job lists the provider's items and downloads them in list order into the cache folder.
Files of items shown on an output, favorite, rated above 0 or in a manual playlist are protected:
downloading stops when the other files would exceed `cache_mb` minus the protected files (the
rest count as `skipped`), and older unprotected files are then evicted. The limit is enforced
while a file arrives: a `Content-Length` above the room left is refused before the body is read,
and a transfer without one stops as soon as it passes that room. A protected file never gets more
room than `cache_mb`: a larger one is refused the same way and counts as `skipped`. Eviction resolves symbolic links
first and only deletes regular files directly inside the cache folder. The folder is scanned
(evicted items become `missing` and never rotate), the items get the provider's titles while
their title is still the file name, and their credits (title, attribution, license, and a link
for public services) are kept in the folder's `.credits.json` and reported as
`ItemDetails.credit` and `GetState().outputs[x].credit`.

A provider whose `refresh` is not `0s` syncs automatically when it has never synced or its last
sync is older than `refresh`; the schedule is checked again 5 s after `Add`, `Set` and
`SetSecret`. Failed syncs are retried after 30 s, doubling up to 1 h, and cancelled ones after
1 h. A provider whose required secret is missing is in state `needs_secret` and does not sync
automatically; one whose secret was rejected (HTTP 401 or 403, `NeedsSecret`) waits for
`SetSecret`. Automatic syncs never show an unlock prompt: with a locked keyring the provider's
state becomes `locked` and the sync waits for its next due time. A sync that could not reach the
server at all (the connection failed or timed out) fails with `Offline` and leaves the provider in
state `error` with `offline = true`; any other failure, such as an error answer of the server or an
invalid setting, leaves `offline = false`. Clients tell the user there is no connection only for
`offline`, and show `last_error` as the provider's own problem otherwise.

## `List() → (s json)`

A `ProvidersReply`; `store` is `keyring` or `file` and `keyring_found` says whether a Secret Service answers (always `true` with `file`). Entries of `config.toml` that cannot be read (an unknown kind or option, a
bad value, a secret written by hand) are listed with state `error`, their reason in
`last_error` and no options; they are never synced and are kept in the file unchanged.

## `Add(s id, s kind, a{ss} options, s secret) → (s source)`

Validates everything, then stores a non-empty `secret` in the keyring (`KeyringUnavailable`:
nothing is created), registers the cache folder as a library source, writes
`[providers.<id>]` to `config.toml`, and returns the source uid. `secret = ""` stores none; a
kind whose secret is required then starts in `needs_secret`. On a kind with a secret,
`secret = ""` also makes sure that no secret left in the keyring under the same id (by a
provider removed while the keyring could not be reached) reaches the new provider: such an item
is deleted when that needs no unlock prompt. When the keyring is locked or cannot be reached, the
provider is added anyway with `orphan_secret = true`: the old item is never read for it, and it is
deleted without a prompt as soon as the keyring is unlocked or appears again (the daemon watches
the Secret Service for that and also retries at startup); `SetSecret` replaces it at once. `InvalidArgs` for an invalid id,
an unknown kind, option or value, a missing required option, an option named like a secret
(`password`, `api_key`, `token`, `secret`), a refused URL, or a secret given to a kind without
one; `Exists` for a taken id, also while another `Add` of the same id is running (the second call
never touches the keyring).

## `Set(s id, s key, s value)`

Changes one option with the same checks. Cached files stay until the next sync replaces them.
On a kind with a secret, changing `url`, `server` or `user` also deletes the stored secret first
(`KeyringUnavailable`: nothing changes), so a new host never receives the old secret.

## `Update(s id, a{ss} options)`

Changes several options at once: the result is checked as a whole, so options that only make
sense together (a `collection` with its search terms cleared, `similar` replacing `query`) can
change in one call, and nothing changes when any value is refused. `Set` rules otherwise.

## `Remove(s id, b purge)`

Cancels a running sync, deletes the stored secret (`KeyringUnavailable` only when it exists and
cannot be deleted; then nothing is removed), the library source with its items and thumbnails,
and the configuration entry. When the keyring cannot be reached, its secret is deleted later in
the same way as an orphaned one. `purge` also deletes the cache folder; without it the downloaded
files stay, but `.credits.json` goes too, so a provider added again under the same id syncs at
once instead of waiting for the old `refresh` interval.
