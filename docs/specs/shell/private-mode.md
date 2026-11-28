# Private mode, Name of the screens, Now block, Lists and Third-party licenses

On this page: private mode, name of the screens, now block, lists, third-party licenses.

## Private mode

- A switch in Settings › Device, the first row: "Private mode", with the help "Cuts the whole network:
  sources and repositories are not updated, covers are not downloaded, web wallpapers only load files
  from this computer and your server and your phone are not contacted." Off by default; it is stored
  in `network.private` (of each computer, never synchronized) and the service reads it on startup,
  before any network task.
- When on, the program sends nothing to any network and looks up no name: every network client of
  the service, of `reiwand-ctl` and of the app is born at a single gate (`reiwand_core::net`) that
  refuses before resolving a name or connecting, with the phrase "private mode is on: the network is
  cut…". It cuts sources, repositories and their downloads, covers from the internet, the sign-in of
  accounts, OpenRGB (even if it is on this computer, because it speaks over TCP), model downloads and
  publishing. Web wallpapers are reloaded without the network they had been allowed and their process
  runs without network. What is already on the computer stays the same: library, bundled examples,
  local-file repositories, scenes, videos and local web wallpapers.
- Explore, Sources, Account and Repositories show at the top a single calm line "Private mode: the
  network is cut." with the "Turn off" button beside it, and their network buttons stay disabled.
  There is no dialog or error in coral: it is a state the person chose. The line and the button are
  reached with Tab and respond to Enter.
- Terminal: `reiwand-ctl private on|off|status`; `SetPrivateMode` and the `PrivateMode` property in
  `dbus.md`. A network command of `reiwand-ctl` with the mode on fails with the same phrase and a
  non-zero code.
- Limit: an exchange that was already in progress when it was turned on finishes, and a connection
  left open with a server may carry one more request for at most 15 seconds. Opening a link in the
  browser, which asks for confirmation, keeps working: another program does it.

## Name of the screens

A single function of the window (`App.screen`) and its twin in the tray (`Daemon.screen`) give the
name the person sees: the own panel of a laptop (eDP, LVDS, DSI connectors) is "Laptop"; any other is
"Monitor", numbered in a stable order by connector name when there are several ("Monitor 1",
"Monitor 2"). They are used by the first-start notice and its "Apply to…" menu, the mark of the cover
set, the main button of the card, the strip of screens, Settings › Screens (which leaves the
connector as secondary text), the tray and the editor's question "on which screen". The terminal keeps
using connector names.

## Now block

At the top right of the title screens is what is on as wallpaper on the chosen screen:

- With one screen it says "On your desktop"; with several it shows a strip with the thumbnail of each
  one and its name, and choosing one changes the screen of the page (`target`) or of the block.
- Title of the wallpaper, its type and who provides it; "Set a moment ago" for a minute after changing
  it with the window visible.
- Controls: pause or resume, master volume with mute (only if the wallpaper has sound), favorite,
  fit and filter of that screen.
- If the screen is rotating a list: "List "Evening" · changes in 7 min" with Previous, Next and
  "Pause the list". The clock updates once a minute and only with the window visible. "Pause the
  list" stops the rotation and offers Resume in the notice; with the `resume` rule it returns to the
  same wallpaper.
- Setting a wallpaper by hand on a screen that rotates warns "The rotation of "Evening" stopped
  because you set "Northern lights"." with Resume.
- Jobs in progress ("Making the thumbnails: 9 of 12") with Cancel, and failed ones with their reason.
- If the scene of that screen is too heavy and stays on its poster (`quality_auto` = `still`), an
  Amber line "Still at its poster" under the type line. The reason for a pause is not shown: it goes
  only to the terminal.
- Without any wallpaper set: "On your desktop", "You have no wallpapers yet", "Choose where to start:
  the first thing that enters your library becomes your wallpaper." and the note of the example
  image, without controls. If the library already has wallpapers, the title says "You have not set
  any wallpaper yet" and the sentence "Choose one in your library to put it on your desktop.".
- In the Library and in Lists, which use the low band, the block goes whole. It is only shortened
  (without strip, without type line and without jobs) on a page that puts its own block above the
  band. In a short window the band moves to the column and the block stays on top, whole.
