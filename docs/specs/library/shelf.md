# Shelf

- A row of covers at the bottom, over the image of the wallpaper in use; the focused cover is lit
  and under it appears its card: name, footer (type and size) and the actions.
- Covers use the thumbnail that covers the lit cover at the screen's scale (512 px at scale 1, 1024
  px at scale 2): each point of the screen has a point of its own of the image, unless the
  wallpaper or the cover of its package are smaller. Stored thumbnails of another size are not
  deleted: the service makes the new ones in the background and the old ones leave through the
  cache limit.
- A packaged scene whose cover is smaller than the requested thumbnail, of any size, is drawn: the
  service paints a still frame of the scene at its poster moment, at the requested size and never
  larger than the scene, without pointer, sound or sensors and without network. It does it in the
  background, one scene at a time, while the cover is seen; it does not draw with playback paused,
  the session locked, the battery pause or with quality fixed at Q0 or Q1, and then it uses the
  cover. If the scene cannot be drawn, the service log says so with the id of the item and the
  reason. A cover much smaller than the card is shown at its size, centered over the still
  background of the card, without stretching. Web pages and packaged presets always use their
  cover.
- With the pointer still for about 300 ms on the cover of a scene, a web page or a video (on the
  shelf, on the lit cover and in "See all"), the cover shows the wallpaper running, drawn by the
  service in a `preview:` stream of the cover's size (`RequestStream`, `Library1.Apply` on that
  output and `ReleaseStream`, the same as `reiwand-ctl stream`, `reiwand-ctl lib apply --output` and
  `reiwand-ctl release`). It lasts while the pointer is over it; on leaving, on scrolling the row,
  on opening another page or when the window loses focus the stream is released and the cover
  returns. Sweeping the shelf with the pointer starts nothing and there is never more than one at a
  time. It does not change the screens' wallpapers nor is it noted in the settings or the history,
  it uses the quality the user set, and it does not start for a scene with flashes that the user
  did not allow, one hidden for mature content, a damaged one or one that asks for a newer version:
  those stay on their cover, without a notice. With the keyboard the lit cover keeps showing its
  cover. Images do not change. The running cover always goes without sound: the service opens no
  audio output for the stream of a cover (`preview:card-…`) and the web page of a cover sounds at
  volume 0, even if the scene has sound layers or videos with audio or the page plays sound, and the
  sound of the screens' wallpaper stays the same.
- "Set as wallpaper" (Enter) sets it right away, without a dialog. With two or more screens it says
  "Set as wallpaper on …" and sets it on the chosen screen (the first, while there is no other
  choice); a wallpaper that spans all the screens is set spanned. A notice says where it ended up.
- "Options ›" opens the wallpaper's panel.
- "Edit a copy…" (only on a scene stored as a package) opens the step that copies it as a new
  project, also if the scene is yours: the original never changes. To continue with a project of
  your own, it is opened from "Create".
- Right click or the menu key on a cover (shelf or grid) open a menu with the same: "Set as
  wallpaper" (or "Show in the folder"), "Options" and "Edit a copy…". In Explore the menu offers
  the main action of the card, "Details" and "Edit a copy…" (installed scene); the latter is also a
  button of the lit card.
- The cover of the wallpaper in use carries only a small icon, without text or a frame around it;
  its name for screen readers and the hint when hovering over that icon (only there: the rest of
  the covers have no hint) say "In use" or, with two or more screens, "On …" with the name of each
  screen where it is. The focus ring is separate: it marks the cover with keyboard focus and only
  shows after using the keyboard (arrows, Tab or another key); a click or a touch hides it. The
  cover with focus stays chosen, so the keys of the bar respond without a prior click.
- A note on the cover warns of a missing or damaged file, of a wallpaper withdrawn by its
  repository or of one with flashes (off until the user allows it).
