# Assets, presets and dependencies

Some packages are not wallpapers on their own. An assets package offers parts for other authors,
and a preset carries values for a base wallpaper. This page also explains how dependencies are
installed.

## Assets packages

An assets package (`assets`) offers layers, particle systems, scripts and loose effects for other
authors to copy into their scenes.

In the scene editor, the *Assets* panel has two actions:

| Action | What it does |
| --- | --- |
| *Make an assets package* | You mark the parts, give them a title and thumbnail, and choose the license, credit and image of the package. |
| *Import from an assets package* | Shows the thumbnail of each part and the license and credit of each file before copying anything. |

What is imported ends up in `assets/<id>-<version>/`, with its credits and origin written
automatically. The scene works without the package.

The library lists assets packages separately (the *Assets* filter or `kind:assets`). They are not
applied as a wallpaper.

```sh
reiwand-ctl pack new-assets my-scene/ parts/ --part layer:snow --part particles:snow \
  --id org.example.parts --title Parts --license CC-BY-4.0 --attribution "Your name"
reiwand-ctl pack build parts/ && reiwand-ctl pack parts org.example.parts-1.0.0.wpak
reiwand-ctl scene act other-scene/ import_assets '{"package":"org.example.parts-1.0.0.wpak","parts":["layer:snow"]}'
```

## Presets

A preset published separately appears in the library as a variant of its base wallpaper:

- the card of the base lists its presets;
- the card of the preset names its base;
- *Apply* (or `reiwand-ctl lib apply`) shows the base with those values.

Values that the installed base does not know, of another type or out of range are discarded with a
notice in the log.

## Dependencies

Installing a package from a repository also installs, from the same repository, the packages it
names in `requires.packages` that are missing.

If the base is still missing, the card offers to install it.
