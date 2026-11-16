# Export and import file (`.reiwand`)

The `reiwand-carry` crate defines the file that moves a device's configuration to another device. It is a pure library (no D-Bus, no network) used by the daemon, the CLI and the phone. Exporting and importing is always a file the user moves as they like; there is no account, server or QR involved.

## Container

A `.reiwand` file is a zip archive:

- `carry.json`: the manifest. Fields: `format` (1), `program`, `version`, `created` (text given by the caller), `sections` (names of the sections held), `encrypted`, and `sums` (SHA-256 of every file under `files/`).
- `files/content.json`: the data of every section.
- `files/own/<id>`: one file per own wallpaper (stored, not recompressed). `<id>` only has letters, digits, `.`, `-` and `_`.

Unencrypted files carry no signature: integrity comes from the sums in the manifest. They catch accidents and tampering with one file, not a deliberate rewrite of the whole archive. A file that asks for secrets is always encrypted.

## Sections

- `repositories`: address, `repo_id` anchor and server origin of each added repository.
- `providers`: photo providers (kind, name, configuration) without secrets.
- `lists`: name and items (`repo_id` optional, `id`).
- `favourites`, `ratings`, `tags`: per item.
- `installed`: repository wallpapers as references `(repo_id, id, version)`. They are downloaded again on import and never carried.
- `settings`: portable settings only.
- Optional `own`: the files of the user's own wallpapers.
- Optional `secrets`: account sessions per repository and provider keys.

## Settings of this device

These keys are never exported and are dropped on import even if a file holds them: playback volume and mute, `playback.software_parallax`, `playback.max_fps`, the thumbnail cache size, the model download permission, `render.gpu`, `playback.quality`, the layer namespace, remote covers, the private network switch, battery pause (`pause.on_battery`) and everything that names a screen of this machine (`outputs.`, `screens.`, `device.` prefixes and the per-screen `playback.quality.<output>` and `transition.kind.<output>`). The list is `DEVICE_KEYS` and `is_device_local` in the crate; `portable` filters a settings map.

## Encryption

When the user ticks secrets, the whole zip is encrypted with a password chosen at export. Key: Argon2id (default 64 MiB, 3 passes; the cost is stored in the header and bounded on read) over a random 16-byte salt. Cipher: XChaCha20-Poly1305 in 64 KiB segments. The nonce is a 19-byte random prefix, the segment number and a flag on the last segment, so reordering and truncation are detected; the header is authenticated with every segment. A file starts with `REIWANDE` and a header version byte. Errors are distinct: needs a password, wrong password or damaged, truncated.

## Hostile input

Reading refuses: more than 100,000 entries, an entry above 8 GiB, a total above 16 GiB, an expansion ratio above 200 for entries above 1 MiB, a JSON file above 32 MiB, names with `..`, absolute paths, backslashes or `:`, symbolic links, own wallpaper ids that are not plain names, and a `format` above the one the program knows (error asking to update). Sizes are enforced while reading, not only from the declared values. `Limits` makes them adjustable.

## Merge rules

`plan(content, local, pick)` is a pure function that returns the changes to apply; `preview(content, local)` returns, per section, how many items the file holds, how many are new, skipped and in conflict, for the import checklist. Importing adds and never replaces:

- A repository with a `repo_id` already present is skipped; the same for a provider with the same kind and name.
- Favourites and tags are added; a rating is set only when the item has none.
- Installed references already installed are skipped.
- A list with the same name (ignoring case and spaces) is joined (only missing items are added) or kept as a second list, which gets the suffix passed by the caller (the crate has no interface texts) and a number if that name is also taken.
- Settings only when `pick.settings` is set, and never device-local ones.
- Own wallpapers already present by id are skipped; secrets are not added for a repository or provider key that already has one.

## Limits

An encrypted file is built and opened in memory, so very large own wallpapers in an encrypted export need that much memory. Plain files stream from and to disk.
