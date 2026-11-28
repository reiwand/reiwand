# Accent color, playback and shortcuts

A package can propose an accent color, limit its frame rate, set its volume and offer a shortcut.
This page explains how the desktop and the phone handle these settings, and how the card says where
a package works.

## Accent color

The author can propose an accent color (`display.accent`, a color or a color property).

- The desktop gives it to the Plasma wallpaper, which uses it if you choose the accent from the
  wallpaper.
- The phone gives it to the system.

Without it, the dominant color of the preview is proposed. `reiwand-ctl state` shows it per screen.

## Frame rate

`playback` also applies to scenes: frame limit, volume and silence of the author.

The frames per second never exceed the minimum among:

- what you ask for;
- what the author asks for;
- the screen;
- the battery;
- the quality level.

## Volume

Scenes with sound, videos and web pages have their own volume per screen. It starts at the author's
volume (or at zero if it asks to start silent) and is multiplied by the master volume.

### Silence while other apps play

| Platform | Setting | Behavior |
| --- | --- | --- |
| Desktop | *Silence wallpapers while another application plays sound* (`playback.duck`, off by default) | Lowers them with a fade while some other application plays, counting its PipeWire streams without reading their names or their content. |
| Phone | *Silence the wallpaper while another app plays sound* | Follows the active playbacks that Android announces, without knowing which app they are from or what they play. |

On the phone the setting only listens while a video, a scene or a page with sound is seen. It
lowers the sound only if there are more playbacks than its own: one per video that sounds, one per
page with sound and one per scene that sounds.

## Quality levels

Properties with values per quality level (`quality`) take them by themselves when the computer
saves power. Scripts and pages receive the level.

## Shortcuts

A shortcut (`shortcut`) is chosen by you in the wallpaper's settings. On the desktop it can be:

| Target | How it is chosen or opened |
| --- | --- |
| An `https://` address | Opened with the desktop portal. |
| A file or a folder | Chosen with the picker. |
| An application | By its `.desktop` id (outside the Flatpak). |

The wallpaper never sees what you chose. It can only ask to open it within a real click, once a
second at most, on a screen that accepts clicks.

On the phone you choose, on the package's page:

- an installed app;
- an `https://` address; or
- a file or folder from the document picker (stored with a read permission that survives a
  restart).

It is opened with a tap on the home screen if the package has permission for taps (never from the
lock screen).

If it cannot be opened (the app is no longer installed, no app opens the address or Android rejects
it), *Settings* says so in one line until it is dismissed or a shortcut opens fine. That line is
kept even if the process restarts.

> **Note:** Android does not report when it silently blocks a wallpaper service from opening an
> app, and that cannot be detected.

## Warnings and where a package works

- If a scene uses clicks on a screen that they do not reach (such as Plasma inside the Flatpak),
  its settings warn of it.
- A package that asks for compute shaders warns before applying it if the graphics do not have
  them.
- The card of each package says on which computers it works entirely (`desktop`, `android`).

The validator computes where it works from the features it uses and the phone's limits:

| Phone limit | Value |
| --- | --- |
| Uncompressed size | 1 GiB |
| Image size per side | 8192 px |
| Image size | 32 Mpx |
