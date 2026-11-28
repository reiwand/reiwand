import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

RowLayout {
    id: row

    property var finding: ({})
    property bool first: false
    readonly property string level: finding.level ?? "note"

    signal go

    spacing: Look.e3
    Accessible.role: Accessible.StaticText
    Accessible.name: row.finding.message ?? ""

    IconImage {
        Layout.alignment: Qt.AlignTop
        Layout.topMargin: Look.e1
        name: row.level === "error" ? "x-circle" : row.level === "warning" ? "warning" : "info"
        sourceSize: Qt.size(Look.noteIcon, Look.noteIcon)
        color: row.level === "error" ? Look.coral : row.level === "warning" ? Look.amber : Look.mist
    }

    Text {
        Layout.fillWidth: true
        text: row.finding.message ?? ""
        wrapMode: Text.WordWrap
        font.family: Look.textFont
        font.pixelSize: Look.tBody
        color: Look.frost
    }

    QQC2.Button {
        Layout.alignment: Qt.AlignTop
        flat: true
        visible: (row.finding.pointer ?? "") !== ""
        text: App.tr("Go to the problem")
        onClicked: row.go()
    }
}
