# Packages

Besides photos and videos, the phone uses `.wpak` packages: the same image, video, scene and web
page wallpapers as the desktop. This page covers getting them, the package types, presets,
dependencies and mature content.

## Example wallpapers and opening a package

The *Scenes* and *Web pages* filters of the library have two buttons:

| Button | What it does |
| --- | --- |
| *Example wallpapers* | Adds back the local repository "Example wallpapers" that the app adds on first start, and opens *Explore*, where they are installed like those of any repository. |
| *Open a package* | Imports a `.wpak` with Android's document picker. |

The examples are the six scenes and the web page *Lights* that the APK carries, the same examples
as the desktop.

On the phone, *Open a package* stores the `.wpak` in the device's single library, without asking
where; there is no local repository to choose.

## Package types

The four package types that are a wallpaper are supported, as on the desktop:

| Type | How the phone shows it |
| --- | --- |
| Image | Like one more photo, with its fit, its framing, its edit and its parallax. See [Images](images.md). |
| Video | Like one more video. See [Videos](videos.md). |
| Scene | With the scene engine. See [Scenes](scenes.md). |
| Web page | With the system WebView. See ["Web wallpapers"](../web-wallpapers.md). |

Resource packages are rejected with a notice, as on the desktop.

The image or video of the package is extracted when importing it, in parts and without loading it
whole into memory, to the app's files. It is deleted with the package.

> **Warning:** A package of more than 1 GiB uncompressed is rejected. A package made only for the
> desktop is rejected with a translated notice.

## Presets

A preset published separately is shown with its base package and its values. The preview uses the
values without saving them. When the preset is used as a wallpaper, it writes its values into the
base's settings.

- The preset's page names the base with *Show* or, if it is missing, *Install it* from the
  repository that offers it.
- The page of the base lists its presets, as on the desktop.

## Dependencies

When installing a package from a repository, the packages it names in `requires.packages` that are
missing are also installed from the same repository (the newest version that meets the range). The
packages that these ask for are installed too, up to four levels. An already installed preset or
resource package counts as present.

If a dependency could not be installed:

- the phone says so with a notice that names what was missing (also when updating or when
  installing the base of a preset);
- the desktop likewise: the installation job ends with that text and the result carries `missing`.

## What the phone leaves out

The page of a package warns of everything the phone leaves out:

- the color around the image and the spanning over several screens;
- the new sounds;
- what makes no sense on the phone (the sounds of scripts);
- RGB lighting.

If a package asks for compute shaders and the phone's graphics do not have them, its page says so.

The page warns of each part that does not work on the phone, from its sounds to its video settings
and the new features it leaves out.

## Language and system colors

The library and the page of a package show their title and description in the phone's language.

The accent color of the wallpaper (or, if it does not propose one, the dominant of its preview) is
the color that Android gives the system.

## Mature content and flashes

- The thumbnails of mature packages come out blurred, unless *Show mature content without blur* is
  on in *Settings*.
- *Hide mature content* (`library.hide_mature`, off by default), right above, removes them from the
  library and its searches, as in the desktop app and in `reiwand-ctl`, unless the search asks for
  `content:` itself.
- Mature content is remembered per package.

> **Warning:** A photosensitive package carries the notice "flashes and is disabled". It is neither
> previewed nor used as a wallpaper while the general flashes setting stays off (it comes off).
> Lists skip it.
