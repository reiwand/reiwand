# Conventions and Discovery and health

On this page: conventions, discovery and health.

## Conventions

- JSON in UTF-8. Times are Unix milliseconds unless they belong to repository files.
- Replies and stored data are always complete: every field is present, with `null` or an empty list where there is no value. A `?` after a name in a request body (`{private?, publishing?}`) marks a field of a command that may be left out because the call changes only what it names; it is not a field of a stored format.
- Errors are `application/problem+json`: `{type: "about:blank", title, status, code, detail}`.
  `code` is stable (`invalid`, `malformed`, `unauthorized`, `forbidden`, `not_found`,
  `rate_limited`, `handle_taken`, `version`, …).
- `429` responses carry `Retry-After`.
- Authentication is `Authorization: Bearer reiwand_<kind>_<64 hex>_<8 hex CRC-32>` (kinds `d`
  device, `p` personal) or the session cookie of the web site. The server stores SHA-256 of
  tokens and sessions only; revocation is immediate.
- Scopes: `catalog:read`, `upload`, `account`, `mod`, `admin`. Device tokens get `upload account`: only
  account sessions manage devices (a device may still sign itself out). Personal tokens
  are created on the web site; `mod` and `admin` need the role and expire within 90 days. Web
  sessions have every scope of the role. A token acts with the lower of its role and its scopes
  everywhere, the web pages included: without `admin` it is at most `moderator`, without `mod`
  either at most `trusted`, so a device token of an administrator cannot open the administration.
- Roles: `user`, `trusted`, `moderator`, `admin`.
- Requests with a cookie and an unsafe method must be same-origin (`Sec-Fetch-Site` and
  `Origin` are checked); bearer requests are exempt.
- Request bodies are limited to 1 MiB; uploads stream without limit up to the declared size. Requests time out after 30 s except uploads and moderation decisions.
- Every form endpoint also accepts `application/x-www-form-urlencoded`; with `?back=/path` the
  answer is an HTML page. `POST …?_method=DELETE|PATCH` stands for those methods in forms.
- Public reads (`/r/<name>/`, `/media/`, `/api/v1/r/<name>/catalog`, `/api/v1/r/<name>/packages/…`) send
  `Access-Control-Allow-Origin: *`; nothing else allows cross-origin use.

Rate limits per client address (IPv6 per /64): 20 requests/s with bursts of 50 overall; 5 per
minute for login, registration, setup, device code start and every password check
of a signed-in account; 10 failed logins or password checks per account and client in 15 minutes (so guessing from
elsewhere never locks the owner out); 5 reports or
notices per hour.

## Discovery and health

| Method | Path | Answer |
|--------|------|--------|
| GET | `/healthz` | `ok` |
| GET | `/readyz` | `200` or `503` with `{database, repository, index_fresh}` |
| GET | `/.well-known/reiwand-server` | `{name, version, api: ["v1"], registration, uploads: {max_bytes, per_day, licenses}, repositories: [{name, url, repo_id, entry_expires, private, publishing}], legal: {notice, terms, privacy, contact}}` |
