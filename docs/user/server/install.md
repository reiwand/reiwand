# Installing the server

`reiwand-server` can be brought up in two ways, with Docker or without it, and both leave the same
server. This page covers both ways, the first administrator and the first repository.

You do not need a browser on the server computer: everything the administration web asks for has
its terminal command, below.

## What the server is

- A single binary in a container without a shell.
- It runs as root by default. With a `user:` line in `compose.yaml` it can run as another user.
- Everything lives in the `./data` folder, mounted as `/data`. The backup is copying that folder
  (see [Backups and data from another version](backups-and-versions.md)).
- It sends no telemetry and does not connect to third parties.

## With Docker

This uses the published image.

1. Copy the sample settings and edit `REIWAND_PUBLIC_URL`, the only required variable.
2. Start the server; this pulls the published image.
3. Create the repository.
4. Get the one-time token of the first administrator.

```sh
cp .env.sample .env                           # edit REIWAND_PUBLIC_URL, the only required variable
docker compose up -d                          # pulls the published image and starts it
docker compose exec reiwand reiwand-server repo init --name "Home wallpapers"
docker compose exec reiwand reiwand-server setup-token   # one-time token of the first administrator
```

The data ends up in `./data`.

> **Note:** In the rest of these pages, `reiwand-server …` is
> `docker compose exec reiwand reiwand-server …` if you use Docker.

## Without Docker

This uses the `reiwand-server` binary, which is built with Rust (the minimum version is in
`Cargo.toml`) in a copy of this repository. The Docker image is built with just that, without Qt
or PipeWire.

```sh
cargo build --release -p reiwand-server          # leaves target/release/reiwand-server
export REIWAND_DATA=$HOME/wallpaper-server       # without this it uses /data
export REIWAND_PUBLIC_URL=http://127.0.0.1:8080  # the address the computers will type to reach it
export REIWAND_LISTEN=127.0.0.1:8080             # without this it listens on all interfaces (0.0.0.0:8080)
target/release/reiwand-server repo init --name "Home wallpapers"
target/release/reiwand-server serve              # stays in the foreground; stop it with Ctrl-C
```

## The first administrator

While there is no administrator, `serve` writes a line like this to its log:

`no admin account yet: open http://127.0.0.1:8080/setup and enter the one-time setup token reiwand_s_…`

With Docker, find it with `docker compose logs reiwand | grep setup`. `reiwand-server
setup-token` gives another token at any time.

**With a browser:**

1. Open `<REIWAND_PUBLIC_URL>/setup`.
2. Paste the token.
3. Choose the name and a password of 10 characters or more.

**Without a browser**, the same page is a form that is submitted with `curl`:

- The `303` response says the administrator already exists.
- `cookies` stores its session for the following steps.
- `$REIWAND_PUBLIC_URL` is the address of the server. If you do not have it exported, type it,
  for example `http://localhost:8080` with Docker.

```sh
curl -sS -o /dev/null -w '%{http_code}\n' -c cookies --data-urlencode "token=<token>" \
  -d handle=admin --data-urlencode "password=<password of 10 characters or more>" \
  "$REIWAND_PUBLIC_URL/setup"
curl -sS -c cookies -o /dev/null -d handle=admin --data-urlencode "password=<password>" \
  "$REIWAND_PUBLIC_URL/login"                    # a new session, later on
curl -sS -b cookies -H 'content-type: application/json' -d '{"role": "user"}' \
  "$REIWAND_PUBLIC_URL/api/v1/admin/invites"     # an invitation for another account: prints its code and its url
```

## The repository and its keys

`repo init` creates the repository keys. The root key is kept by the server, encrypted, in its
data folder: it goes into the backups and is never shown.

- For more repositories, *Repositories* in the administration header asks only for a name and
  builds the address itself.
- If `repo init` answers `error: a repository already exists`, that server already has its
  repository and there is nothing to do. `reiwand-server repo status` says its state and its
  address.
- The server uses the online key to sign the index again every day. Each signature expires after
  7 days.

## Where things are on the web

| Page | What it does |
|------|--------------|
| *Administration* | Creates invitations (also possible with the API, as above) |
| *Account* | Tokens, devices linked with a code, and uploads of `.wpak` packages of both manifest versions |

Next: [Repositories and accounts](repositories-and-accounts.md).
