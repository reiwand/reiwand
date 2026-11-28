# Mature content, flashing and search

The app, the tray and the phone treat mature and photosensitive packages with care. This page
explains how thumbnails are blurred, how flashing packages are confirmed, and how search filters
packages by rating and facets.

## Mature packages

The app and the tray blur the thumbnails of mature packages until you decide to show them. You can
show one:

- in the details of the app;
- with a click or the keyboard on the blurred thumbnail of a screen, on the app's *Settings* page
  or in the tray;
- in *Explore*, where a click on the thumbnail is enough.

The decision is stored for that package.

| Setting | Key | Effect |
| --- | --- | --- |
| *Hide mature content* | `library.hide_mature` | Removes mature packages from searches and from repositories. |
| *Show mature content without blur* | `library.show_mature` | Off by default. Stops blurring them in the app, the tray and the phone. |

## Photosensitive packages

Before applying a photosensitive package, the app and `reiwand-ctl` warn and ask for confirmation
once per package. The app also allows it:

- from its details, with *Allow this wallpaper…*;
- for several selected items at once, with *Allow these wallpapers…*.

Until you allow it:

- the daemon does not apply it;
- a profile does not set it (the screen shows the reason, and the profile sets it as soon as you
  allow it);
- lists skip it.

The app's *Settings* page and the tray warn on the screen that shows it.

## Search by rating and facets

Search filters by rating and by facets computed from the package itself:

| Facet | Meaning |
| --- | --- |
| `facet:audio`, `facet:pointer`, `facet:clicks` | The inputs the scene uses. |
| `facet:4k` | The size. |
| `facet:photosensitive` | It flashes or shows fast patterns. |
| `facet:android` | It works entirely on the phone. |

There are more, and the list above is not complete.

Web pages take their facets from the inputs they declare. In all other packages, `media` included,
the facets come from what they use.

## Commands

```sh
reiwand-ctl lib show 1R1AB2                     # rating, origin, credits, languages and description
reiwand-ctl lib search 'content:general facet:audio'
reiwand-ctl config playback.allow_flashing true       # allow photosensitive packages
reiwand-ctl lib confirm mature 1R1AB2           # stop blurring its thumbnail
reiwand-ctl pkg reveal org.example.night        # the same for a package from a repository
reiwand-ctl pack check wallpaper/               # what it needs, uses and declares, and whether desktop and phone play it
```