- Jobs in progress and failed ones go in lines under the block. A task that is retried (the sync of
  a source without network, for example) shows a single line: that of its last attempt; the earlier
  failure disappears as soon as the task runs again.

## Lists

The lists page uses the low band: filter All and Running (each word with its number), "New
playlist…" and a shelf of covers made with four thumbnails of the list. The card says how many
wallpapers it has, how often it changes and whether it is stopped, with "Start" (or "… on" the
chosen screen), Enter and "Edit ›". Without lists it shows "You have no playlists yet" with the path
from the library.

Edit opens the options skeleton with the path "Lists › Evening": Name, "Changes every" (10 min, 30
min, 1 h, Every day or "Other…", which admits seconds, minutes or hours), Screen (with several), the
wallpapers with Remove, "Add wallpapers…" and "Add a folder…", and More (history, search, rules and
remove the list). At the top are "Changes in", "Pause the list" and "Stop". Removing a list offers
Undo in the notice, which creates it again with its wallpapers and its rules.

"Wallpapers" unfolds at the same level: each wallpaper brings its thumbnail, "on screen now" if it
is being shown and Remove. In a manual list the order is changed by dragging the handle of the row
(it is dropped where it is wanted; when brought near the top or bottom edge the view scrolls by
itself, so it reaches far in a long list) or with Ctrl+Up and Ctrl+Down on the chosen wallpaper; the
chosen row keeps focus while moving. If the list changed meanwhile, the notice says so and loads it
again. A list with a search shows its wallpapers without handles or Remove. "Add a folder…" copies
the wallpapers of the folder once; following it so that new files come in by themselves is in More,
"Follow a folder…". Up to 500 wallpapers are shown.

More is built with rows: Search (empty is a manual list; with text it says how many wallpapers
match), "Make manual" in a list with a search, "Follow a folder…", one row per rule of the list
(switch, choice or field depending on the rule), History (a fold with what it showed recently, the
most recent first, and the screen when there are several) and "Remove the list". At the bottom the
keys Move around, Change and Back are seen.

## Third-party licenses

- `packaging/THIRD-PARTY-NOTICES` lists the Rust libraries that the daemon, the terminal, the
  application, the web helper and the server carry, with the text of each license and the copyright
  notices, plus the fonts, icons and models. `android/app/src/main/assets/third-party-notices.txt`
  does the same for the phone's library and the Android libraries.
- Both files come from `scripts/notices.py write`, which reads `Cargo.lock`, the local cargo registry
  (`--registry DIR` adds another) and `android/gradle/libs.versions.toml`. `scripts/notices.py check`
  compares the list of packages with `Cargo.lock` and fails if the file is stale; a `reiwand-ctl` test
  and the checks step of the *Tests* workflow (started by hand) run it.
- A package whose text is not in the local registry stays in the "Without a licence text" section of
  the file; today they are `basisu_wasm_libcxx` and `interpolate_name`.
- They are carried by the AppImage (root), the RPM (`%license`), the Flatpak
  (`/app/share/licenses/…`) and the server image (`/usr/share/licenses/reiwand-server/`).
- It is seen in the application in Settings, Advanced, "Licenses" (scrolling text; arrows, Page Up,
  Page Down, Home and End; Esc goes back), in `reiwand-ctl licenses`, in the "Open source licenses"
  page of the server (`/legal/licenses`) and on the phone, in "Licenses".
- The cards of online sources (and the phone's list of sources) say that the photos belong to their
  authors and are fine as a wallpaper, not for publishing; the notice for downloading the model and
  the engine on the phone says that it comes from a third party and that the store has not reviewed
  it.
- The texts of the fonts (OFL) and of the icons live next to them: `apps/reiwand-app/look/fonts/`,
  `apps/reiwand-app/look/icons/`, `apps/reiwand-server/assets/` and, for the phone, in `assets/`.
- `hum.wav` and `example.jpg` are generated by `scripts/example-assets.py` (a synthesized tone and an
  image drawn with code), so they belong to the project and carry the license of the examples.
