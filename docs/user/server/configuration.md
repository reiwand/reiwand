# Image and configuration

This page explains which container image the server uses, how to check its signature, how to
build it yourself, and the environment variables that configure it.

## The published image

`compose.yaml` pulls `ghcr.io/reiwand/reiwand-server` with the tag of `REIWAND_VERSION` (`latest`
if not set).

Each version publishes that image:

- Platform `linux/amd64`, with the tags `<version>` and `latest`.
- Signed with cosign without a key of its own, like the files of *Releases*.
- With the BuildKit SPDX SBOMs attached (the packages of the image and of the build stage) and its
  provenance.
- With a signed SPDX attestation with the Rust dependencies of the sources it was built from.

## Pinning and checking an image

To pin a specific image:

1. Take its digest.
2. Check the signature and the attestation.
3. Put the digest in place of the tag (`image: ghcr.io/reiwand/reiwand-server@sha256:<digest>`).

```sh
IMAGE=ghcr.io/reiwand/reiwand-server
DIGEST=$(docker buildx imagetools inspect "$IMAGE:<version>" --format '{{ .Manifest.Digest }}')
ID=https://github.com/reiwand/reiwand/.github/workflows/ci.yml@refs/heads/master
ISSUER=https://token.actions.githubusercontent.com
cosign verify "$IMAGE@$DIGEST" --certificate-identity "$ID" --certificate-oidc-issuer "$ISSUER"
cosign verify-attestation "$IMAGE@$DIGEST" --type spdxjson \
  --certificate-identity "$ID" --certificate-oidc-issuer "$ISSUER" > reiwand-server.sources.spdx.dsse
docker buildx imagetools inspect "$IMAGE@$DIGEST" --format '{{ json .SBOM }}' > reiwand-server.sbom.json
```

## Building the image yourself

To build the image from this repository instead of pulling it, add the build file:

```sh
docker compose -f compose.yaml -f deploy/compose.build.yaml up -d --build
```

A server built that way identifies itself with the version of `Cargo.toml` and the suffix
`-local`. With `REIWAND_VERSION` in the environment it shows that of a published version.

## Environment variables

The configuration goes in environment variables, in the `.env` next to `compose.yaml`.

- `.env.sample` lists them all with their default value.
- `VARIABLE_FILE` reads the value from a file, for secrets.
- `REIWAND_VERSION` and `REIWAND_BIND` are compose-only.

| Variable | Default | Use |
|----------|---------|-----|
| `REIWAND_PUBLIC_URL` | required in compose | Public URL of the server |
| `REIWAND_VERSION` | `latest` | Tag of the image compose pulls |
| `REIWAND_BIND` | `127.0.0.1:8080` | Address and port that compose publishes |
| `REIWAND_REGISTRATION` | `invite` | `closed`, `invite` or `open` |
| `REIWAND_AUTO_PUBLISH` | `trusted` | Publish without review: `never`, `trusted` or `all` |
| `REIWAND_LICENSES` | `CC0-1.0,CC-BY-4.0,CC-BY-SA-4.0` | Accepted licenses |
| `REIWAND_LEGAL_OPERATOR`, `REIWAND_LEGAL_CONTACT` | empty | Operator and contact of the legal pages |
| `REIWAND_LOG_CLIENTS` | `off` | IP in an abuse log: `off`, `prefix` or `full`; deleted after `REIWAND_LOG_CLIENTS_DAYS` days (7) |
| `REIWAND_NOTICE_DAYS` | `180` | Days that a legal notice keeps the name, email and signature of whoever sends it |
| `REIWAND_TAKEDOWN_DAYS` | `90` | Days that a withdrawn package is kept out of public view, as evidence and for a counter-notice |
| `REIWAND_TLS` | `off` | `self-signed`: serves HTTPS with a certificate of its own, which clients accept only when you confirm its fingerprint |

The complete list of variables and the HTTP API are in
[`docs/specs/server.md`](../../specs/server.md).
