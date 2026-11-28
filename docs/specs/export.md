# Export and import file (`.reiwand`)

The `reiwand-carry` crate defines the file that moves a device's configuration to another device. It is a pure library (no D-Bus, no network) used by the daemon, the CLI and the phone. Exporting and importing is always a file the user moves as they like; there is no server, sync or QR involved.

## Container

A `.reiwand` file is a zip archive:

- `carry.json`: the manifest. Fields: `format` (2), `program`, `version`, `created` (text given by the caller), `sections` (names of the sections held), `encrypted`, and `sums` (SHA-256 of every file under `files/`).
- `files/content.json`: the data of every section.
- `files/own/<id>`: one file per own wallpaper (stored, not recompressed). `<id>` only has letters, digits, `.`, `-` and `_`.

Unencrypted files carry no signature: integrity comes from the sums in the manifest. They catch accidents and tampering with one file, not a deliberate rewrite of the whole archive. A file that asks for secrets is always encrypted, and reading refuses a file with a `secrets` section that is not encrypted (`secrets are only carried in an encrypted file`).

## Sections

- `repositories`: address, `repo_id` anchor and server origin of each added repository.
- `providers`: photo providers (kind, name, configuration) without secrets.
- `lists`: name and items (`repo_id`, `null` for an item with no repository, and `id`).
- `favourites`, `ratings`, `tags`: per item.
- `installed`: repository wallpapers as references `(repo_id, id, version)`. They are downloaded again on import and never carried.
- `settings`: portable settings only.
- `own`, only when the user ticks it: the files of the user's own wallpapers.
- `secrets`, only when the user ticks it: account sessions per repository and provider keys.

## Settings of this device

These keys are never exported and are dropped on import even if a file holds them: playback volume and mute, `playback.software_parallax`, `playback.max_fps`, the thumbnail cache size, the model download permission, `render.gpu`, `playback.quality`, the layer namespace, remote covers, the private network switch, the daily check for a new version (`updates.auto`),
what this device consents to (`playback.allow_flashing`, `playback.scene_capture`,
`library.show_mature`, `repos.auto_update`: a file never turns them on or off), battery pause (`pause.on_battery`), the HDR choice (`playback.hdr`), the bezel size (`layout.bezel_mm`) and everything that names a screen of this machine (`outputs.`, `screens.`, `device.`, `layout.`, `rgb.` prefixes and the per-screen `playback.quality.<output>`, `playback.hdr.<output>` and `transition.kind.<output>`). The list is `DEVICE_KEYS` and `is_device_local` in the crate; `portable` filters a settings map.

## Encryption

When the user ticks secrets, the whole zip is encrypted with a password chosen at export. Key: Argon2id (64 MiB, 3 passes; the cost is stored in the header and bounded on read) over a random 16-byte salt. Cipher: XChaCha20-Poly1305 in 64 KiB segments. The nonce is a 19-byte random prefix, the segment number and a flag on the last segment, so reordering and truncation are detected; the header is authenticated with every segment. A file starts with `REIWANDE` and a header version byte. Errors are distinct: needs a password, wrong password or damaged, truncated.

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

## In the daemon (`Carry1`)

The daemon (`apps/reiwandd/src/carry`) fills the content with the real state: added repositories (address, `repo_id` and origin; the examples repository does not travel), providers, lists, favorites, ratings, tags, installed packages as references (`repo_id`, `id`), the portable settings and, if checked, your own wallpapers with their file. Smart lists travel with their search and their rules (without items). Repositories carry the pinned fingerprint of the certificate for their origin, which the import never trusts by itself: a repository whose certificate this computer does not trust is left pending, as when it is added by hand, until the user confirms the fingerprint, and a fingerprint already pinned here is never replaced. Provider keys and account sessions leave only with `secrets` and always encrypted. On import, each computer gets its own session with `POST /api/v1/auth/session/copy` (the session in the file is not reused); if the server does not respond or rejects it, the account is left as pending "sign in again" and the rest is imported.

Importing only adds and follows this order: repositories, providers and keys, packages, your own wallpapers, library (lists, favorites, ratings, tags) and settings. A repository that asks for confirmation is left as pending and does not make the rest fail. What refers to a wallpaper that is not here is counted as unresolved. With `lists = join` a second import duplicates nothing; with `both` each import creates another copy with the suffix (` (imported)`, ` (imported) 2`…).

`reiwand-ctl export FILE [--own] [--secrets] [--password-file FILE]` and `reiwand-ctl import FILE [--list] [--take SECTION]… [--lists join|both] [--suffix TEXT]`; `--list` only shows what it brings. See `dbus.md`, `Carry1`.

## In the desktop app

Settings > Device > "Move my settings" offers "Export my configuration…" and "Import a configuration…". Export lets you check your own wallpapers (it shows the size they add up to) and the passwords and keys (it asks for the password twice and encrypts the whole file), and runs `Carry1.Export` as a task. Import opens the file, asks for the password if it is encrypted, shows the preview per section (new, already present, name conflicts; everything checked except the settings), lets you choose "Join" or "Keep both" for lists with the same name and runs `ImportChosen` as a task. The result lists what was added and what is left to resolve, with buttons to the corresponding page.

## On the phone

`crates/reiwand-android` (`carry`) builds the same content as the daemon and applies the same order and the same report. The file is chosen with the system file picker: Kotlin copies the document to the app's cache (or from it) and Rust works on that path. Operations of `Native.account`: `carry_preview`, `carry_export` (`own`, `secrets`, `password`) and `carry_import` (`take`, `lists`, `suffix`). A file exported on the desktop is imported on the phone and the reverse; provider keys and sessions follow the same rules. Account sessions in a file are not applied on the phone: they are noted so the person signs in again. The phone has no terminal subcommand: parity is that of the two programs on the same file.

## Format 2: explicit values

- `carry.json` and `files/content.json` carry all fields. A value without content is written as `null`; only the blocks of unused features (`List.rules`, `Content.secrets`) may be missing, and a present block carries all its fields. The reader (`reiwand_carry::read`) rejects the file and names the path of the missing field.
- A file of another format, also encrypted, is rejected: the message says which format it has and that only format 2 is read, and asks to export it again with this version. There is no conversion.
- Table of prefilled values: `reiwand_carry::PREFILL` (one per format). `Option` fields are classified as `block` (block of an unused feature: it may be missing or `null`; if present, it carries all its fields: `List.rules`, `Content.secrets`) or `valueBox` (value: the `null` means something and is always written: `Item.repo_id`, `Repository.pin`, `List.query`).

## Phone: own settings

The phone writes a complete `settings.toml` on first start, with `format = 2` and the values of `own::PREFILL`. An old file or one with a value outside what is allowed is not read: the application says so and asks to delete the data to start with the factory settings. `providers.json` files are also read without omissions: a missing field is rejected naming its path.
