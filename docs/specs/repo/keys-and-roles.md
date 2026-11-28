# Keys and roles, Signature files, Layout and Identity and URL

On this page: keys and roles, signature files, layout, identity and url.

## Keys and roles

| Role | Signs | Threshold | Prefilled lifetime |
|------|-------|-----------|------------------|
| `root` | `root/<N>.json` | t of n, 1 to 8 keys | 365 days (30 to 1096) |
| `online` | `entry/<N>.json` | 1 | 7 days, at most `policy.entry_max_days` (1 to 90, prefilled 14) |
| author | `manifest.json` inside a package | 1 | no expiry; revoked through `revoked` |
| online author | the same, listed in `authors/<sha256>.json` instead of the root | 1 | the same |

- A key is identified by its fingerprint: lowercase hex SHA-256 of the 32 Ed25519 public key
  bytes (bytes 10..42 of the decoded minisign public key). The minisign key id is only a hint.
- A key belongs to at most one role or author. Root keys stay offline.

## Signature files

`<file>.minisig` holds 1 to 16 standard four-line minisign blocks, concatenated. A file with
one block verifies with stock minisign tools.

- Every block must parse and carry the same trusted comment, or the whole file is rejected.
- Each block is tried against every key of the role; the threshold counts distinct key
  fingerprints, never blocks.
- Trusted comments bind role, version and repository: `wpak-repo root <N> <repo_id>`,
  `wpak-repo entry <N> <repo_id>`,
  `wpak manifest <id> <version> files:<digest>` for package authors and `wpak-author-rotate <author> <from> <to>`
  for the rotations of online authors (signed text and trusted comment alike).

## Layout

```
root/<N>.json, root/<N>.json.minisig    immutable, never deleted
entry/<N>.json, entry/<N>.json.minisig  immutable
entry/latest                            "<N>\n", unsigned hint, the only mutable file
index/<sha256>.json                     shards, immutable
revoked/<sha256>.json                   revocations, immutable
featured/<sha256>.json                  packages the operator features, immutable, optional
authors/<sha256>.json                   online authors, immutable, only with policy.authors = online
previews/<sha256>.<ext>                 preview images copied from packages
packages/<id>/<version>.wpak            packages, immutable once published
catalog.json                            operator state, not used by clients
```

Serve `entry/latest` with `Cache-Control: no-cache` and everything else as immutable.
Mirrors copy immutable files first and `entry/latest` last.

## Identity and URL

- `repo_id` is the SHA-256 of the exact bytes of `root/1.json`. It never changes.
- Address: `https://example.org/r/wallpapers`, pasted as it is. The first time, the client reads
  `root/1.json`, checks that it is signed by its own root keys and keeps its hash as the `repo_id`
  of the repository for good (the anchor: trust on first use, as the secure connection to the server vouches for that first root); from then on every refresh must chain from that root.
  The connection must have a certificate valid for a public authority. A home server with its own
  certificate is added only after the user confirms its fingerprint, which is then pinned for that
  server address alone (see `dbus.md`, `Repos1.AddTrusting`); a different certificate later is
  refused and warned. Plain `http://` is accepted only for this computer.
- An address has no parameters. One with `?fingerprint=` or `?repo_id=` (from a 1.x link) is refused with
  a message that says to use the plain address.
- If a refresh finds another `repo_id` at the address, or a root that does not chain from the stored
  one, nothing is applied and the repository shows the warning `SignatureChanged`.
- An address that answers `/.well-known/reiwand-server` but has no `root/1.json` is a server, not a
  repository: the add fails with `NotARepository`, naming the page where the server lists its
  repositories  (`repositories` of the discovery document).
- `reiwand-ctl remote add` also takes a folder (`remote add FOLDER`): the CLI turns it into the `file://`
  URL of its absolute path, and the anchor rules are the same as for any URL.
- A package is identified by `(repo_id, id)`.
