# Keys and the library

How to move around the app with the keyboard, and the Library screen, where you see and set the
wallpapers you already have. The names are the ones you see in the app.

## Keys

The line at the bottom right shows only the keys that work where you are.

| Key | Shown as | What it does |
|-----|----------|--------------|
| **← → ↑ ↓** | "Move around" | Moves inside a block (the menu, a row, a list). |
| **Tab** | "Next part" | Moves to the next block (menu, "now", actions, page). |
| **Enter** | "Open" | Does the main thing of the block. In the menu, it opens the chosen word. |
| **Esc** | "Back" or "Menu" | Goes up one level. If there is no level to go up, it moves focus to the menu. |

> **Note:** In the editors, Esc does not leave: the back word at the top does.

Focus is shown with a light ring.

A button that changes function in place ignores the mouse for half a second, so that a double
click does not do what you did not ask for.

## Library

What it is for: seeing and setting the wallpapers you already have.

### The shelf and the card

The **Shelf** is a row of covers at the bottom. Under the lit cover is its card with the name, the
type, the size and the actions.

| Action | What it does |
|--------|--------------|
| **Set as wallpaper** (Enter) | Sets it right away. With several screens it says "Set as wallpaper on …". |
| **Options ›** | Opens the wallpaper's panel (see [Options of a wallpaper](#options-of-a-wallpaper)). |
| **Edit a copy…** | On a scene saved as a package. The copy is a new project and the original does not change. |
| **Right click**, or the menu key, on a cover | Opens a menu with the same actions. |

**Marks on the cover**:

- "In use", or "On …" with the name of the screen.
- A note if the file is missing or damaged.
- A note if it has flashes and they are turned off.

### Importing packages

**Import wallpaper…** lets you choose one or several `.wpak` files and puts them in your library.
There is no need to ask where they end up.

`.wpak` files are also imported:

- by dragging them onto the window;
- by double-clicking one, or with "Open with Reiwand" (also several at once, with the app open or
  not);
- with `reiwand-ctl pkg import`.

The app then says what was added, as Import does.

A picture or a video opened with Reiwand asks, as "Add wallpapers…" does, whether to copy it to the
library or leave it where it is.

The package is checked for damage or tampering:

- A tampered one is rejected with a notice.
- If it has no known signature it is imported anyway and its card says "Unverified signature".
- Importing a newer version updates the wallpaper you already have.
- With an older one the app warns and offers "Use the older one".

The **Local** filter of the shelf shows only these wallpapers. They are removed from their cover
with *Remove from the library*.

### Filters and search

The **Filters** of the bar are All, Scenes, Images, Videos, Web, From sources and Favorites, each
with its count. "More" opens Packages, Assets and the tags.

The search box says "Search Scenes" (or the chosen filter) and accepts `tag:`, `kind:`, `fav:`,
`provider:` and `sort:`.

### Adding wallpapers

**Add wallpapers…** opens the desktop file picker and asks whether to copy them to the library or
leave them where they are.

- "Write the paths…" lets you paste one path per line.
- Dropping files on the window adds them without copying them.

### See all

**See all** shows the collection in a grid. The wallpapers set now come first. "Show shelf" goes
back.

### Several wallpapers

| To | Do |
|----|----|
| Choose one | Ctrl+click, the circle on the cover, or Space |
| Choose a range | Shift with an arrow |
| Choose all | Ctrl+A |
| Leave | "Stop choosing" or Esc |

The selection line offers "Rotate these every…", "Playlist", "Tags" and "More" (favorite, rating,
flashes, remove).

### Keys in the library

| Key | What it does |
|-----|--------------|
| ← → | Move |
| ↑ | Goes up to the filters |
| ↓ | Opens the grid |
| Enter | Sets the wallpaper |
| Tab | Moves to the buttons of the card |

> **Note:** If the wallpaper service does not respond, the page says so and offers "Restart the
> service".

## Options of a wallpaper

The path is "Library › name" (Esc goes back). At the top are "Favorite" and "Set as wallpaper".

The panel holds:

- the preview;
- "Author options" (what the scene lets you adjust);
- "My presets";
- "Yours" (tags and lists);
- "More".

"Play anyway" appears if a heavy scene was left still on its poster.

Under "More" are the permissions of that scene, the sound, the name, the rating and "Remove from
the library…", which asks first.
