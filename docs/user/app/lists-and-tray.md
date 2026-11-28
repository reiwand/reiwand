# Lists, the preview and the tray

Groups of wallpapers that take turns, the preview window for changing the wallpaper, and the
Plasma tray panel.

## Lists

What it is for: groups of wallpapers that take turns.

The screen has the "All" and "Running" filter, "New playlist…" and a shelf of covers with four
thumbnails.

The card says how many wallpapers it has, how often it changes and whether it is stopped, with
"Start" and "Edit ›".

### Edit

| Field | What it holds |
|-------|---------------|
| Name | The name of the list. |
| "Changes every" | 10 min, 30 min, 1 h, Every day or "Other…". |
| Screen | The screen it runs on. |
| The wallpapers | Each with Remove and the handle for ordering. |
| "Add wallpapers…" and "Add a folder…" | Add more. |

At the top are "Changes in", "Pause the list" and "Stop".

Ctrl+↑ and Ctrl+↓ move the chosen wallpaper.

### More

- Search (a list with a search fills itself).
- "Make manual".
- "Follow a folder…".
- The rules.
- The History.
- "Remove the list" (with "Undo" in the notice).

## Changing the wallpaper with a preview

Open it from the tray, a shortcut or `reiwand-ctl next --preview`. A small window opens centered on
your screen, with the rest dimmed.

The chosen wallpaper is shown large in the center and its neighbors, sharp, at the sides. Below are
its name and its position.

| To | Do |
|----|----|
| Move the selection | ← and →, the wheel, or a click on a neighbor |
| Apply and close | Enter, or a click on the central one |
| Close without changing anything | Esc, or a click outside |

More in [changing the wallpaper with the keyboard](../keyboard-switching.md).

## The tray

On Plasma, the tray icon opens a panel with:

- what is playing now ("“…”, 2 of 5");
- Pause or Resume;
- Next;
- "Master volume";
- "Mute all";
- per screen, "Choose…" and "Change with preview…".

"More" brings favorite, adjust, filter and remove. "Open Reiwand" and "Settings…" open the app.

If the service is not running, the panel says so.

After Reiwand is updated, Plasma keeps running the tray it loaded when it started. When the
service runs a newer version than the tray, the icon asks for attention and the panel says
"Reiwand was updated", with "Restart Plasma" (which starts Plasma again with the new tray) and
"Later" (which hides the notice until Plasma starts again). Logging out and in also loads the new
tray.

With the Flatpak, a `flatpak update` leaves the service running the old version too, so the tray
also looks at the version installed (it runs `flatpak list` on the computer when it starts, when it
opens and every 15 minutes). When a newer one is installed, the icon asks for attention and the
panel says "A new version of Reiwand is installed", with "Restart Reiwand": it copies the new
wallpaper plugin and tray into Plasma (`reiwand-plasma | sh`), stops the old service (only the
service: an open app window and its edits stay) and restarts Plasma, which starts the new service.
"Later" hides it as above. Without the Flatpak nothing of this runs.
