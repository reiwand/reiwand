# From the scene folder to a package that others install

A scene is a folder with `scene.json`, a `manifest.json` and its files. For other people to install
it, you turn it into a **package**: a `.wpak` file, which is a ZIP with the manifest, the scene and
what it uses. These pages cover the whole way, with its commands (`cli.md` gathers them all):

1. Write the manifest and set a preview.
2. Check the package.
3. Sign it and package it.
4. Test the installation.
5. Publish it in a repository.

The same steps apply to web wallpapers (`web-wallpapers.md`), images and videos; what changes is
the type (`kind`) and the entry file (`entry`).

| `kind` | `entry` | What it carries |
|--------|---------|-----------------|
| `scene` | `scene.json` | Images, videos, sounds, fonts, scripts, models, own effects |
| `web` | an `.html` page | Its page with its files |
| `image` | an image | The image |
| `video` | a `.webm`, `.mp4` or `.mkv` video | The video |
| `preset` | `preset.json` | The values of another package |
| `assets` | a scene document | Scene parts for other authors |

A package never carries programs or libraries (`.exe`, `.dll`, `.so`, `.sh`, `.bat`, `AppImage`…):
they are rejected by extension. Nor symbolic links or file names with `..`, `\` or `:`. It has at
most 4096 files, 1.5 GiB per file and 2 GiB in total.

The package, by topic:

- [The manifest](package/manifest.md): every field of `manifest.json`.
- [Languages, licenses and ratings](package/manifest-rights.md): languages, licenses, rating, photosensitivity, origin and inputs.
- [Previews and checking](package/checking.md): previews and what the check looks at.
- [Packaging and signing](package/signing.md): making the package, signing it and trying the installation.
- [Publishing and copies](package/publishing.md): repositories, editing a copy of an installed package and videos.

## Before publishing: a checklist

1. `reiwand-ctl pack check` without errors and `reiwand-ctl pack check` at `ok`.
2. The inputs of the manifest are those the scene uses, not one more.
3. `content` and `photosensitive` tell the truth, and the license is the one you want.
4. The scene with scripts was seen in the preview (or with `scene render`) and the console has no
   errors.
5. The thumbnail looks right with the capture of `scene thumbnail --capture`, which runs the
   scripts. The phone has no isolated process for scripts: it shows the preview of the package and,
   if there is none, draws the scene without running them (a text that a script fills in comes out
   with its initial value). That is why it is best to always package a preview made with the
   capture.
6. `version` was raised, and `pack sign` and `pack build` were done after the last change.
7. `pack validate --key author.pub` prints `ok … signer=`.
8. There are no keys, passwords or personal paths inside the folder.

## Next

- `scripts.md`: the scripts of the scene.
- `web-wallpapers.md`: web pages as wallpaper.
- `cli.md`: all the creation commands.
- `properties.md`: the properties and presets of the manifest.
