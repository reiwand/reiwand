# Formats and Package signatures

On this page: formats, package signatures.

## Formats

All metadata is compact JSON with fixed field order; every field is written and a file that lacks one is
refused naming it (the unused optional blocks `mirrors` of a root, `authors` and `featured` of an entry and the
release blocks `preview`, `animated`, `content` and `requires` may be absent, or `null`, and a block that
is present carries all its fields); unknown fields are rejected, except in
the releases of the shards, whose readers ignore fields and
values they do not know (also inside `content` and `requires`; an unknown rating counts as
`mature`). Times are RFC 3339 UTC (`2027-01-15T08:00:00Z`).

`root/<N>.json` (at most 512 KiB):

| Field | Rule |
|-------|------|
| `format` | `wpak-repo-root/2.0` |
| `genesis` | `null` in version 1, the `repo_id` after |
| `version` | equals `N` |
| `expires`, `name` | expiry and display name |
| `keys` | map of fingerprint to `{"minisign": "<base64 public key>"}`; fingerprints must match |
| `roles` | `{"root": {keys, threshold}, "online": {keys, threshold}}` |
| `authors` | `[{"id": "[a-z][a-z0-9_-]{0,31}", "keys": [fingerprint…]}]`, unique ids |
| `policy` | `{require_author_signature, max_package_size, entry_max_days}` and `authors` (`null` when the root signs the authors, `"online"` when the online key does) |
| `mirrors` | a block, absent or `null` without mirrors: at most 16 `https://` or `file://` base URLs; `file://` mirrors are used only by clients that added the repository itself from a `file://` URL or a folder, and skipped for repositories fetched over the network |

`entry/<N>.json` (at most 128 KiB): `format` (`wpak-repo-entry/2.0`), `repo_id`, `version`
(= `N`), `timestamp`, `expires` (`0 < expires - timestamp ≤ entry_max_days`), `root`
(`{version, sha256}` of the current root), `shard_bits` (0 to 8), `shards` (exactly
`2^shard_bits` `{sha256, size}` in shard order), `revoked` (`{sha256, size}`) and, exactly when the
root sets `policy.authors = "online"`, `authors` (`{sha256, size}` of the online authors file), and,
when the operator features packages, `featured` (`{sha256, size}` of the featured file). Each of
these files is at most 32 MiB and all of them together at most 256 MiB: clients refuse an entry that
lists more before downloading anything, and `publish` refuses to write one.

`index/<sha256>.json`: `format` (`wpak-repo-shard/4.0`), `repo_id`, `shard` (its position) and
`packages`: `[{id, author, versions: [release…]}]`, with every release of every package. A package
belongs to the shard given by the first `shard_bits` bits of SHA-256 of its id, so every id appears
once in the repository. Ids follow the manifest rules; versions are canonical SemVer, unique and
ascending. A release holds `version`, `kind`, `title`, `description`, `tags`, `license`, `authors`,
`size`, `sha256` (whole `.wpak`), `manifest_sha256`, `preview` (`{sha256, size, ext}` or `null`),
`published`, `schema`, its `default_locale`, `changes`, `content`, `photosensitive`, `requires`
(with the `packages` it needs), the `facets` the repository computed, `animated` (the animated
preview, like `preview`), `platforms` and, for a preset, the id of its `base`, so that clients can
filter before downloading; `title`, `description` and `changes` are text or `{language: text}` with
every `@key` resolved. A client skips the
releases whose `kind` it does not know, and a package left with none, so that a kind added later
never stops it from reading the rest of the shard; the fields it does not know in a release are
ignored, while the signed hashes still cover them.

A release may carry `hidden: true`: the moderators of the repository
took the package out of browsing. Clients leave it out of `browse` unless a version is installed,
and keep installing and updating it.

A release may also carry `listed: false`: the version is unlisted. Clients
leave it out of `browse` and its search unless a version of the package is installed, and install it
when asked by id. `reiwand-ctl repo add --unlisted` adds versions unlisted, and `repo unlist DIR ID [--version V]`
and `repo list-again` change it for a package or one version; publish next. Unlisting is not the
moderators' `hidden` flag (`reiwand_format::hide`), which no subcommand of `repo` sets.

