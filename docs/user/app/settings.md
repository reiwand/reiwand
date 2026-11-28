# Settings and permissions

The Settings screen, how to move your configuration to another computer, the questions a scene
asks before it uses something, and private mode.

## Settings

Three tabs: Wallpapers, Screens and Device. The search box at the top filters the rows of all
three. Rows that open something lead to a place with a path; Esc goes back.

### Wallpapers

| Group | Rows |
|-------|------|
| "What is shown" | "Mature content" (Hidden, Blurred, Visible) and "Flashes" (Disabled, Allowed). |
| "Your library" | "Library folders" (each offers to check it, rotate its photos or remove it) and "Lists per screen". |
| "Lock screen" | What the lock screen shows (see below). |
| "More" | Keyboard shortcuts for changing the wallpaper, "News from the repositories" (only when you ask, look for them by itself or install them by itself), "Thumbnail cache", "Show the number of votes" and "Watch the library". |
| "Making wallpapers" | "Light editor": the editor drawn as dark ink on light paper instead of its dark tones. Only the editor changes; the editor's View menu changes it too. |

#### Lock screen

"Lock screen" sets what the lock screen shows. It is "Like the desktop" until you choose.

- Inside, there is one row for every screen and, with several, one per screen.
- Each chooses between the desktop, the wallpaper picked for the lock screen and your lists.
- A screen set to "Like every screen" takes the choice of the first row.
- While that row has a choice, a screen can also pick "Like the desktop" to keep showing its
  desktop.

"Show on the lock screen" in the menu of a cover in the library picks that wallpaper (for the
screen the library targets, or for every screen with a single screen). With several screens, "Show
on every lock screen" picks it for all of them.

> **Note:** It is used where the Plasma lock screen has the reiwand wallpaper type.

### Screens

| Row | What it sets |
|-----|--------------|
| "Screen you adjust" | The screen (with several). |
| "Scene quality" | Automatic, Save battery or Maximum; the seven levels are under "More of …". What you set is always respected. |
| "Transition when the wallpaper changes" | The transition. |
| "Less motion" | Like the system, Always, Never. |
| "Pause" | Full screen, battery and lock. |
| "More of …" | Smoothness, fill color and HDR. |
| "Layouts" | Use it, Automatic rules, Rename and Delete it. |

### Device

| Row | What it holds |
|-----|---------------|
| "Private mode" | See [Private mode](#private-mode). |
| "Scene permissions" | Each permission of each scene is a row that withdraws it with Enter. |
| "Sound of the scenes", "What you are listening to" and "RGB lights" | Sound, media and lights (with OpenRGB). |
| "Advanced" | "Graphics card", HDR, "Depth quality", Maintenance ("Reload the settings", "Check the whole library"), the version and "Licenses". |

The details of the version hold "Install new versions by itself" when Reiwand came from the
AppImage.

## Moving your settings

Settings › Device › "Move your configuration" does two things.

**Export** writes to a file the repositories, providers, lists, favorites, ratings, tags, installed
wallpapers and settings of this computer.

- Your own wallpapers and the keys go in only if you check them.
- The keys encrypt the file with a password.

**Import** reads one made on another computer:

1. It shows what it brings.
2. It adds only what you check.
3. It lets you merge or keep both lists with the same name.

It is the same file that the phone uses and that `reiwand-ctl export` and `import` use.

## Permission questions

A scene can ask for the pointer, the computer's sound, clicks and touches, what is playing or the
network.

When you set it as the wallpaper, before it uses anything it asks you once per scene: "“name” asks
for permission to use", with one line per thing and "Allow" or "Without this".

> **Note:** A preview does not ask: it is shown without those permissions.

- The scene already works without the permissions.
- If something is left out, a notice says where to change it.
- On Plasma the question can also arrive as a desktop notification.
- What you answered is remembered and is withdrawn in Settings › Device › "Scene permissions".

## Private mode

In Settings › Device, first row. It cuts the whole network:

- sources and repositories are not updated;
- covers are not downloaded;
- web wallpapers load only files from your computer;
- your servers are not contacted.

What you already have keeps working.

Explore, Sources and Repositories show "Private mode: the network is cut." with the "Turn off"
button.
