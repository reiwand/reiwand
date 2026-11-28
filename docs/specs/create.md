# Create page

It is where an author starts: a new project, a recent one, a copy of an installed wallpaper or a
folder. It uses the title skeleton; the editor opens as always, with `host.work`.

## What it shows

- "Continue with yours": recent projects, the newest first. They are stored in
  `recent-projects.json`, in the app's state directory (200 at most, with a "recent" mark per
  project and the list of those forgotten), and the folders found in the projects folder are added.
  It is local state of the app, not of the service: it has no command in `reiwand-ctl`. A folder
  that no longer exists stays in the row, marked "Its folder is gone", with "Find it…" and "Remove
  from the list". The row shows only those that fit across and ends with "All my projects N".
- "All my projects": a grid (the same as "See all" of the library) with all the projects the app
  knows, the newest first, and a search box that filters by name. Enter, the button at the bottom
  or a double click opens the project; Esc or "Back" returns to the row.
- Right click or the menu key on a project (in the row or in the grid) opens a menu with "Keep
  editing" ("Find it…" if the folder is missing), "Remove from recents" (leaves the row, stays in
  "All my projects") and "Remove from the list" (in the grid, or in the row if the folder is
  missing: the app forgets it even if it is in the projects folder; the notice offers "Undo"). None
  deletes files. "Clear", next to the title of the row, empties it with "Undo". A project returns
  to the row when it is edited again.
- "Start a new one": a scene (empty, particles, layered parallax, text clock), a single-file
  wallpaper (image, video, web page) and "To reuse" (preset and assets package), all three in view,
  with the same footer.
- "Learn from an example": the example scenes that come inside the app (those of `scenes/` in the
  repository, about 350 KB), in every installation, without network or repository. Choosing one
  opens the "Edit a copy…" step with the license and the name of the copy; the app first builds a
  temporary package in its cache. Installed packages are copied from their own card (library or
  Explore).
- "Open a project…": chooses the folder of a project that is not in the list (the one with its
  `manifest.json` and, if it is a scene, its `scene.json`); its help says so. If the folder is not a
  project, the notice says what was expected in it and offers "Choose another".

Without projects, the page opens on "Learn from an example". The footer follows the marked card:
title, a line, "Change…" (title and folder) and the main button. "Change…" leaves in a single row
the name of the starting point, "Title", "Folder" and "Create", with focus on "Title"; the folder
is shown with `~` for the home folder and can also be typed that way. Enter in either of the two
fields creates; Esc goes back to the cards with the same card marked. In a window that is too
narrow the fields go below the title and "Create" stays on the title line.

## How the editor opens

`App.compose(folder, starting point, title)` creates the folder if needed and opens the scene
editor or the packager with `host.work`. For image and video the file is chosen first: its path
travels in `App.media` and the packager takes it (`App.takeMedia()`, which leaves it empty) and
applies it with `manifestEdit("media", {file})` after creating. When any project loads it is noted
in the list of recents.

## Limits

- The cards of "Start a new one" are still images or icons, without an animated preview.
