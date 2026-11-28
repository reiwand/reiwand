# Uploads

A package is accepted only when its manifest is a whole `reiwand.wpak/3` one: an incomplete or older manifest is refused naming the field (or, for an older schema, the schema it reads); the server never completes it. See `package.md`.

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/api/v1/uploads` | `upload` | `{size, sha256, visibility?}` → `201 {id, put_url, expires_at, visibility}`; `visibility` is `public` (the default) or `unlisted`; checks blocklist, strikes, daily limit, two open sessions (an upload received and still being checked counts as open) and quota first, counting the bytes of the open sessions as used |
| PUT | `/api/v1/uploads/{id}` | `upload` | Body is the `.wpak`; `Content-Length` must equal `size`; size and SHA-256 are checked while streaming; one request at a time per upload (`429`) |
| POST | `/api/v1/uploads/{id}/finalize` | `upload` | Validates and returns the status |
| GET | `/api/v1/uploads/{id}` | `upload` | Status |
| DELETE | `/api/v1/uploads/{id}` | `upload` | Cancel an unfinished upload → `204` |
| DELETE | `/api/v1/r/{repo}/me/packages/{id}` | `upload` | `{confirm: true}` withdraws a package of this account for good (see «Deleting» under «Moderation and administration») → `{id, withdrawn: true}`; `404` for a package of another account |
| POST | `/api/v1/r/{repo}/me/packages/{id}/visibility` | `upload` | `{visibility: public\|unlisted, version?}` sets every version of a package of this account, or one, → `{id, version, visibility}`; `404` for a package of another account or an unknown version |
| POST | `/upload` | session | `multipart/form-data` field `package`, and an optional `visibility` before it, for the web site; strikes, daily limit, open uploads and quota are checked and an open upload session is reserved before the package is read, the blocklist right after; at most 8 fields, others up to 4 KiB, 30 s each |

An unlisted version is published, signed and served like any other, with `listed: false` in its
release of the repository (`repo.md`): the catalog, its search and the web site's lists leave it out,
while its page, the package API and installing it by id keep working. Clients that do not know the
field never see it, since first-version shards leave unlisted releases out. Unlisted is not private:
the detailed shards list every release with its title, description and id, and anyone can read the
repository, so the account page, the upload dialog of the app and the page of the package say so and
point to an own private repository. Clients ask for
`unlisted` only when the answer of `POST /api/v1/uploads` repeats it, and cancel the upload otherwise,
so that a server without unlisted uploads never publishes a package openly. `/api/v1/me/packages`
gives each version its `visibility`, and the account page has the picker on the upload form and a
button per version; every change is audited and the repository is published again.

Status: `{id, state, errors: [string], package, version}` with `state` one of `open`,
`received`, `failed`, `pending_review`, `published`, `rejected`, `removed`, `revoked`.

Validation runs `reiwand-server validate` in a child process limited to 1 GiB of address space
and 60 s of CPU, two at a time. It opens the package with `reiwand_format::open` (strict ZIP
profile and manifest rules), verifies `manifest.json.minisig` against the uploader's author
keys that the root also lists, or every registered key of the uploader with online authors (required
when the root demands signatures), keeps the rating of packages rated `mature` (`content` and the
facet `mature`), refuses `imported` packages and provider packages
under other licenses than those `package.md` allows them, and decodes the
thumbnail (`previews.thumbnail`, or `preview` in the first manifest version) (PNG, JPEG, WebP, GIF or BMP, at most 16384 px) into a JPEG thumbnail. The
server then checks the licence list, that the id belongs to the uploader (the first uploader
owns it) and that the version is newer than every earlier one. Accepted versions wait for
review unless `REIWAND_AUTO_PUBLISH` publishes them.
