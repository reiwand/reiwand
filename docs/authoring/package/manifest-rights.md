# Languages, licenses and ratings

On this page: languages, licenses, rating, photosensitivity, origin and inputs.

## Languages

A text can be plain, a map from language to text (`{ "en": "…", "de": "…" }`) or a key `"@key"` of
`strings`. Languages are tags such as `en`, `de` or `pt-BR`. The program picks the first one that
matches the languages the user has configured, then `default_locale`, then `en`. Common labels
(`color`, `speed`, `volume`, `quality`…) are already translated by the program: they are written
`"@reiwand.color"` and need no `strings`.

## Licenses

`license` is an SPDX expression: one or several identifiers joined with `AND`, `OR` or `WITH`
(`CC-BY-4.0 OR MIT`). The program only requires that it be well written (up to 256 characters).
What it does do with it is decide whether someone can **edit a copy** of your package.

| License | What it allows |
|---------|----------------|
| `CC0-1.0` | Everything, without conditions: use, change and redistribute. |
| `CC-BY-4.0` | Use, change and redistribute **with credit** to its author. |
| `CC-BY-SA-4.0` | The same, and what is derived is shared under the same license. |
| `CC-BY-NC-4.0`, `CC-BY-NC-SA-4.0` | With credit and **no commercial use** (the second, in addition, sharing alike). |
| `CC-BY-ND-4.0`, `CC-BY-NC-ND-4.0` | Redistribute without changes. **They do not allow derived works.** |
| `MIT`, `Apache-2.0` | For code (scripts, shaders): use and change with notice of the license. |
| `GPL-3.0-or-later`, `AGPL-3.0-or-later`, `LGPL-3.0-or-later` | With copyleft: what is derived is published under the same license. |
| `LicenseRef-<name>` | Conditions of your own, which you describe in the package. |

**In the editor** the license is chosen from a list (*Wallpaper details…*) of ten, each with a line
that says what it allows: `CC0-1.0`, `CC-BY-4.0`, `CC-BY-SA-4.0`, `CC-BY-NC-4.0`,
`CC-BY-NC-SA-4.0`, `CC-BY-ND-4.0`, `CC-BY-NC-ND-4.0`, `MIT`, `Apache-2.0` and `GPL-3.0-or-later`.
You can type any other SPDX expression by hand (for one that is not in the list, the editor only
says it is an expression of your own). If you leave it empty, it reminds you to choose the one
that says what others can do with your package. From the terminal: `reiwand-ctl scene act FOLDER
manifest`.

What the program does with each one: when opening a copy to edit it, it works out whether the
license allows derived works and what conditions it carries (credit, share alike, no commercial
use, copyleft, own conditions). The legal text of the license is what rules; the program only
warns.

**Licenses that do not allow derived works.** A license does not allow them if all its
alternatives carry `ND` (for example `CC-BY-ND-4.0`) or if it is empty. The verdict reads only the
text of the license: a `LicenseRef-` passes with a note of own conditions, because that free text
cannot be read; check those conditions yourself. Editing a copy of such a package is rejected and
nothing is written, unless you confirm that the package is yours or that its author allows it:

- in the app, *Edit a copy…* always shows the license of the original, with a line that says what it
  allows and what conditions it brings, and a tip to put your name among the authors. The checkbox
  *I made this package or its author allows a copy* **only appears when the license does not allow
  derived works**; with another license the dialog only informs (and, if the license asks for
  something, such as credit or share alike, it lists it) and there is no checkbox to tick. When it
  appears, it does not go on without ticking it. When adding the copy to a static repository
  (dialog *Add to a repository in a folder*) the warning appears again, with the same checkbox, and
  `pack check`, `pack check` and the *Problems* panel repeat it as a warning;
- in the terminal, `reiwand-ctl scene open FOLDER --from package.wpak --mine`. Without `--mine` it
  fails with "<license> does not allow derived works".

A server of your own only accepts the licenses of its list (by default `CC0-1.0`, `CC-BY-4.0` and
`CC-BY-SA-4.0`; whoever runs it can change it). A static repository accepts any.

## Rating

`content.rating` is `general` or `mature` (for adults). You choose it; the program never deduces
it. `mature` requires naming at least one reason, without repeats, in `descriptors`: `sexual`,
`violence`, `gore` or `language`. Apps blur the previews of a `mature` package until the user
decides to show them, and can hide it from searches. A server of your own rejects `mature`
packages. Only the current version of the format, `reiwand.wpak/3`, is opened.

## Photosensitivity

`photosensitive` is `true` when the package flashes or shows fast patterns that can trigger
seizures. It is set by hand. As a help, the editor has the *Measure the flashes* button, and
`pack check` measures the scene if you did not mark it: it draws a few seconds and warns if there
are flashes of more than three times a second over a tenth of the image ("the scene flashed more
than N times a second near second S of a render; if people can see that, set photosensitive"). It
is a hint, not a proof: sound, pointer and clock decide the real image. With `true`, the app,
`reiwand-ctl` and the Android app warn before applying it, and the daemon does not apply it until
the user confirms; playlists skip it.

## Origin

| `origin.kind` | When | What else it carries |
|---------------|------|----------------------|
| `original` | A work made for Reiwand. | Nothing. |
| `derived` | You remade it by hand from other works. | `derived_from`: 1 to 64 `{ author, title?, url?, license? }` (without `<` or `>`). |
| `imported` | A conversion from another format for your personal use. | Nothing. **It is never published**: `repo add` and the server reject it. |
| `provider` | An image or a video downloaded from an online source. | `provider: { name, item_url, credit, terms_url? }`. It is only published with `CC0-1.0`, `CC-PDM-1.0`, `CC-PDDC`, `CC-BY-2.0`, `CC-BY-3.0` or `CC-BY-4.0`. |

Anyone can add `source_url` (the `https://` page of the original work). Apps show the origin, the
provider's credit and the packages the files with credits come from.

## Inputs

`inputs` says what the package asks of the user: the computer's sound (`audio`), the pointer
(`pointer`), clicks (`clicks`), what plays in the media player (`media`) and the network
(`network`, only for web pages). For scenes, the check works out which inputs the scene uses with
any value of its properties and **rejects** a manifest that does not declare them, saying which
one is missing and where it is used: `the scene uses pointer at /scripts/0 of scene.json; add
"pointer"`. If you declare an input that nothing uses, it warns. A package only receives the
inputs it declares. The keyboard never reaches a package.
