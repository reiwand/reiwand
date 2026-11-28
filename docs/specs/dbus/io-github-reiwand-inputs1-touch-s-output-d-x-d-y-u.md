# `io.github.reiwand.Inputs1`: `Touch` to `SetPointer`

On this page: `touch`, `setpointer`.

## `Touch(s output, d x, d y, u buttons)`

A press or a release on the wallpaper of `output` in Plasma, at `x`, `y` in fractions of the
output, with the buttons held after it (bit 0 left, 1 right, 2 middle). Only the Plasma shell that
runs the wallpaper plugin may call it (identified as for `Grant`, so never inside the Flatpak; the app
and `reiwand-ctl` may not), only while the output is in `clicking` and, while the KWin script reports
the pointer, only for the output the pointer is on; other callers get `AccessDenied`. The plugin
listens to a pointer handler of an item that it puts above everything else in the window of the
desktop, which sees every press and release without taking them, so the folder view of the desktop
(icons, context menu) keeps its clicks and the scene gets them too, also those that land on an icon
or a widget; a handler below the folder view would only see the clicks it leaves to the wallpaper,
none with the left button, and a KWin script cannot report them (its `workspace` has no pointer
buttons and a scripted effect, which sees them, cannot call the bus). The daemon cannot tell the plugin from other code running in the Plasma shell,
such as another applet, so it bounds what that code can send instead of ruling it out.

## `SetPointer(i x, i y)`

The pointer position in logical desktop coordinates, as sent by the KWin script. It is placed
on the output whose `SetOutputInfo` geometry contains it when that output follows the pointer, and
ignored otherwise. Only the KWin script of the daemon and `reiwand-ctl` (identified as for `Grant`,
so not inside the Flatpak) may call it: the connection that owns `org.kde.KWin` counts while the
daemon has its script loaded and, outside the Flatpak, only when it runs a system `kwin_wayland` or
`kwin_x11` (owned by root and not writable by others). Other callers, the app and the Plasma shell
included, get `AccessDenied`.
