# Presets, Shared resources, Languages and Markup

On this page: presets, shared resources, languages, markup.

## Presets

A `preset` package carries values for another package, its base, and the pictures or videos those
values name; it is published, versioned and credited on its own:

```json
{ "kind": "preset", "entry": "preset.json",
  "requires": { "engine": null, "features": ["packages", "preset"], "packages": [{ "id": "org.example.dusk", "version": "^1.2" }], "gpu": null } }
```

```json
{ "base": { "id": "org.example.dusk", "version": "^1.2" },
  "values": { "tint": "#3d5aff", "speed": 0.4 }, "files": { "photo": "assets/photo.webp" } }
```

`preset.json` (at most 256 KiB) names the base, which `requires.packages` lists with the same
version requirement, at most 256 `values` and `files` by property id (an id is in one of them only),
and the files are pictures or videos of the package. A preset package has no properties, presets
or inputs of its own; values that the installed base does not know, of another type or out of range
are left out with a warning, like the values of a preset of the base. Repositories copy the id of
the base into the release (`base`), so that clients can list the presets of a package. The library
keeps the base of each preset (`package.base` of an item), and `GetItem` lists the installed presets
of a package (`variants`) and the installed base of a preset (`base`), which the app shows as
variants of the base. Applying a preset applies the newest installed version of its base that meets
the requirement on that output, with the values of the preset kept as the settings of the base
there, like a preset of the base, and the files of the preset taken out of it into the cache as the
chosen files of their `file` settings; while no such base is installed, applying it fails with
`NotFound` naming the base, and the app offers to install it from the repository of the preset.
In a playlist, the turn of a preset shows the same base with the values and files of the preset over
the settings of the base on that output, for that turn only: they are not written as those
settings, and a setting changed during the turn lasts until the turn ends. While the base is
missing, the error of the output names it.
On the phone, applying works the same way: a preset in its library previews its base, *Set as
wallpaper* writes the values of the preset as the settings of the base (its files go to the app's
own storage) and shows the base, and without the base it says which package to install. Installing a
package from a repository on the phone also installs, from that repository, the newest release of
each package of `requires.packages` that meets its range and is missing, and theirs in turn, four
levels deep; an installed package of any kind, presets and asset packages included, counts as
present.

## Shared resources

An `assets` package offers parts of scenes for other authors: its entry is a scene document
(`scene.md`), usually `assets.json`, whose layers, particle systems, scripts, effects (`effects`, see
"Package effects" in `scene.md`) and assets are the parts it offers, with their own files. It is never
shown as a wallpaper: the library lists it apart (`kind:assets`; searches without a `kind:` leave it
out) and applying it fails. It needs no other package (`requires.packages` is refused), since an
author copies what they use into their own package and credits it with `credits[].source`, so that
the package works without the shared one and the license of every file stays written.

`ext.parts` of the document names what it offers, at most 256 `{kind, id, title?, thumbnail?}`:
`kind` is `layer` (a layer at the top of `layers`), `particles` (a particle system), `script` (the
`path` of a script) or `effect` (a package effect), each once and present in the document; `title` has
1 to 128 characters on one line without `<` or `>`; `thumbnail` is a png, jpg or webp of the package of
at most 1024 pixels a side and 1 MiB. Without `ext.parts`, every top-level layer, particle system,
script and effect is a part. Fragment of a scene (the other fields are written as in `scene.md`):

```json
{ "schema": "reiwand.scene/2", "canvas": { "size": [1920, 1080] },
  "assets": { "flake": { "path": "assets/flake.png" } },
  "layers": [{ "blend_space": "canvas", "id": "snow", "type": "particles", "system": "snow" }],
  "particles": { "snow": { "capacity": 2000, "emitters": [{ "shape": "edge", "rate": 120 }] } },
  "ext": { "parts": [{ "kind": "layer", "id": "snow", "title": "Snow", "thumbnail": "parts/0.jpg" }] } }
```

Importing a part (`editor.md`, "Shared parts") copies it with everything it names (its child layers,
the assets, particle systems, effects, templates, pools and timelines it uses) and their files under
`assets/<id>-<version>/`, renames ids that the scene already uses, adds one credit per license and
attribution with `source: {id, version}`, and adds the package to `origin.derived_from` of an original
or derived scene (an original one becomes derived).

## Languages

Texts are plain text, a map from language to text, or `"@key"`, a key of `strings`. `strings`
holds at most 64 languages, each with at most 2048 keys of `[A-Za-z0-9_.-]{1,64}` and 512 KiB in
all; languages are BCP 47 tags such as `en`, `es` or `pt-BR`. With maps or `strings`,
`default_locale` is required and every localized text has a value in it. Labels, help texts,
combo options and preset labels of properties take the same forms. A text is picked by comparing
tags without case: the user's languages in order (each as given, then its base language, `pt-BR`
then `pt`), then `default_locale`, then `en`, then the first one; `zh-chs`, `zh-cn` and `zh-sg`
read as `zh-Hans`, and `zh-cht`, `zh-tw`, `zh-hk` and `zh-mo` as `zh-Hant`. Control characters
(except line breaks in long texts) and byte order marks are refused.

The strings of a language can also live in `locale/<language>.json`, a map from key to text of at
most 512 KiB, which the package lists with the feature `locale_files`; a language is either in
`strings` or in its file, never both, and the limits above count both. Readers merge the files with
`strings` and keep them apart when they write the manifest back. Packages and source folders read these files
as texts only when they list `locale_files`; without it a `locale/*.json` file is an ordinary file
of the package, so folders that keep data of their own there are not read as texts. The author chooses the feature: `pack build` never
adds it, and `pack build`, `pack check` and the editor note each `locale/<language>.json` that stays
an ordinary file.

A few common labels are texts of the engine, which every reader knows in its languages: `"@reiwand.<key>"`
with `accent_color`, `background_color`, `brightness`, `color`, `density`, `opacity`, `quality`,
`size`, `speed` or `volume` (`engine_strings`). They need no `strings`, and keys of `strings` or of
the language files may not start with `reiwand.`.

## Markup

`description` and `changes` use a small, safe markup: paragraphs separated by an empty line,
lists of lines starting with `- ` or `* `, `**bold**`, `*italic*` and links `[text](https://…)`.
A `*` or `**` opens before a word and closes after one on the same line; one without a partner is
refused, and `\*` writes an asterisk. HTML, images and links other than `https://` are refused.
Clients render it without loading anything remote and open a link only after a click and a
confirmation (through the OpenURI portal in the Flatpak); `reiwand-ctl` prints it as plain text. Every
other text is plain: clients show it as written,
and plain texts (titles, labels, help, options, preset labels) have no `<`
or `>`.
