# Catalog and Accounts

On this page: catalog, accounts.

## Catalog

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/v1/r/{repo}/catalog?q=&kind=&content=&facet=&sort=&page=&limit=` | — | Newest published and listed version of each package that the moderators did not hide; `q` matches title, description, tags and id; `kind` is `image`, `video`, `scene` or `web`; `content` is `general`, `mature` or `unrated` (packages of the first manifest version); `facet` is one facet the package must have (`mature` included); `sort` is `newest` (the default: newest publication first), `title` (A to Z, without case) or `votes` (up minus down votes, highest first); `page` counts from 1; `limit` 1 to 100 (24); any other `sort` is `422` |
| GET | `/api/v1/r/{repo}/packages/{id}` | — | Latest version plus `versions: [string]` |
| GET | `/api/v1/r/{repo}/packages/{id}/versions` | — | `{items: [item]}` |
| POST | `/api/v1/r/{repo}/packages/{id}/reports` | — | `{category: copyright\|illegal\|malware\|mature\|spam\|other, details}` → `201 {case}` |
| GET | `/media/{sha256}.jpg` | — | Thumbnail of a pending or published version |
| GET | `/band/{sha256}.css` | — | Tones of the band for the version on its cover (`404` for any other version) |
| POST | `/api/v1/r/{repo}/packages/{id}/vote` | session or token | `{vote: up\|down\|none}` → `{id, up, down, mine}`; one vote per account and package, at most 60 changes an hour (`429`), `404` for a package that is not published |
| GET | `/api/v1/r/{repo}/packages/{id}/vote` | session or token | `{id, up, down, mine}` |
| GET | `/api/v1/r/{repo}/votes?ids=` | — | `{items: {id: {up, down, mine?}}}` for at most 100 package ids separated by commas; `mine` (`up`, `down` or `none`) only with a session |

`featured=true` keeps the packages the operator features. `votes` is `{up, down}`: counts of the
votes of accounts, unsigned hints that are not part of the repository. The server keeps one vote per
account and package to count it once and lets each account read only its own; it never lists who
voted and keeps no record of installs or downloads. The page of a package shows the counts, the
note when it is featured and, for a signed-in visitor, buttons to vote up, down or withdraw the vote.
The desktop app and the phone vote with the token of the device and read the counts with
`/api/v1/r/{repo}/votes`, which adds `mine` for that account; both skip the counts while `repos.vote_counts`
is off. They ask and vote only for packages of the repository the server announces (its `repo_id`
in `/.well-known/reiwand-server`): ids of packages from other repositories never reach the server, and
rows of other repositories show no counts and no vote buttons.

Every page of the server draws, behind the menu and the title, the cover of the newest version
that a visitor without an account sees in the catalog: published, listed, newest listed version of
a package the moderators did not hide, and rated for everybody (`content` is `general`; mature and
unrated versions never reach the band, which the sign-in page shows too). The page links its
thumbnail (`/media/{sha256}.jpg`) and a stylesheet of the same server (`/band/{sha256}.css`) that
sets the tones of the page; both addresses carry the version, so browsers keep them for a year and
fetch new ones when the newest version changes. A credit line under the title names the wallpaper
and its owner. The tones are worked out once from the thumbnail, the first time that version is
on the band, and stored with it; the veil over the cover grows with the cover until the title, the
menu and the status line read at 4.5:1 or more over its brightest part (95th percentile). With no
such version, or with a thumbnail that cannot be read, pages draw the plain band in the default
tones. The setup assistant draws the plain band until the first wallpaper is published.

Search uses an SQLite FTS5 index with the trigram tokenizer (without diacritics) over the id,
title, description and tags of published versions, kept up to date by triggers: every word of `q`
with at least 3 characters must appear as a substring, in any case and with or without accents;
a `q` without such a word matches tags that start with it.

Item: `{id, version, kind, title, description, license, tags, authors, size, sha256, published_at,
owner, thumbnail, download, content, photosensitive, facets, hidden, listed, featured, votes}` (`thumbnail` and `download` are
paths; `content` and `photosensitive` are `null` for packages of the first manifest version; `hidden`
is the moderators' reason, or `null`, and only `/api/v1/r/{repo}/packages/…` answers hidden packages; `listed` is
`false` for an unlisted version, which only `/api/v1/r/{repo}/packages/…` and the page of the package show, with a
note that it is unlisted). Titles and
descriptions are those of the package's default language. The web catalog filters by rating and
facet as well, shows the description with its markup (as written for packages of the first manifest
version), a warning on the page of a photosensitive package and the previews of mature packages
blurred until the visitor selects them, one package at a time, without JavaScript; the web site
keeps no choice, so the blur comes back on every visit. The list answer is `{items, page, pages,
total}` (`pages` is at least 1; a page past the last answers no items) with `Link: <…>; rel="next"`
and `rel="prev"` for the pages around it, carrying every filter of the request.

The web catalog `/r/<name>/` takes the same query keys. Its search form has a «Sort by» picker
(«Newest», «Title», «Most votes»), and below the grid a `nav` labelled «Pages» shows «Previous»,
«Page N of M» (`aria-current="page"`) and «Next» when there is more than one page. Every link
carries the search, the filters, the order and the page as query keys and leaves out those that
are empty or the default (page 1), so each page has one address that can be bookmarked.

## Accounts

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/api/v1/auth/register` | — | `{handle, password, invite}` → `201 {id, handle}` |
| POST | `/api/v1/auth/login` | — | `{handle, password, device: {platform, client?}}` → device token (below) |
| POST | `/api/v1/auth/logout` | token or session | Revokes the token or ends the session → `204` |
| GET | `/api/v1/me` | any | `{id, handle, role, created_at, quota_bytes, used_bytes}` |
| DELETE | `/api/v1/me` | session | `{password}` → deletes the account and everything it owns → `204` |
| POST | `/api/v1/me/password` | session | `{current, new, devices?}`; see below |
| GET | `/api/v1/me/export` | session | Profile, devices, tokens, keys, versions and uploads |
| GET, POST | `/api/v1/me/tokens` | any, session | List; create `{name, scopes: "upload account", days?}` → `201 {id, token, scopes, expires_at}` (shown once) |
| DELETE | `/api/v1/me/tokens/{id}` | session | Revoke a personal token |
| GET | `/api/v1/me/devices` | any | `{items: [{id, name, platform, created_at, seen_at}]}` |
| DELETE | `/api/v1/me/devices/{id}` | session, or the device itself | Revoke a device and its tokens |
| GET, POST | `/api/v1/me/author-keys` | any, `upload` | List; add `{public_key}` (minisign public key file or line) → `201 {fingerprint}` |
| POST | `/api/v1/me/author-keys/rotate` | `upload` | `{from, public_key, signature}`: hand an online author over from a registered key to a new one → `201 {fingerprint}` |
| DELETE | `/api/v1/me/author-keys/{fingerprint}` | `upload` | Remove |
| GET | `/api/v1/me/packages` | `upload` | `{items: [{package, version, title, state, created_at, published_at, decision, statement}], uploads: [{id, state, errors, created_at}]}` |

