# Phone: daily use

The screens a person uses every day in the phone app (`android/`): the library, the page of a
wallpaper and the lists. Each screen draws only its content over the app's frame (`Shell.kt`): the
whole poster of the wallpaper in use behind, the veil and the bar of three words (Library, Lists,
Settings). While a list is set, the poster is that of the wallpaper that is seen at that moment.

## Library

The top row carries search and "Add" as two boxes with an icon (`Now.kt`). Below goes the card of
the wallpaper in use: where it is set (only when the app's wallpaper service is chosen on a
screen), the title, its data, "Options" and "Favorite" (`Now.kt`, first element of the grid of
`Library.kt`). The grid shows each cover with its name below; that of the wallpaper in use says "In
use" and a wallpaper with flashes carries an amber line under the name instead of looking blurred
(`Cover.kt`). Opening search raises the grid and focuses the field. Search finds a package by its
title in any of the languages it carries ("Kiste" and "Turning" both find "Turning crate", whose
German title is "Drehende Kiste"), with any filter; if the person renamed it, only by the new name.
A search with no results says what was searched and offers to clear it; an empty "Favorites" offers
to see everything; without network, the connection line disappears.

A long press on a cover opens selection (`Select.kt`, `Picked.kt`). At the top: "Cancel", "N chosen"
and "Choose all". At the bottom: "Playlist", "Favorite", "More" ("Remove from the list", "Remove
favorite", "Fit", "Remove") and "Rotate these". "Rotate these" creates the list "Rotation", which
changes every hour (if the name already exists, the core appends a number and the notice shows the
real name), and notifies "Playlist "Rotation" created: changes every hour" with "Change", which
opens the interval. Inside a list the bar shows neither "Playlist" nor "Rotate these", and "More"
offers "Remove from the list". "Favorite" in the bar always marks; unmarking is in "More" → "Remove
favorite".

Pieces shared with the other screens: `Seek` (search field), `Cover` and `Thumb` (covers), `Choose`
(sheet to choose a list) and `Removing` (question before removing). Tests: `LibraryLookTest` (normal,
empty search, empty favorites, no network, selection and rotation).

## Wallpaper page

`Detail.kt`. It opens when tapping a cover in the library or a wallpaper inside a list.

- **A single scrolling column**, with a fixed footer at the bottom. In portrait, at the top goes the
  live preview (a little more than a third of the screen height) with the back arrow over it; in
  landscape, the preview on the left and the column on the right, with the title at the top.
- **Preview.** It shows the thumbnail instantly and the full image replaces it when it has finished
  decoding, off the interface thread: a large photo never leaves the frame empty or freezes the
  page.
- **Header.** The title, the "In use" mark if it is the wallpaper set and a data line: the type
  (Image, Video, Scene, Web page), the size and the duration when it has one.
- **Why it looks different.** If it is the wallpaper in use and the engine is holding it back, a
  line says so in words: "Still because it is heavy", "Quality lowered for the battery" or "Quality
  lowered for the heat", each with its explanation. It is polled every two seconds only while the
  page is in view and the wallpaper is set.
- **Flashes.** A wallpaper that flashes with the flashes setting off says "Has flashes, disabled"
  and cannot be set. Mature content is filtered by the general setting: the page does not cover it.
- **Framing** (images only): dragging and pinching the preview, with the help "Drag and pinch the
  preview to frame the picture", the fit and "Reset framing".
- **What belongs to the wallpaper:** the stars, the tags ("Add tags", which asks for names separated
  by commas) and "Rename", both in a sheet from the bottom with its field; the description; what the
  phone cannot show of that wallpaper; similar wallpapers; the depth map of a photo; the options of
  a scene and what its author lets you adjust.
- **Fixed footer:** Favorite, "Add to a list", Remove and "Set as wallpaper", which asks only once
  where: the home screen, the lock screen or both. Remove asks for confirmation.
- Each control on the page is seen whole and can be tapped on the smallest screen the app supports,
  with the three system buttons or with gestures.

## Lists

`Playlists.kt` (the tab) and `Entries.kt` (the page of a list).

### The Lists tab

- At the top the title "Lists". Each list is a row with the thumbnail of its first wallpaper, the
  name and below, on one line, the "In use" mark if it is set on some screen, the interval and how
  many wallpapers it has: "Every hour · 3 wallpapers". A list with wallpapers that are not yet on
  this phone says so under its row.
- Below the rows, "New playlist" opens a sheet from the bottom with the "Name" field and "Create".
- **Empty:** "No playlists yet", with the explanation "Choose several wallpapers in the library and
  tap "Rotate these".", the "Go to the library" button, which switches to that tab, and "New
  playlist".
- **Error:** if the lists cannot be read, the tab does not pretend to be empty: it says "Could not
  read the playlists", offers "Retry" and keeps the reason in "Details".

### The page of a list

- At the top the back arrow, the name and the pencil "Rename the playlist" (sheet with the "Name"
  field and "Save").
- **Empty list** (a manual list without wallpapers): "This playlist is empty", "Add wallpapers from
  their page in the library." and "Go to the library".
- The rules, in rows: "Change wallpaper" says its interval and opens a sheet with the seven options
  (every 10, 15 or 30 minutes, every hour, every 6 hours, every day or never) and "Other…", which
  asks for whole minutes from 1 to 43200 and stores them as `45m`; a list with another interval says
  it as is ("Every 45m"); "Shuffle"; "Skip videos on battery"; "The same wallpaper on the home screen
  and the lock screen"; the days of the week and the fixed hours ("Add an hour", each hour with its
  "Remove this hour").