The desktop app operates a static repository from the *Static repository* tab of *Repositories*: it
opens the folder (name, `repo_id` and `stock(dir)`: each package with its author, whether it is
featured and its note, and each version with its title, whether it is listed and whether it is
revoked), adds `.wpak` files with *Add unlisted* as `repo add --unlisted` does, unlists or lists
again a package or one version and features (with an optional note) or unfeatures a package, each
after a confirmation that says what the CLI says, through the same `reiwand_format` functions. *Publish…*
takes a secret key file of an online key, its password when it has one, and the days until the index
expires (prefilled 7), and calls `publish` as `reiwand-ctl repo publish` does. The same tab revokes a
package or one version with a reason and a message (`stall::revoked`, as `repo revoke`), withdraws a
revocation (`stall::unrevoked`, as `repo unrevoke DIR ID [--version V]`, which `unrevoke` the whole
package without a version) and, from *Author keys…*, revokes an author key by its fingerprint
(`stall::key_revoked`, as `repo revoke-key`). *New repository…* writes `root/1.json` in an empty
folder from a name, root and online `.pub` files, the root threshold, an author with its keys and the
policy (`stall::made` with a `reiwand_format::Charter`, as `repo init`); *Root…* shows the version of the
root, how many root keys signed it of its threshold and its address, signs it with a root key file
and its password (`stall::root_signed`, as `repo sign-root`) and *Rotate the root…* writes the next
root with other keys, removed or added authors and another policy (`stall::rotated`, as `repo
rotate-root`). `stall::opened` also answers the root status, the fingerprints of the root, online and
author keys, the revoked keys and the policy. Every change to the
catalog runs off the window thread, one after another, and answers when done; the page shows that it
is busy meanwhile and keeps the other changes and *Open* disabled until then. The phone has no tool to create, sign or
publish a repository: it is a client only. From *Repositories* in *Sources* it adds a repository by
its address and its fingerprint, or accepts once a proposal that arrives from another device with its
key; it reads the signed index (see *Client update*), browses its packages in *Explore* with their
previews, installs them checked against that index (see *Package installation*) and checks them for
updates.

`featured/<sha256>.json` (optional): `format` (`wpak-repo-featured/2.0`), `repo_id` and `items`: at
most 64 `{id, note}` (`note` is `null` without one), ids of packages of the repository, each once, with a note of at most 200
characters on one line without `<` or `>`. The entry cites it as `featured` (`{sha256, size}`), so the
online signature covers it.
Clients mark these packages as featured when browsing and list them first. Nothing in a manifest can
feature a package: only the operator does, with `reiwand-ctl repo feature DIR ID [--note TEXT]` and
`repo unfeature DIR ID` (`feature(dir, id, note, on)`), then `publish`.

`authors/<sha256>.json` (only with online authors): `format` (`wpak-repo-authors/2.0`), `repo_id`
and `authors`: `[{id, keys: {fingerprint: {minisign}}, rotations: [{from, to, sig}]}]`, ids as in
the root and unique, `to` listed in `keys`, `sig` a minisign block of the key `from` over
`wpak-author-rotate <id> <from> <to>`. The entry signed by the `online` key pins it, so it is
signed indirectly.

`revoked/<sha256>.json`: `format` (`wpak-repo-revoked/2.0`), `repo_id`, `packages`
(`[{id, versions, reason, message}]`, `"*"` revokes every version) and `keys`
(`[{fingerprint, reason}]`). Reasons: `malware`, `license`, `takedown`, `broken`, `other`.
A revoked key invalidates every signature it made even while a root still lists it.

## Package signatures

An author signs the exact bytes of `manifest.json`; the signature is stored in the package as
`manifest.json.minisig` with the trusted comment `wpak manifest <id> <version> files:<digest>`,
where `<digest>` is the SHA-256 of one line `<sha256 of the file> <path>` per file of the package
except the signature, sorted by path. Verifiers compute the digest from the package they hold, so
a signed manifest moved into a package with other files (by a mirror, a stolen online key or an
uploader) no longer verifies, and a grant kept for the signing key (`dbus.md`, `Inputs1`) never
passes to other content. The manifest part can be checked by hand with
`minisign -Vm manifest.json -p author.pub`.
