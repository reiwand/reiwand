import QtQuick
import io.github.reiwand.app

FocusScope {
    id: part

    required property var page
    readonly property bool updating: App.backend.selfUpdating()
    readonly property string problem: [App.state.config_error, App.state.library_error].filter(text => !!text).join("\n")

    implicitHeight: rows.height

    ListRows {
        id: rows

        ListRow {
            text: App.tr("Version")
            value: App.state.version ?? ""
        }

        ListRow {
            text: App.tr("Install new versions by itself")
            visible: part.updating
            kind: "switch"
            checked: (App.state.options ?? {})["updates.auto"] !== false
            value: checked ? App.tr("Yes") : App.tr("No")
            explain: App.tr("Once a day it asks GitHub (github.com) for the latest release of Reiwand and, when it is newer and signed with the Reiwand key, installs it. Nothing else is sent. Private mode stops it too.")
            status: checked && App.cut ? App.tr("Private mode is on: no check is made.") : ""
            onToggled: App.act("SetOption", ["updates.auto", String(checked)], "Daemon1")
        }

        ListRow {
            text: App.tr("Graphics")
            value: App.state.adapter ?? ""
            visible: value !== ""
        }

        ListRow {
            text: App.tr("Data model")
            value: String(App.state.model ?? "")
        }

        ListRow {
            text: App.tr("Configuration")
            value: part.problem === "" ? App.tr("No problems") : App.tr("Has a problem")
            explain: part.problem
            status: part.problem === "" ? "" : App.tr("Fix the file and use “Reload the settings”.")
            statusTone: "coral"
        }
    }
}
