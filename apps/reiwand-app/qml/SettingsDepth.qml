import QtQuick
import io.github.reiwand.app

FocusScope {
    id: part

    required property var page
    readonly property var options: App.state.options ?? {}

    implicitHeight: rows.height + note.implicitHeight + Look.e4

    ListRows {
        id: rows

        ListRow {
            text: App.tr("Allow downloading the depth model")
            note: App.tr("Downloaded once, when a picture first needs it")
            kind: "switch"
            checked: part.options["models.allow_download"] !== false
            value: checked ? App.tr("Yes") : App.tr("No")
            onToggled: App.act("SetOption", ["models.allow_download", String(checked)])
        }
    }

    FillNote {
        id: note

        x: Look.rowSide
        y: rows.height + Look.e4
        width: rows.width - Look.rowSide * 2
    }
}
