import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

RowLayout {
    id: copy

    property string text: App.tr("No copy saved")

    signal save

    spacing: Look.e2

    IconImage {
        name: "warning"
        sourceSize: Qt.size(Look.noteIcon, Look.noteIcon)
        color: Look.amber
    }

    Text {
        Layout.fillWidth: true
        text: copy.text
        wrapMode: Text.WordWrap
        font.family: Look.textFont
        font.pixelSize: Look.tFoot
        color: Look.amber
    }

    QQC2.Button {
        flat: true
        text: App.tr("Save a copy…")
        onClicked: copy.save()
    }
}
