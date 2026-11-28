# The app: `io.github.reiwand`

The desktop app owns `io.github.reiwand` (`APP_ID`) at `/io/github/reiwand` (`APP_PATH`)
and implements `org.freedesktop.Application` there. Its desktop file sets `DBusActivatable=true`
and `packaging/dbus/io.github.reiwand.service` lets the session bus start it for any of these
calls. A second `reiwand-app` passes its files and page to the running one through the same calls
and exits.

| Method | Effect |
|--------|--------|
| `Activate(a{sv} platform_data)` | raises the window |
| `Open(as uris, a{sv} platform_data)` | imports the `file://` URIs and absolute paths into the library, then raises the window |
| `ActivateAction(s name, av parameter, a{sv} platform_data)` | `library`, `playlists`, `profiles`, `providers`, `repos`, `settings` or `account` opens that page, also in an app the bus starts for this call; any other name only raises the window |

`platform_data` may carry `activation-token` (or `desktop-startup-id`): the app hands it to the
compositor when it raises the window, so that Wayland lets it take the focus. A second `reiwand-app`
passes its own `XDG_ACTIVATION_TOKEN` and `DESKTOP_STARTUP_ID` that way.

A request for the page already shown keeps it as it is, and while the image editor has unsaved
changes a page request only raises the window. `reiwand-app --page <name>` and the launcher action
*Settings* (`settings`) make the same request. The tray offers *Open reiwand* (`Activate`)
and *Settings…* (`ActivateAction("settings")`) unless `ListActivatableNames` lists the daemon but
not the app and the app is not running; there the tray says that the other settings are changed
with `reiwand-ctl`. The Flatpak owns the app's name through `--own-name` too and exports its
activation file, so the tray offers the app there as well. When the bus answers that it does not know the app
(`ServiceUnknown`, as with dbus-broker, which does not watch for new service files, right after an
install from the AppImage), the tray asks it to read its configuration again (`ReloadConfig`) and
tries once more; `reiwand --install` and `reiwand --uninstall` ask for that reload themselves.
