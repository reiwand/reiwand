# The app and the library

The **reiwand** app is where you add photos and videos and set them as the wallpaper. This page
covers the app's tabs, adding files, the page of each item and selecting several at once.

## The three tabs

The bar at the bottom has three tabs:

| Tab | What it does |
| --- | --- |
| *Library* | The photos and videos added with *Add*. See below. |
| *Lists* | Lists of wallpapers that change by themselves. See [Lists](lists.md). |
| *Settings* | Fit, fade, sound, frame rate, graphics and diagnostics. See [Settings](settings.md). |

Behind each page you see, still and veiled, the image of the wallpaper in use.

## Add photos and videos

*Add* offers three paths:

| Option | What it does |
| --- | --- |
| *Photos and videos* | Opens the Android photo picker. |
| *Choose files* | Opens the document picker to pick several images and videos from any folder, from the card or from an installed cloud. |
| *Import a folder* | Adds the images and videos inside a folder and its subfolders (5000 at most), showing how many it has done out of how many. |

Everything is copied to the app's private storage, so no permission is needed.

- The same file is not copied twice.
- Files that cannot be opened are counted in a single notice.

> **Note:** The app does not use the network, except to download the depth model when you ask for
> it (see [Parallax and depth](parallax-depth.md)) and for the repositories you add (with your
> accounts on their servers).

On the phone, *Open a package* stores the `.wpak` in the device's single library, without asking
where; there is no local repository to choose. See [Packages](packages.md).

## The page of an item

Tapping an item opens its page. It shows:

- a live preview in the shape of the screen;
- its dimensions and duration;
- the buttons *Set as wallpaper*, favorite, *Add to a list* and *Remove*.

For the fit, the framing and setting the wallpaper, see [Images and setting the
wallpaper](images.md).

## Select several items

1. Long-press an item in the library, or in an open list, to start selecting.
2. Tap items to mark or unmark them. The top bar shows how many are marked, with *Choose all* and
   *Cancel*.
3. Use the top bar's actions. They apply to all marked items at once:
   - add them to a list;
   - mark or unmark them as favorites;
   - choose their fit;
   - remove them from the library, with a single confirmation (in a list, take them out of it).

The back gesture leaves the selection. The selection is kept when the phone is rotated.
