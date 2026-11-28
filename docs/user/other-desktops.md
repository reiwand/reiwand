# Sway, Hyprland, river, niri and COSMIC

No plugin is needed here: `reiwandd` puts a surface on the background layer (`wlr-layer-shell`) on
each screen and draws on it with the GPU, without copies. It is installed with the AppImage and
started from the compositor's configuration:

```sh
exec reiwandd                       # Sway (~/.config/sway/config)
exec-once = reiwandd                # Hyprland (~/.config/hypr/hyprland.conf)
riverctl spawn reiwandd             # river (~/.config/river/init)
spawn-at-startup "reiwandd"         # niri (~/.config/niri/config.kdl)
```

If your Sway configuration has an `output … bg …` line (or you use another wallpaper program on
the background layer), it is worth knowing how the compositor orders them: on the same layer the
oldest surface is shown on top, not the newest. `reiwandd` keeps its surface alive (when the
wallpaper is removed it leaves it without an image instead of destroying it), so it keeps its place
when the wallpaper is removed and recovered and when you reload Sway (`swaymsg reload` launches
`swaybg` again, which becomes the newest). What the protocol does not allow is raising a surface
already created: if `swaybg` was created before the one of `reiwandd` (when starting `reiwandd`
after it, or when a screen is connected or reactivated and `swaybg` arrives first), the Sway
background ends up on top of the one of `reiwandd`. In that case run `swaymsg reload` once, or
remove the `output … bg …` line; without it, when the wallpaper is removed you see Sway's own
plain color.

On COSMIC, add it to the startup applications and choose a solid color as the desktop's own
background. If the AppImage install left the user service active, pass it the graphical environment from
the same configuration instead of launching the binary: `systemctl --user import-environment
WAYLAND_DISPLAY && systemctl --user restart reiwandd` (on Hyprland, also import
`HYPRLAND_INSTANCE_SIGNATURE`).

`reiwandd` chooses by itself: with Plasma (`plasmashell` on the bus or `KDE` in
`XDG_CURRENT_DESKTOP`) it uses the plugin's streams and on another Wayland desktop with
`wlr-layer-shell`, its own surfaces. `reiwandd --backend streams|layer-shell|x11` forces it. While
there is no wallpaper chosen (first start, or after *Remove the wallpaper* or `reiwand-ctl clear`)
`reiwandd` puts nothing on the screen: on layer-shell it destroys its surface, so the desktop's own
background is seen, and on X11 it destroys the window and gives the root window back what it
showed before `reiwandd` drew (the `_XROOTPMAP_ID` pixmap, or a copy of what was seen; X does not
report the plain color of `xsetroot`, which is why it comes back as a copy). Without a wallpaper
there are no frames, timers or GPU work on that screen, and the surface is created again when a
wallpaper is set or recovered. On Plasma the plugin cannot step aside by itself: it shows the
background color, and *Use Plasma's own wallpaper* (or `reiwand-ctl plasma-back`) asks Plasma,
through its scripting interface, to go back to the wallpaper type it had; setting or recovering a
wallpaper in reiwand-ctl turns it on again. Removing from the library the item that is the current
wallpaper also frees its screens. Screens are named as the compositor announces them (`DP-1`,
`HDMI-A-1`…; shown by `swaymsg -t get_outputs`, `hyprctl monitors`, `niri msg outputs` or
`wlr-randr`) and those names are the ones of `-o` and of the configuration. Screens that are
connected or removed with the session open are followed automatically, and fractional scale is
respected. A change of screens is applied once it has been stable for half a second, so a screen
that the compositor announces twice, removes and puts back, or leaves for an instant without a
size does not rebuild its surface or restart its video. If the compositor cuts the connection over
a protocol error, `reiwandd` exits with status 70 and the user service starts it again.

Each surface carries a name (the `wlr-layer-shell` *namespace*), which by default is `wallpaper`.
Some compositors decide by that name whether the surface is the desktop background, and only then
show it behind their overview; if yours looks black, choose another name. The first that is set wins, in
this order: `reiwandd --layer-namespace NAME`, the variable `REIWAND_LAYER_NAMESPACE`, the key
`surfaces.layer_namespace` of `config.toml` (`reiwand-ctl config surfaces.layer_namespace NAME`,
or *Surface name* in *Settings › Device › Advanced › For tests* in the app) and an automatic choice for known desktops: with
`KineticWE` in `XDG_CURRENT_DESKTOP` it uses `noctalia-wallpaper`. `auto` lets it pass to the next
one, and in the key an empty value too. The name has 1 to 64 ASCII letters, digits, `-`, `_` or
`.`; the option rejects a badly written or empty one, the key rejects a badly written one, and an
empty or badly written variable is ignored with a notice in the log and in `ignored` of the state.
It takes effect from the next start of the daemon and only with `layer-shell` surfaces:
`reiwand-ctl state` shows in `layer_shell` the name in use, where it came from, whether a change
is waiting for that start and which value was ignored, and `reiwand-ctl config
surfaces.layer_namespace NAME` and the app say so. It does not travel in the exported file or
enter the profiles, because it depends on the desktop of each computer.

The daemon starts as a systemd user service, which does not see the variables or options of your
terminal. The simplest is the key or the setting in the app; for the variable or the option, use
`systemctl --user edit reiwandd` and add `Environment=REIWAND_LAYER_NAMESPACE=NAME` under
`[Service]`, or an `ExecStart=` of your own with `--layer-namespace NAME`, and restart the
service. The automatic choice reads `XDG_CURRENT_DESKTOP` from the environment that the session
passes to systemd; if it does not arrive, use the key.

Videos pause when a window covers the screen: the compositor stops asking the wallpaper for
frames, an active full-screen or maximized window confirms it and, on Hyprland, its IPC too; on
COSMIC, its overlap notices, when the windows, together with the panels, cover the whole
background. A screen that is connected again is not left covered by a notice from the previous
surface, and if the compositor closes the wallpaper surface with the screen connected, `reiwandd`
creates it again: three times a minute at most, and if it closes it more times, it creates it
again as soon as that minute passes. The Flatpak does not work on these desktops: they do not let
sandboxed applications use `wlr-layer-shell` or the window information.

## X11 sessions

With `DISPLAY` and no Wayland or Plasma session, `reiwandd` puts a desktop window below all the
others on each RandR monitor (if the window manager does not place them, an *override-redirect*
window that stays at the bottom) and draws on it with the GPU. Monitors are named as in `xrandr
--listmonitors` and are followed as they are connected or removed. With still images it also sets
the background of the root window (`_XROOTPMAP_ID` and `ESETROOT_PMAP_ID`) for compositors and
terminals with transparency (and paints it again if the root window changes size), and videos
pause when a window of the current desktop is full screen or maximized on that monitor according
to EWMH, when several tiled windows cover it between them or, without a compositor, when the
server reports that the background window was fully covered. The desktop window of another
program, such as that of the icons of a file manager, always stays above the wallpaper (and, if
it is opaque, the screen counts as covered), and without a window manager `reiwandd` stops
contesting the bottom of the stack with another program that keeps its window there after lowering
its own three times in 10 s. It is started with `reiwandd &` in `~/.xinitrc`, before the window
manager, or from the autostart of the latter; with the user service,
`systemctl --user import-environment DISPLAY && systemctl --user restart reiwandd`. If the window
manager starts after `reiwandd` (or exits), the windows become managed desktop windows (or
*override-redirect*) by themselves. When a window stops covering the wallpaper, the last frame is
drawn again, also with the video paused, without restarting it.