Handles are 3 to 32 of `a-z 0-9 _ -`; passwords 10 to 256 characters without the handle,
hashed with Argon2id (19 MiB, 2 passes). A new password revokes every session (the answer sets a
new session cookie with `204`), every personal token and every approved device code of the
account; with `devices: true` it also revokes every device, so devices
sign in again. The password of an
account without one can only be set from a session signed in within the last 10
minutes, and so can such an account be deleted. The web site adds `/login`, `/register`, `/setup`,
`/link` and `/account`.

The server holds no library data of its users: no lists, favourites, settings, files or mailbox of devices, and no key of theirs. What a person moves between devices is the exported file (`docs/specs/export.md`). An account exists to publish, to vote and to read the private repositories it is a member of.

Devices are named on the server `Device 1`, `Device 2`… and never by the client. `platform` is `linux`, `android` or
`other`.

Device token answer: `{token, device_id, device_name, scopes}`.

### Device enrollment

For devices without a keyboard or accounts without a password (RFC 8628 style):

1. `POST /api/v1/auth/device/start {platform, client?}` (`client`: at most 64
   characters naming the client software) →
   `{device_code, user_code: "BCDF-GHJK", verification_uri, verification_uri_complete, interval: 5, expires_in: 600}`.
2. The device shows `user_code` and `verification_uri_complete`.
3. The user approves at `/link` with a web session. A device token cannot approve or preview a
   request (`403`), so a stolen device token cannot add devices. `POST /api/v1/auth/device/preview {user_code}`
   (a web session) answers what the pending request sent, `{platform, client, requested_at, expires_at}`,
   and never an address; `/link` shows the same with the warning to approve only a code shown on the
   user's own screen.
4. The device polls `POST /api/v1/auth/device/token {device_code}` no faster than `interval`.
   Answers `400` with `code` `authorization_pending`, `slow_down` (add 5 s), `expired_token`,
   `access_denied` or `invalid_grant`, or `200` with the device token.
