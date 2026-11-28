import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

ColumnLayout {
    id: list

    required property var page
    property var rows: []

    spacing: Look.e1

    SceneRepeater {
        rows: list.rows
        keyed: row => row.widget + "\u0002" + row.pointer.replace(/^\/scene(\/layers\/\d+)+/, "")

        delegate: SceneInput {
            required property int index

            Layout.fillWidth: true
            page: list.page
            field: list.rows[index] ?? {}
        }
    }
}
