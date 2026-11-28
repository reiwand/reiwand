# Layer-shell and X11 outputs

Outside Plasma, on Wayland compositors that offer `zwlr_layer_shell_v1`, the daemon shows each
`wl_output` itself on a background layer surface, and in X11 sessions each RandR monitor on a
desktop window below all others (`reiwandd --backend auto|streams|layer-shell|x11`). These outputs
are named after the compositor's output names or the RandR monitor names and appear in
`GetState` like streams, with one claim owned by the daemon: `ReleaseStream` and
`SetOutputCovered` do not apply to them. They count as covered while the compositor stops
sending frame callbacks, while an active fullscreen or maximized window is on them, on Hyprland
while their workspace has a fullscreen window, on COSMIC while its overlap notices
(`zcosmic_overlap_notify_v1`) show windows that, with the panels that reserve space, cover the whole
surface, or in X11 while a window shown on the current desktop is fullscreen or maximized on them,
while the windows shown on the current desktop cover the monitor together (tiled windows) or while
the desktop window is fully obscured (`VisibilityNotify`, without a compositing manager);
`pause.when_covered` then pauses their video as for streams. In X11, still images also become the root window background (`_XROOTPMAP_ID` and
`ESETROOT_PMAP_ID`).

Every layer surface carries a namespace, which some compositors read to tell the desktop background
from other surfaces (and only then show it behind their overview). The first of these that is set
and not `auto` chooses it: `reiwandd --layer-namespace NAME`, the environment variable
`REIWAND_LAYER_NAMESPACE`, `surfaces.layer_namespace` of the configuration file, then a short table of
known desktops matched against the entries of `XDG_CURRENT_DESKTOP` without regard to case
(`KineticWE` gives `noctalia-wallpaper`), and otherwise `wallpaper`. The option refuses an invalid
or empty name before the daemon starts; a variable that is empty or invalid is ignored with a
warning in the log and in `ignored`. The name is chosen once, when the daemon connects to the
compositor, and only matters with layer surfaces. `GetState().layer_shell` holds `namespace` (the
name in use, `null` while the daemon shows no layer surfaces), `from` (`"option"`,
`"environment"`, `"config"`, `"automatic"` or `"default"`, empty with no layer surfaces),
`restart`, true while the configuration now chooses another name, which takes effect the next time
the daemon starts, and `ignored`, the reason a value given at start was refused, or `null`; an older
daemon leaves `ignored` out. `reiwand-ctl config surfaces.layer_namespace` says all this after a
change.

The systemd user service starts `reiwandd` without the environment or the options of a terminal: the
configuration key is the simple way there, and `systemctl --user edit reiwandd` takes
`Environment=REIWAND_LAYER_NAMESPACE=NAME` under `[Service]` or an `ExecStart=` override with
`--layer-namespace NAME`. The automatic choice reads `XDG_CURRENT_DESKTOP` from the user's systemd
environment (`systemctl --user show-environment`), which the session usually imports.

An output that appears, changes or disappears is applied once it has stayed the same for 0.5 s: an
output that the compositor reports twice, removes and adds back, or announces for a moment with a
zero size keeps its wallpaper, its playback and a single surface; a layer surface without a size
is shown only once the compositor gives it one. A new surface reports its own coverage at once, so
a screen that comes back is never left covered by a notice about the surface it replaced. When the
compositor closes the layer surface of an output that stays connected, the daemon creates it again,
at most three times a minute for each output; a fourth close within the minute waits until that
minute has passed and then creates it again. When the compositor closes the connection the
daemon exits with status 0; a protocol error ends it with status 70 so that the user unit starts
it again. In X11, an exposed desktop window is drawn again from the last frame, so a paused video
stays paused. The desktop windows are managed by the window manager when it announces
`_NET_WM_WINDOW_TYPE_DESKTOP` in `_NET_SUPPORTED`, and are *override-redirect* windows kept at
the bottom otherwise; the daemon replaces them when a window manager starts or stops. The desktop
window of another program, such as the icons of a file manager, stays above the wallpaper: the
daemon asks the window manager to stack its own windows below it (`_NET_RESTACK_WINDOW`, three times
in 10 s at most for each pair), and an opaque one that fills a monitor covers it. Without a window
manager, when another program keeps its own window at the bottom, the daemon lowers its windows
again three times in 10 s at most and then leaves the bottom to that window. When the root window
changes size, the root background is painted again.
