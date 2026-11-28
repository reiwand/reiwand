# Changing the wallpaper with the keyboard

A shortcut opens a window in the center of the screen, above everything else, which is dimmed. It
shows the wallpaper that would result, large, and at the sides the previous and next ones as sharp
thumbnails; under the central one you read its name and its position ("3 of 12"). It opens already
moved one step, follows the list that screen plays (or your library) and nothing changes until you
apply it.

## With the shortcuts

The shortcuts are global: nothing needs to have focus.

- **Next** and **previous** (`LOGO+ALT+N` and `LOGO+ALT+P`): open the window and move the
  selection.
- **Apply** (`LOGO+ALT+Return`): keeps the one in the center, with the corresponding transition.
- **Cancel** (`LOGO+ALT+Escape`): closes the window; the real wallpaper stays as it was.
- If you touch nothing for 5 seconds (no keys, no shortcuts, no wheel, no moving the pointer over
  the window) it closes by itself and changes nothing; while you hold a button down on a
  thumbnail it does not close. If the window disappears suddenly, the daemon discards the
  selection after about 30 seconds.

With the window open: the ← and → arrows and the mouse wheel move the selection; Enter, a click on
the central one or the apply shortcut apply and close; Esc, a click outside or the cancel shortcut
close without changing. A click on any other thumbnail chooses it. If there is only one wallpaper,
the window says so and offers to open the library; if something fails, it names the problem and
lets you retry.

It follows the light or dark scheme of the system. If you asked for less motion (or your system
does), the selection jumps without sliding and the central one is not animated.

When closed, the window's process stays hidden for 5 minutes, doing no work (it uses no
processor; it keeps about 180 MB of memory) and, after that time unused, it exits. That way the
next time it appears instantly (about 50 ms); the first time after logging in, or after 5 minutes
unused, it takes about 300 ms because it has to start.

Only the wallpaper in the center moves: scenes and videos are seen running inside the frame, drawn
by the daemon on a separate target, without sound and without touching the real wallpaper; the
other thumbnails are still images. When moving to another wallpaper or closing the window the
target is released, so nothing is left working. It stays still if the computer is on battery or if
the wallpaper is photosensitive. The animation uses the QML modules of KPipeWire, the same as the
animated cards of the library; without them the central one stays a still image.

## Turning on the shortcuts

```sh
reiwand-ctl config shortcuts.enabled true
```

The program asks the desktop for four shortcuts (`shortcuts.next`, `shortcuts.previous`,
`shortcuts.apply` and `shortcuts.cancel`; they can also be changed in Settings). On Plasma, GNOME
and Hyprland the desktop registers them and may show you its own dialog to choose other keys.

## Where the shortcut is not registered by itself

On Sway and the other wlroots compositors, and on X11, the desktop does not offer shortcuts to
programs. Bind the commands to keys yourself; the window opens just the same:

```sh
bindsym $mod+Alt+n exec reiwand-ctl next --preview
bindsym $mod+Alt+p exec reiwand-ctl prev --preview
bindsym $mod+Alt+Return exec reiwand-ctl browse apply
bindsym $mod+Alt+Escape exec reiwand-ctl browse cancel
```

On X11 do the same with the shortcut manager of your environment (or `xbindkeys`). With
`--output NAME` the window opens only on that screen.

## Limits

- Library changes made with the window open are seen the next time you open it; at most two
  thousand wallpapers are taken from the library.
- With several screens and none chosen, the window opens on the screen where the pointer is
  (Plasma) and goes through the list of that screen; with `--output NAME` it opens on the one you
  indicate. Outside Plasma the first screen is used.
- The desktop may not accept the proposed keys (for example, if something else already uses
  them): choose others in its dialog or bind the commands by hand.
- Screens are named by their connector (`DP-1`, `HDMI-A-1`).
- The window does not show the author or the repository of the wallpaper: that is in the library.
- Only the central one is animated and only if its preview is a GIF or WebP.
