import QtQuick
import QtQuick.Layouts
import "../look"

// After an update the tray keeps running the code Plasma loaded when it started: this says so, once,
// and offers to start Plasma again so the new tray loads.
ColumnLayout {
    id: updated

    required property var daemon

    visible: daemon.stale && !daemon.later
    spacing: Look.e2

    Words {
        Layout.fillWidth: true
        text: i18n("Reiwand was updated")
        title: true
        wrapMode: Text.Wrap
    }

    Words {
        Layout.fillWidth: true
        text: i18n("This tray still runs the version Plasma loaded when it started (%1; the service runs %2). Restart Plasma, or log out and in again, to load the new one.", updated.daemon.loaded, String(updated.daemon.current.version ?? ""))
        soft: true
        wrapMode: Text.Wrap
    }

    RowLayout {
        spacing: Look.e2

        Command {
            glyph: "arrow-clockwise"
            text: i18n("Restart Plasma")
            onClicked: updated.daemon.restartPlasma()
        }

        Command {
            quiet: true
            text: i18n("Later")
            onClicked: updated.daemon.later = true
        }
    }
}
