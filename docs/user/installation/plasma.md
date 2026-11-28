# Setting up Plasma and the tray

After installing, choose Reiwand as the wallpaper of the desktop and of the lock screen. This page
also covers the tray icon, the filter and opening the app on a given page.

## Desktop wallpaper

1. Right click on the desktop.
2. Choose *Configure Desktop and Wallpaper*.
3. Pick the wallpaper type **reiwand**.

## Lock screen

1. Open *System Settings* → *Screen Locking*.
2. Choose *Appearance: Configure…*.
3. Pick the wallpaper type **reiwand**.

It shows the wallpaper of each screen, or the lock wallpaper you chose in the app (Settings ›
Wallpapers › "Lock screen") or with `reiwand-ctl lock`.

It keeps running as long as "Pause" does not include the locked screen (the windows underneath do
not cover it).

## Notes on screens

**Fractional scaling.** With fractional scaling and Qt older than 6.11, the plugin asks for more
pixels than the screen has. The daemon reduces the stream to the real pixels of the screen, those
of the mode that the Wayland compositor announces, so as not to spend GPU or memory needlessly. The
size and scale the plugin passes are not enough, because they carry the same scale as the request.

**Screen priority.** Turning a screen on without a priority in *Display Configuration* can make it
the primary one, and Plasma then swaps the desktops of the screens. Each wallpaper asks for the
stream of its new screen before it gives back the old one, so the first screen's wallpaper goes on
without starting again.

> **Note:** Giving each screen its priority keeps each desktop on the screen it was on.

## The tray icon

The Reiwand tray icon gathers the basics with the look of the app:

- pause or resume, mute and the volume;
- a cover or a row per screen with the thumbnail and the name of its wallpaper;
- the buttons *Next* (only with a list running), *Choose…* and *⋯* (favorite, adjust, filter and
  remove the wallpaper);
- *Change with preview…*;
- *More* (sound, flashes and pause on battery).

If the wallpaper flashes or is still, the row says so. The tray also asks what to allow to a scene
that asks for something.

Everything else is in the app:

- *Open Reiwand* opens it.
- *Settings…* opens it directly on its *Settings* page, with the fit mode, the background color,
  the filter and the list of each screen or of all at once, the settings of each scene, the frames
  per second, the pauses and the transitions.

## The filter

The filter (the *Filter* button, or `reiwand-ctl filter`) tints any wallpaper of a screen, image,
video, scene or page, without touching the files.

It has one of 15 styles of its own, with its intensity: warm, cool, golden, vivid, intense, muted,
matte, faded, black and white, noir, sepia, sunset, mint, pink and night.

It also adjusts exposure, contrast, saturation, warmth and tone.

It goes inside the pass that draws the wallpaper on the screen, so it adds no other one, and with
no filter it costs nothing.

Profiles save and set the filter of each screen along with its wallpaper.

## Opening the app on a page

`reiwand-app --page settings` (or the *Settings* action of the launcher) does the same as
*Settings…* in the tray.

`--page` also accepts `library`, `scenes`, `playlists`, `profiles`, `providers`, `repos` and
`account`.

If the app is already open, it moves to that page.

> **Note:** If the image editor or the scene editor has unsaved changes, only the window is shown
> and the editor stays open.
