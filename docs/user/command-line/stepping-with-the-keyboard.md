# Stepping through wallpapers with the keyboard

You can look at the next or previous wallpaper before applying it, from the terminal or with global
shortcuts. The strip that appears over the wallpaper when stepping through them is covered in
[Changing the wallpaper with the keyboard](../keyboard-switching.md).

## Browsing from the terminal

`reiwand-ctl next --preview` and `reiwand-ctl prev --preview` show which the next or the previous
wallpaper would be without changing anything. They start going through the list that screen shows
(or the library, with `--from library` or if the screen has no list), and each call moves forward or
back one.

| Command | What it does |
|---------|--------------|
| `reiwand-ctl next --preview` | shows the next one |
| `reiwand-ctl prev --preview` | shows the previous one |
| `reiwand-ctl browse apply` | sets the one you are looking at (with `--transition fade` or `none` for that change only) |
| `reiwand-ctl browse cancel` | leaves everything as it was |
| `reiwand-ctl browse show` | says which it is |

If you touch nothing for 30 seconds, it stops browsing by itself.

## Global shortcuts

With `reiwand-ctl config shortcuts.enabled true` the program asks the desktop for two global
shortcuts. It proposes `LOGO+ALT+N` and `LOGO+ALT+P`, which are changed with `shortcuts.next` and
`shortcuts.previous`.

On Plasma, GNOME and Hyprland the desktop registers them and may show you its own dialog to choose
other keys.

## Sway, wlroots and X11

On Sway and the other wlroots compositors, and on X11, there is no shortcuts portal. Bind the
commands to keys in your compositor, for example on Sway:

```sh
bindsym $mod+Alt+n exec reiwand-ctl next --preview
bindsym $mod+Alt+p exec reiwand-ctl prev --preview
bindsym $mod+Alt+Return exec reiwand-ctl browse apply
bindsym $mod+Alt+Escape exec reiwand-ctl browse cancel
```

On X11, do the same with the shortcut manager of your environment (or `xbindkeys`).

## The strip

The shortcut, the tray and `reiwand-ctl next --preview` draw a strip over the wallpaper with the
current one, the previous and the next ones. The shortcuts and the wheel step through, and a click
on the central one applies. The guide `docs/user/keyboard-switching.md` explains it with its limits.