- **Smart search:** the search field, how many wallpapers match, "Save the search" and "Make manual".
  A list with a search fills itself.
- "Set as wallpaper" sets the list (with the same question of where). "Delete playlist" does not
  delete at once: it asks "Delete the playlist "…"?", warns that its wallpapers stay in the library
  and the button that deletes is the only filled one, in coral.
- **Wallpapers:** one row per wallpaper with its thumbnail and its name. Tapping it opens its page;
  holding it starts choosing several. In a manual list each row carries "Remove from the list" and a
  handle to drag and change the order.
- **"More options"**, folded at the end: "Resume where it stopped" and "Fade between its
  wallpapers" ("As in Settings", "None" or a duration).

## Grouped question

When a wallpaper asks for something nobody has answered yet (sound, sound over the network, touch,
taps, media, media over the network, network), its page shows a single block under the status line:
"This wallpaper asks for", the note "Until you answer, it plays without these." and a row for each
request with "Without it" and "Allow" (`Asks.kt`). Nothing is drawn if the wallpaper asks for
nothing (photos, videos). "Allow" for sound first asks the system for the microphone; if the system
denies it, a notice is given and the answer is not stored. Each answer reloads the wallpaper with
what is allowed. Sound over the network is not asked if sound was denied, nor media over the network
if media was denied.

The same question exists as a sheet from the foot (`AsksSheet`), which closes by itself when nothing
is left to answer; the first start and the settings use it. Answers are stored per package:
`inputs.json` (what is allowed) and `declined.json` (what was answered "Without it"), by id and, as
on the desktop, by who made the package: the key that signs one installed from a repository
(`key:<fingerprint>`, so its updates keep what was answered) or the SHA-256 of the file of any other
(`sha256:<hex>`). Another package with the same id never inherits a permission; answers stored
before this change, only by id, are discarded and the phone asks again. Operations of the phone's
core (`crates/reiwand-android/src/asks.rs`): `asks` (what is left to answer), `decide` (allow and
deny several at once; a request in both lists is the error `android.decided_twice`), and `scene`
also returns `undecided`.

A wallpaper that the phone judges heavy stays still with the line "Still because it is heavy" and the
"Play anyway" button, which sets the quality to "minimum 1" for all wallpapers (undone in Settings →
Quality). The first time per package a notice also appears at the top of the page with "Not now" and
"Play anyway".

## First start

