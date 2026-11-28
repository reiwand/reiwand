# Scenes and what they can see

No scene hears the sound, follows the pointer or receives clicks without your permission. This page
covers how that permission works, what is captured and kept, the media player and scene scripts.

## Giving and withdrawing permission

The permission is given and withdrawn per scene from the app, the tray or `reiwand-ctl`, while the
scene is visible:

```
reiwand-ctl input grant|revoke <scene> audio|pointer|clicks
```

Asking for a permission that is already granted asks nothing.

### When a scene asks

When a scene that asks for them is applied without permission, from anywhere (the app, the tray, a
list or `reiwand-ctl`):

- the app and the tray ask once per scene what to allow, explaining what is captured and that
  nothing is kept or sent;
- their settings show a notice with the *Allow* button until you decide;
- *Not now* is remembered and does not ask again.

`reiwand-ctl lib apply`, `playlist start`, `next` and `prev` print the exact command to grant it.

## Clicks

Clicks are off on all screens until you turn them on for one
(`reiwand-ctl input clicks on -o <screen>`, or in the app).

Only real ones arrive:

| Desktop | Which clicks arrive |
|---------|---------------------|
| Sway and the other desktops with `wlr-layer-shell` | Those of the compositor, also the touches of a touch screen. |
| X11 | Those that no other program sends with `SendEvent`. |
| Plasma | Those of the desktop (see below). |

On Plasma, this also works with the folder view and its icons, which keep their clicks: the
wallpaper plugin sees them with a handler that does not take them, so a click on an icon or a
widget also reaches the scene.

- The plugin sends them and the daemon only accepts them from the Plasma interpreter (not from the
  app or `reiwand-ctl`).
- While the KWin script reports the pointer, they are accepted only on the screen where it is.

No other program can inject clicks over D-Bus.

> **Note:** The daemon cannot tell the plugin from other code that runs inside Plasma, such as
> another applet, and for that reason it bounds it instead of preventing it.

## Pointer positions

The daemon accepts pointer positions from no one except:

- the KWin script that it loads itself (outside the Flatpak, only if a system `kwin_wayland` or
  `kwin_x11` sends it);
- `reiwand-ctl`.

An image whose parallax follows the pointer uses it under the same rules, because choosing that
source is the permission.

## Which package holds the permission

The permission is tied to the package that was seen:

| Package | The permission is tied to |
|---------|---------------------------|
| Installed from a repository | The key that signs it (its updates keep it). |
| Any other | Its exact content. |
| In the editor preview | The draft folder if the app or `reiwand-ctl` asks, or its exact content if another program asks. |

So another package with the same id does not inherit it, not even if it is seen at the same time
on another screen. Then `reiwand-ctl input grant` asks for `-o` with the screen, and the app and
the tray give the permission to the package of the screen they ask about.

Permissions saved by earlier versions, which only had the id, are asked again.

## Requests from other programs

If another program asks for it over D-Bus, the daemon does not grant it.

1. It asks you with a Reiwand desktop notification that names the scene.
2. Only your answer in it counts, and it only applies it to that package. *Not now* does not remove
   the permission from other packages with the same id either.
3. If you answer *Not now*, close it or do not answer, that question does not come back until ten
   minutes later. The app, the tray and `reiwand-ctl input status` say so.
4. After that time, a permission you rejected can be asked for again as usual.

## When capture runs

Even with permission, sound capture and pointer tracking only work while a scene that uses them is
visible.

> **Note:** Sound capture is a PipeWire stream called "Reiwand audio analysis", which can turn on
> the microphone indicator.

They stop:

- when the screen is paused, covered or locked, even if pausing when covered is disabled;
- when no one is looking at the stream of that screen (on Plasma, when the desktop is not visible).

If PipeWire restarts, the capture reconnects by itself: first after 0.5 s and then waiting twice as
long each time, up to 30 s, without waking the computer while it is connected.

### Blocking sound capture for all scenes

`playback.scene_capture = false` blocks the capture for all scenes even if they have permission.
Set it with *Let scenes react to the sound that plays* in the settings of the app, the tray and the
phone, or with:

```
reiwand-ctl config playback.scene_capture false
```

## What is kept

| Input | What is kept |
|-------|--------------|
| Sound | Only bands and levels are computed from it, in memory. It is not kept, not logged and does not leave the computer. |
| Pointer | Only the last position, in memory. |
| Clicks | Only those waiting for the next frame. |
| Keys | Never read. |

## Android

On Android, the microphone permission is asked the first time *React to sound* is turned on in a
scene. It only listens while a scene that uses it and has that permission is visible.

## The media player

What plays in the media player is only read while a visible wallpaper with your permission uses
it, and is never kept, logged or sent.

Covers from the internet are only downloaded with `media.remote_covers`, off by default.

## Scene scripts

Scene scripts run in a separate process per scene, with no files, network or other processes.

They have limits on memory, CPU per frame, timers, layers created, console lines and shared values.

They never see paths, screen names or the battery.
