# Running

```sh
cp .env.sample .env                          # set REIWAND_PUBLIC_URL, the only required variable
docker compose up -d                         # pulls the published image
docker compose logs reiwand | grep setup        # one-time token for the setup assistant at /setup
```

| Command | Effect |
|---------|--------|
| `serve` (default) | Run the server; the database is created first when it is missing |
| `prepare` | Create the database if it is missing, check its version and exit; a database made by another version of the program is refused and left untouched |
| `healthcheck` | Exit 0 when `GET /healthz` on the local port answers 200 (the image `HEALTHCHECK`); with `REIWAND_TLS=self-signed`, when the port accepts a connection |
| `setup-token` | Print a one-time token (24 h) that opens the setup assistant at `/setup`; once the setup is finished it opens nothing |
| `fingerprint` | Print the lowercase hex SHA-256 of the certificate served with `REIWAND_TLS=self-signed`, making it first when there is none |
| `repo init --name N [--require-author-signature] [--online-authors]` | Create the repository: a root key and an online key. The server seals the root secret key in its data folder (see «Root key kept by the server») and prints only its fingerprint. `--online-authors` lets accounts register author keys online (below) |
| `repo import DIR [--online-key FILE]` | Import `root/<N>.json` files signed offline (and `catalog.json` the first time) plus the online key |
| `repo status`, `repo publish` | Show the repository and its share URL; sign and publish now |
| `backup [DIR]` | Snapshot the database with `VACUUM INTO`, copy the blobs, thumbnails and complete private files that the snapshot names (a file deleted meanwhile takes a new snapshot, up to three times; files missing from two snapshots in a row are listed as `missing` in the manifest with a warning), the withdrawn packages, the keys and the repository; default `DIR` is `<data>/backups/<UTC time>`, removed again when the backup fails |
| `restore DIR [--force] [--advance N]` | Check every checksum, `PRAGMA integrity_check` and that the snapshot has this version's schema, restore with the server stopped, then publish `N` (default 1) new index versions. Signed root and index files newer than the snapshot stay, so index numbers are never reused; after losing the data directory, pass an `N` that goes past the last version clients saw. Sync heads jump to the restore time in milliseconds, so every device downloads its records again (`410 resync`), and unfinished private files start again from the first byte |

The image builds from base images pinned by digest, has no shell and runs as root by default, like a plain image;
whoever wants another user adds a `user:` line to `compose.yaml`. `./data` (mounted as `/data`) holds everything, so a backup is a copy of that folder.
`compose.yaml` pulls `ghcr.io/reiwand/reiwand-server:${REIWAND_VERSION:-latest}`; to build from the
checkout instead, add the override: `docker compose -f compose.yaml -f deploy/compose.build.yaml up -d --build`.

The server closes any connection whose request headers do not arrive within 10 seconds (fixed value, idle keep-alive connections included).

TLS is left to a reverse proxy of the operator's choice. It must pass `Host`, `X-Forwarded-For` and
`X-Forwarded-Proto`; by default the server believes `X-Forwarded-For` only from loopback and, inside a
container, from the gateway of its network, which is where a proxy of the same host arrives through
the published port. Any other client, a device of the home network included, is counted by its own
address and cannot pick another one with that header; a proxy elsewhere (another container of the
compose network, another machine) is named in `REIWAND_TRUSTED_PROXIES`. `deploy/nginx/reiwand.conf` is an Nginx example: it allows
bodies up to the default `REIWAND_PACKAGE_MAX_MIB`, uses HTTP/1.1 upstream, streams uploads, and serves
streams uploads. The server uses no WebSockets and no long-lived event streams.

Each release also pushes the image to `ghcr.io/<owner>/reiwand-server` (`linux/amd64`, tags
`<version>` and `latest`) with BuildKit SPDX SBOMs of the image and of the build stage and a
provenance attestation, signs its digest with cosign keyless from the release workflow
(`scripts/server-image.sh`) and adds a signed SPDX attestation (`cosign attest --type spdxjson`) of
the Rust dependencies in the sources it was built from. Check both with `cosign verify` and
`cosign verify-attestation --type spdxjson` against the identity
`https://github.com/<owner>/reiwand/.github/workflows/ci.yml@refs/heads/master` and the
issuer `https://token.actions.githubusercontent.com`, then pin that digest in `compose.yaml` in place of the tag
(`image: ghcr.io/<owner>/reiwand-server@sha256:<digest>`).

### First setup

Until it is set up, the server writes `the server is not set up yet: open <url>/setup and enter the
one-time setup token …` at every start. `/setup` asks for that token and then walks four steps;
each one is stored as soon as it is sent, so closing the browser or restarting the server resumes
at the first step not done:

1. **Network**: «Only at home» (the server makes its own certificate, as `REIWAND_TLS=self-signed`) or «I have my certificate» with the
   paths of a PEM certificate chain and its PEM private key inside the server. Both paths empty
   means a reverse proxy in front holds the certificate. Before storing the paths the server reads
   both files and checks that the certificate is in force, that it names the host of
   `REIWAND_PUBLIC_URL` (DNS name, `*.` wildcard or IP address) and that the key is the one of the
   certificate; each wrong file is marked with the reason, and the raw errors are under «Details».
   Serving HTTPS itself needs a `REIWAND_PUBLIC_URL` that starts with `https://`. The choice takes
   effect when the setup is finished and at every later start.
2. **Account**: the administrator. «Change» on a later step edits the same account.
3. **Repository**: its name, its address name (`main` prefilled) and «Online authors» (as `repo init --online-authors`). The root
   secret key is not shown: the server keeps it sealed (see «Root key kept by the server»). A repository made before
   with `repo init` or `repo import` skips this step.
