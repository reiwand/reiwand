# Getting and making wallpapers

The screens that bring wallpapers into the app: Explore (repositories), Sources (photos from the
internet), Accounts (private repositories and publishing) and Create (projects of your own).

## Explore

What it is for: installing wallpapers from the repositories.

### Cards

Each card shows a wallpaper.

| Button | What it does |
|--------|--------------|
| "Install" | The main button. It becomes "Set as wallpaper". |
| "Install and set as wallpaper" | Does both. |
| "Update to …" | Updates the wallpaper. |
| "Details" | Shows authors, version and the permissions it asks for. If you have an account on the repository's server, lets you vote for or against. |
| "Uninstall…" | Uninstalls it. |

### Adding a repository

**Repositories…** opens the list of repositories. "Add a repository" (always visible in Explore and
at the start of that list) adds one by pasting its address.

1. Paste the address.
2. If it is private, the box asks for an account: "Use the account X" or "Sign in" (approved in the
   browser).
3. If the server uses its own certificate, it shows its fingerprint and "Trust and add".

If you paste the address of a server, it says it is a server and names the addresses of its public
repositories. The full list is on the "Explore repositories" page of that server, with "Open in
Reiwand".

A `reiwand://add?url=…` link opens the box with the address filled in. Nothing is added until you
press "Add".

> **Warning:** If the signature of a repository changes, the app warns and applies nothing.

The included examples need no connection.

### Updates and publishing

- **Check for news** and **Update my wallpapers** bring in what is new.
- **More** gathers the tools for those who publish: "Upload a package…" and "Static repository…".

### When a repository fails

The card says so ("No connection with …", "The signature does not match") and offers "Try again"
or "See the examples".

> **Note:** Nothing whose signature does not match is ever installed.

## Sources

What it is for: bringing photos from the internet.

### Source cards

Each source is a card with its state and "N photos, updated … ago". Buttons: "Rotate these photos",
"Refresh" and "Details".

Under "Details":

- "Stored photos" says how much space they take and offers "Clear…".
- "Remove this source" removes the source.

### Adding a source

"Add a source": Bing, NASA and Wikimedia are added with one click. The others first ask for what
they need (search, server, key).

> **Note:** Keys are never shown again.

"More options" (Wallhaven) sets which photos and how to search for them. Each change is saved at
once.

### Provider plugins

Provider plugins add kinds of source Reiwand does not have built in.

- "Add a source" lists the installed ones after the other sources.
- "Provider plugins…" under them lists the plugins, with their version, the places they contact,
  the key they ask for and the sources that use them. It also installs one from a zip file or a
  folder.
- Before installing, it says what the plugin contacts and which key it asks for, and installs only
  after "Install". An update names the places it adds.
- "Remove…" deletes a plugin that no source uses.
- A source of a plugin shows as a card like the others.

The same is done with `reiwand-ctl provider plugin` ([the command line](../command-line.md)).

> **Note:** The photos belong to their authors and are fine as a wallpaper, not for publishing.

## Accounts

What it is for: getting into private repositories and publishing.

Everything is optional and nothing connects unless you ask. Accounts are linked to a repository and
you can have several. There is no synchronization between your computers.

- When you add a private repository, the box asks for an account: "Use the account X", "Use another
  account" or "Sign in" (a code that is approved in the browser).
- *Repositories* says which account each repository uses.
- When publishing you choose the target repository. If it has no linked account, it is asked for
  there, whether it is public or private.

> **Note:** Passwords do not travel between computers, except inside an exported, encrypted file.

## Create

What it is for: starting or opening a project of your own. It is for those who make wallpapers;
the rest of the app does not need it.

It offers:

- "Empty scene"
- Particles
- "Layered parallax"
- "Text clock"
- Image
- Video
- "Web page"
- Preset
- "Assets package"

plus your recent projects. Everything else is in [Creating scenes](../../authoring/README.md).
