import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

GridLayout {
    id: progress

    property string text
    property string count
    property real value: 0
    property bool indeterminate: false

    columns: 2
    rowSpacing: Look.listGap
    Accessible.role: Accessible.ProgressBar
    Accessible.name: text
    Accessible.description: count

    Text {
        Layout.fillWidth: true
        text: progress.text
        font.family: Look.textFont
        font.pixelSize: Look.tBody
        color: Look.frost
    }

    Text {
        text: progress.count
        font.family: Look.textFont
        font.pixelSize: Look.tBody
        color: Look.mist
    }

    QQC2.ProgressBar {
        Layout.columnSpan: 2
        Layout.fillWidth: true
        value: progress.value
        indeterminate: progress.indeterminate
        palette.dark: Look.frost
        palette.midlight: Look.nightLine
        Accessible.ignored: true
    }
}