4. **Who gets in**: «Closed», «By invitation» or «Open», stored as the registration and
   publishing choices below. The page also makes the first invitation (one use, 7 days) and shows
   it once as a link; choosing «Closed» deletes it.

«Finish» stores the choices, applies the network choice and deletes every setup token and setup
visit. From then on `/setup` shows the «Done» screen to administrators (the web address, the plain address of the repository, how to upload) and sends everyone
else to the catalog; no request to `/setup` can make another administrator or repository, and
new setup tokens are refused. A server whose database already has an administrator and a
repository counts as set up when it starts.

The setup visit is a cookie `reiwand-setup` (`__Host-reiwand-setup` on HTTPS), 24 h, stored as a
digest; the token form shares the login rate limit.

### Stored settings

The registration choice, the publishing choice and the network choice live in the database.
The variables `REIWAND_REGISTRATION`, `REIWAND_AUTO_PUBLISH` and `REIWAND_TLS` only give
the starting value: once a choice is stored it wins over them. «Closed» and «By invitation»
publish every upload at once (`all`); «Open» publishes only from trusted accounts (`trusted`).

### Configuration

Every setting is an environment variable (`.env` next to `compose.yaml` is read, and `.env.sample` lists
them all; `REIWAND_VERSION` and `REIWAND_BIND` only select the image tag and the published address). `NAME_FILE`
reads the value of `NAME` from a file, for secrets.

| Variable | Default | Meaning |
|----------|---------|---------|
| `REIWAND_PUBLIC_URL` | `http://localhost:8080` | External URL; `https://` enables HSTS and the `__Host-reiwand` cookie |
| `REIWAND_LISTEN` | `0.0.0.0:8080` | Listen address |
| `REIWAND_DATA` | `/data` | Data directory |
| `REIWAND_TRUSTED_PROXIES` | `127.0.0.0/8,::1` and, in a container, its gateway | Advanced: CIDRs whose `X-Forwarded-For` is believed, read right to left; setting it replaces the list, so loopback must be named again if wanted |
| `REIWAND_NAME` | `reiwand` | Instance name |
| `REIWAND_REGISTRATION` | `invite` | `closed`, `invite` or `open`, until the setup stores a choice |
| `REIWAND_AUTO_PUBLISH` | `trusted` | Publish uploads without review: `never`, `trusted` (role trusted and above) or `all`, until the setup stores a choice |
| `REIWAND_LICENSES` | `CC0-1.0,CC-BY-4.0,CC-BY-SA-4.0` | SPDX ids accepted in uploads |
| `REIWAND_PACKAGE_MAX_MIB`, `REIWAND_USER_QUOTA_MIB` | `256`, `2048` | Package size and storage per account |
| `REIWAND_UPLOADS_PER_DAY` | `20` | Upload sessions per account and day |
| `REIWAND_ENTRY_DAYS` | `7` | Lifetime of each signed index (1 to 14, capped by the root) |
| `REIWAND_RESIGN_HOURS` | `24` | Re-sign the index at least this often (at most half the index lifetime) |
| `REIWAND_PUBLISH_DELAY_SECS` | `30` | Wait after a moderation decision before signing, to batch changes |
| `REIWAND_LEGAL_OPERATOR`, `REIWAND_LEGAL_CONTACT`, `REIWAND_LEGAL_JURISDICTION`, `REIWAND_LEGAL_DMCA_AGENT` | empty | Shown on `/legal` and in the terms |
| `REIWAND_LOG_CLIENTS` | `off` | Abuse log of client addresses: `off`, `prefix` (IPv4 /24, IPv6 /48) or `full` (address and user agent) |
| `REIWAND_LOG_CLIENTS_DAYS` | `7` | Days the abuse log keeps (1 to 30) |
| `REIWAND_NOTICE_DAYS` | `180` | Days a legal notice keeps the sender's name, email and signature (1 to 730) |
| `REIWAND_TAKEDOWN_DAYS` | `90` | Days a removed or revoked package stays in `held/` as evidence and for a counter-notice (1 to 3650) |
| `REIWAND_RATE_LIMITS` | `true` | In-memory rate limits |
| `REIWAND_TLS` | `off` | `self-signed` serves HTTPS itself with a certificate of its own, made on the first start in `tls/` (ECDSA P-256, valid for the host of `REIWAND_PUBLIC_URL` and `localhost`); clients reach it only by its pinned fingerprint. The same port still answers plain HTTP from loopback (the health check) and redirects plain requests from elsewhere to `REIWAND_PUBLIC_URL`. Until the setup stores a network choice |
| `REIWAND_LOG` | `info` | Log filter |

Logs go to standard output and never contain addresses, user agents, tokens, bodies or emails.
With `REIWAND_LOG_CLIENTS` the server writes `logs/clients-<date>.jsonl` (`at`, `ip`, `agent`,
`method`, `route`, `status`) and deletes files older than the retention.

### Data directory

```
db/reiwand.sqlite      accounts, packages, moderation (WAL)
blobs/<sha256>.wpak  uploads waiting for review
thumbs/<sha256>.jpg  640 px previews re-encoded by the validator
held/<sha256>.wpak   removed or revoked packages (and their previews) for REIWAND_TAKEDOWN_DAYS, not served
repos/<name>/        each repository of the server (docs/specs/repo.md), built with reiwand-format; the first one is `main`
keys/<name>.online.key  its online signing key (0600)
keys/<name>.root.sealed  its root key sealed with XChaCha20-Poly1305 (0600): 24-byte random nonce followed by the ciphertext
keys/seal.key        32 random bytes (0600), made on first use: the sealing key of every `*.root.sealed`
keys/mail.secret     password of the mail server (0600), only when mail is set up
tls/cert.der, key.der  certificate and key of REIWAND_TLS=self-signed (0600)
tmp/ logs/ backups/
```
