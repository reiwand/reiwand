import QtQuick
import io.github.reiwand.app

FocusScope {
    id: part

    required property var page
    readonly property var options: App.state.options ?? {}

    function flag(key, on) {
        App.act("SetOption", [key, String(on)]);
    }

    implicitHeight: rows.height

    ListRows {
        id: rows

        ListRow {
            text: App.tr("Let scenes play their sounds")
            kind: "switch"
            checked: part.options["playback.scene_sound"] !== false
            value: checked ? App.tr("Yes") : App.tr("No")
            onToggled: part.flag("playback.scene_sound", checked)
        }

        ListRow {
            text: App.tr("Let scenes react to the sound that plays")
            kind: "switch"
            checked: part.options["playback.scene_capture"] !== false
            value: checked ? App.tr("Yes") : App.tr("No")
            explain: App.tr("Each scene still asks you first. The sound is turned into levels on this computer and never recorded, kept or sent.")
            onToggled: part.flag("playback.scene_capture", checked)
        }

        ListRow {
            text: App.tr("Silence wallpapers while another application plays sound")
            kind: "switch"
            checked: part.options["playback.duck"] === true
            value: checked ? App.tr("Yes") : App.tr("No")
            onToggled: part.flag("playback.duck", checked)
        }
    }
}
