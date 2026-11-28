# The phone app

The phone app is for having and using wallpapers. It has no editor or tools to create them; for
that, use the desktop app and [Creating scenes](../authoring/README.md).

At the bottom there are three words: **Library**, **Lists** and **Settings**. Behind them you see
the wallpaper you have set.

## Library

### Layout

| Part | What it holds |
| --- | --- |
| Top right | Search and "Add". |
| Card of the wallpaper in use | Where it is set, the title, its details, "Options" and "Favorite". |
| Grid of covers | Below the card. The cover of the wallpaper in use says "In use". |

> **Note:** A wallpaper with flashes has an amber line under the name and is not set while flashes
> are disabled.

With an empty library it says "Nothing here yet" and offers "Explore repositories" and "Add
photos".

### Search and add

- **Search** finds by title in any language of the package. With no results it says what you
  searched for and lets you clear it.
- **Add** gathers photos, files (packages too), folders and animated wallpapers.

### Choose several

1. Long-press a cover.
2. At the top you get "Cancel", "N chosen" and "Choose all".
3. At the bottom you get:
   - "Playlist";
   - "Favorite";
   - "More": remove from the list, remove favorite, adjust, remove;
   - "Rotate these", which creates the list "Rotation", which changes every hour. The notice
     brings "Change" for the interval.

## Wallpaper page

It opens when you tap a cover. From top to bottom:

1. **Preview**, with the back arrow. On an image you can drag and pinch to frame it; "Reset
   framing" undoes it.
2. **Title**, "In use", type, size and duration.
3. **Hold-back line.** If the wallpaper is set and the phone is holding it back, a line says so:
   "Still because it is heavy", "Quality lowered for the battery" or "Quality lowered for the
   heat". "Play anyway" sets the quality to "minimum 1" for all wallpapers; it is undone in
   Settings › Quality.
4. **Details:** stars, tags ("Add tags"), "Rename" and the description.
5. **Permissions.** If the wallpaper asks for permissions, a "This wallpaper asks for" block offers
   "Without it" or "Allow" on each one.
6. **Fixed bar at the bottom:** Favorite, "Add to a list", Remove (asks for confirmation) and "Set
   as wallpaper", which asks once where: home screen, lock screen or both.

> **Note:** Until you answer a permission, the wallpaper works without it. "Allow" for sound first
> asks the system for the microphone.

## Lists

Each list is a row with its thumbnail, name, "In use", the interval and how many wallpapers it has
("Every hour · 3 wallpapers"). "New playlist" asks for a name.

### Inside a list

- The pencil "Rename the playlist".
- "Change wallpaper": every 10, 15 or 30 minutes, every hour, every 6 hours, every day or never.
- "Shuffle".
- "Skip videos on battery".
- The same wallpaper on home and lock.
- Fixed days and hours.
- "More options": "Resume where it stopped" and "Fade between its wallpapers".
- "Set as wallpaper" sets it.
- "Delete playlist" asks first, and the wallpapers stay in the library.

- In a manual list, each row has "Remove from the list" and a handle for ordering.
- A list with a search fills itself; "Make manual" converts it.

## Settings

Five blocks with a search box ("Search the settings"): Privacy, Wallpapers, Screen, Phone and
About Reiwand, with accounts as the first row.

- **Quality**: Automatic, Save battery or Maximum; the seven levels are under "More options". A
  fixed level always rules.
- **Update repositories automatically** and "Update all".
- Mobile data for repositories and depth-model downloads, off until you allow them.
- Mature content and flashes.
- "Volume of videos, scenes and pages".

### About Reiwand and updates

**About Reiwand** shows the installed version with "Look for a new version", and the switch "Look
for new versions automatically" (on by default).

The check reads the latest release on GitHub once a day:

- on Wi-Fi unless mobile data is allowed;
- with the battery not low;
- never in private mode.

How an update is installed:

1. A newer version is downloaded for the phone's processor (`reiwand-android-arm64-v8a.apk`,
   `reiwand-android-x86_64.apk` or `reiwand-android.apk`).
2. It is kept only if its SHA-256 matches the release's `SHA256SUMS`, its version is that of the
   release and it is signed with the same certificate as the installed app. Otherwise nothing is
   installed and the row says why.
3. A notification "Reiwand X is ready to install" (or "Install X" in the row) hands it to Android.
4. Android always asks before replacing the app. The first time, Android asks to allow Reiwand to
   install apps.

## Repositories and sources

### Install from Explore

From Explore (the examples screen of the empty library, or the list of repositories) you install
packages.

- The search box looks in all repositories. With more than one, a row of buttons ("All" and one
  per repository) limits the search to just one.
- Before downloading, the app reads that repository's index again. If it changed on the server,
  what it offers now is installed. If its signature changed, it says so without downloading
  anything.
- While a package downloads, its "Install" button changes to "Installing…" with a bar.
- An error is shown in full, in a box with "Close".

### Add a repository

"Repositories" lets you add one with just an address, no fingerprint or QR.

| Kind | What happens |
| --- | --- |
| Public | It is added at once. |
| Private | It asks for an account: "Use the account X", "Use another account" or sign in with a code in the browser. The account is tied to that repository and its key is only sent to its server. |
| Server with its own certificate | Its fingerprint is shown so you can trust it. |
| Address of a server | Its repositories are listed so you can choose one. |

A `reiwand://add?url=…` link opens this dialog with the address filled in.

> **Note:** Only https:// addresses are valid, or http:// of the phone itself.

### Pictures from the internet

Under "Pictures from the internet" you add sources: Bing picture of the day, NASA or Wikimedia,
Wallhaven, WebDAV, Nextcloud, Immich. Nothing is downloaded until you turn it on with "Download
from the internet".

### Provider plugins

"Provider plugins", under them, lists the installed plugins (version, the sources that use them).

To install one:

1. Tap "Install a plugin from a zip file". It opens the system file picker.
2. Before installing, the phone shows the version, the licence, the places it contacts and the key
   it asks for. An update names the places it adds.
3. Tap "Install". It installs only then.
4. Its kind of source then shows under "Add a picture source", with the plugin's own settings.

Tapping a plugin shows what it contacts. "Remove…" deletes one that no source uses.

These are the same plugins and the same rules as on the desktop.

## Accounts and settings file

Settings › "Account" lists the accounts with their repositories. You can stop using one on a
repository or remove it.

| Action | What it does |
| --- | --- |
| "Export the configuration" | Saves the sections you check to a file, with the system file picker. Passwords only leave if you check it, encrypted with the password you choose. |
| "Import a configuration" | Opens a file exported on the phone or on the desktop, shows what it brings and only adds what you check, replacing nothing. With repeated lists you can merge them or keep both. |

> **Note:** Account sessions do not travel: you sign in again.

## Private mode

Settings › Privacy › "Private mode" cuts the phone's whole network: no repositories, covers,
sources or computer. What is local stays the same.

The network screens show "Private mode: the network is cut." with "Turn off".