- While the desktop creates the depth layers of an image on screen, its cover says so in all
  listings: "Waiting to generate layers" in the queue and "Generating layers · N %" with a thin bar
  on the bottom edge while it advances (with LaMa it advances with each crop it fills). If they
  could not be generated, the note comes out in an error tone and the menu of the cover (also with
  the menu key) offers "Try again" (`RetryLayers`, `reiwand-ctl retry-layers PATH`); the same if the
  user cancelled the creation. When it ends the note disappears. The card of the screen in "Now"
  shows the same state and the same bar, and the task line no longer lists these creations. A
  cancelled creation does not start again by itself: only when applying the image again, when
  changing a depth or parallax setting that changes the layer set, or with "Try again"; the
  cancelled state lives only in memory and is forgotten when the service restarts.
- The bar at the top filters by All, Scenes, Images, Videos, Web, From sources and Favorites; each
  word carries its number of wallpapers and only the classes that have any are seen (All and
  Favorites always, even at 0). "From sources" are the photos that a photo source brought;
  "Images" are the other images, so a source with hundreds of photos does not cover what is your
  own. "More" opens Packages, Assets, the tags and "Rename tag…". The search box searches inside
  the chosen filter: it says "Search Scenes" and, when typing, "3 of 34" next to the field. The
  chosen filter is kept when going to another page and coming back. The answers of a search never
  take focus away from the search box. The search box accepts `tag:`, `kind:`, `fav:`, `provider:`
  and `sort:` (also `reiwand-ctl lib search`).
- Marking a favorite (on the card, in its options or on the wallpaper in use) shows "Added to your
  favorites." with "See favorites", which opens the library with that filter. The filter shows
  everything marked, also photos from sources; empty it says how to mark one.
- The mouse wheel over the library strip (notice line, bar, cards and buttons of the lit card) moves
  the lit card one wallpaper per notch, like the arrows, and stops at the ends; a touchpad adds its
  travel up to a notch. It applies on all shelves; the grid of "See all" keeps scrolling. While it
  turns, the live cover does not start.
- Each wallpaper carries its translated title; lacking a title, a photo from a source carries
  "Photo from <source>", the rest carry the file name without extension and, lacking everything,
  "Untitled wallpaper". An identifier is never shown, not even on the library covers.
- In Explore, the card of an installed wallpaper uses the same cover, the same size and the same
  live cover as the library; that of an uninstalled one shows the repository's preview.
- "Add wallpapers…" opens the desktop file picker and then the "Add N wallpapers" panel, which asks
  whether to copy them to the library or leave them where they are. "Write the paths…" swaps the
  list for a field with one path per line. When it ends, the notice names what happened: with just
  one, ""Name" was added." and "See" opens its card; with several, "N wallpapers were added." and
  "See" goes back to the shelf at All, without filters, where the new ones come first (one that
  was already there and was updated keeps its place). Those already there are counted apart ("N
  were already in your library."), not as failures. If any failed or is an older version of one you
  already have, the notice carries "Details", which opens the list of those files with the reason
  for each (the arrows go through it), "Use the older ones" to replace the newer versions and "See
  the N added". Dropping files on the library adds them without copying them, with the same
  notice.
- The veil at the bottom rises behind the bar, so the words read also over a light background; its
  opacity follows the tones that the app draws from the image.
- Keys: ← → to move, ↑ goes to the filters of the bar (← → between words, Enter chooses, ↓ goes
  back to the covers), ↓ or "See all" shows the grid, Shift with an arrow (or Shift+click) chooses
  the wallpapers between the previous and the new one, Ctrl+A chooses all, Esc stops choosing.
- On reaching the library (on opening the app, from the menu, another page, "Options" or a dialog)
  the covers already have focus, so the keys at the bottom right work without a click. Tab passes
  from the lit cover to its buttons ("Set as wallpaper", "Options") and Shift+Tab goes back to the
  covers; while focus is on those buttons the bar shows only Tab and Esc. Enter sets the wallpaper
  and Space starts choosing.
