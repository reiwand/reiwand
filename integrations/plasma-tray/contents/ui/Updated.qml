import QtQuick
import QtQuick.Layouts
import "../look"

// After an update the tray keeps running the code Plasma loaded when it started: this says so, once,
// and offers to start Plasma again so the new tray loads. With the Flatpak, after `flatpak update`
// the service runs the old version too: it offers one restart that also refreshes Plasma's copies.
ColumnLayout {
    id: updated

    required property var daemon

    readonly property bool flatpak: daemon.flatpakNewer

    visible: (daemon.stale || flatpak) && !daemon.later
    spacing: Look.e2

    Words {
        Layout.fillWidth: true
        text: updated.flatpak ? i18n("A new version of Reiwand is installed") : i18n("Reiwand was updated")
        title: true
        wrapMode: Text.Wrap
    }

    Words {
        Layout.fillWidth: true
        text: updated.flatpak ? i18n("Reiwand %1 is installed, but %2 is still running. Restart it to use the new one: the wallpaper plugin and this tray are refreshed, and Plasma starts again.", updated.daemon.installed, String(updated.daemon.current.version ?? "")) : i18n("This tray still runs the version Plasma loaded when it started (%1; the service runs %2). Restart Plasma, or log out and in again, to load the new one.", updated.daemon.loaded, String(updated.daemon.current.version ?? ""))
        soft: true
        wrapMode: Text.Wrap
    }

    RowLayout {
        spacing: Look.e2

        Command {
            glyph: "arrow-clockwise"
            text: updated.flatpak ? i18n("Restart Reiwand") : i18n("Restart Plasma")
            onClicked: updated.flatpak && updated.daemon.probe ? updated.daemon.probe.restart() : updated.daemon.restartPlasma()
        }

        Command {
            quiet: true
            text: i18n("Later")
            onClicked: updated.daemon.later = true
        }
    }
}