With the library empty the page shows neither header nor filters: it says "Nothing here yet" and
offers first "Explore repositories" and then "Add photos", with the strip of example wallpapers and
its "Explore" button, which opens the examples repository in Explore. The "Add" sheet groups photos,
files (packages too), folders and animated wallpapers. The "Repositories" page (formerly "Sources")
leads to "Add a repository", a page of its own with a single field, the address, and the button fixed
above the keyboard; there is no fingerprint field or code reader, and the app does not ask for camera
permission. A `reiwand://add?url=…` link opens the same page with the address typed and adds nothing
until the button is pressed. The address is valid if it starts with https://; http:// is only valid
for an address of this same phone (localhost or 127.0.0.1), and the notice says so in those words.

The answer decides what follows: a public repository is added and opened in Explore; a private one
asks for an account ("Use the account X", "Use another account" or sign in with a code that is
approved in the browser); a server with its own certificate shows its fingerprint to trust it, once,
for that address; the address of a server (not of a repository) lists the public repositories it
offers, to choose one; and a signature or certificate that changes is warned of without applying
anything. A rejection of the address appears under the field and marks it; any other (the server does
not respond, the address is not a repository) appears below without marking anything. The form
decides the place by the rejection's code, not by its text.

The first thing added to an empty library becomes the wallpaper: a single notice says "X is now your
wallpaper", with the name in the app's language, and asks where (home, lock or both). If the
system's wallpaper is already ours it is set by itself; if not, the system preview confirms it. A
folder without images says so and offers "Choose another". The app's tones come from the poster of
the wallpaper in use, once per change.

## Settings

Three blocks, Wallpapers, Screen and Phone, with the account as the first row and a "More options"
fold per block. The search box filters by the name of the row ignoring case and accents; if nothing
matches it says "No setting is called that". Quality is chosen in three words (Automatic, Save
battery, Maximum) and the seven levels stay behind "More options"; a fixed level rules.

The row "Fade between wallpapers" (`transition.kind`) explains what the choice does on this phone:
at once if it is off or if motion is reduced (`transition.less_motion` or the system's "remove
animations"), with a fade for `fade`, with the wallpaper's own transition for `own`, and with a low
fixed quality only the light transitions. The row says nothing about battery saving: pages do not
tell why a wallpaper is still.

## Private mode

Settings starts with the Privacy block and its single row, "Private mode". It is off by default and
is stored in `settings.toml` (`private_mode`), so it survives restarts.

When on it cuts the phone's whole network:

- Repositories are not updated (neither by hand nor the background job), a repository is not added by
  address and nothing is installed from a remote one.
- Covers or previews are not downloaded, nor depth models, and the wallpaper sources (picture of the
  day, WebDAV, Immich…) query nothing.
- Web wallpapers only load their own files: the view blocks the network and the native library
  answers empty to any allowed origin.
- Your server and your computer are not contacted: no sign-in, no voting, no listening on the local
  network and no search for nearby computers.

Everything local keeps working: the library, installed wallpapers, lists, rotation, settings and the
repositories included in the app.

The screens that use the network (Explore and repositories, the repository form, the account and
updates) show at the top a calm line, "Private mode: the network is cut.", with the "Turn off"
button, and their network buttons stay disabled. It is not the "No connection" notice: there is no
error, only the chosen cut.

Each language has a single gate (`net.rs` in the native library, `Net.kt` in the app) that refuses
before resolving any name, and a test goes through the code so that no connection is opened outside
it. `PrivateModeTest` and `PrivatePageTest` on the emulator, and the native test of the local
listener, check that with the mode on a listener on 127.0.0.1 receives no connection and that with
the mode off it does.

In Explore, the search box looks in all repositories; with more than one, a row of buttons ("All" and
one per repository) leaves only the shelf of the chosen one and the field says "Search in
<repository>". A chosen repository that is removed goes back to "All".

"Install" first reads again the index of the repository that offers the package (like "Update"): a
package published or changed since the last reading is installed as it is now, and a signature that
changed is reported ("the signature of the repository at … changed; nothing was applied") without
downloading. While it downloads, the package's button changes to "Installing…" with an indeterminate
bar, in Explore and in the repository list. Action errors are shown whole in a box with "Close", not
in a floating notice that the system cuts.

In Explore, each package installed from a repository carries "Uninstall", which removes it from the
library and deletes its install record: the repository offers it again with "Install". Removing an
installed wallpaper from the library does the same with its record.
