import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.impl
import io.github.reiwand.app

ColumnLayout {
    id: error

    property string icon: "warning-circle"
    property string title
    property string text
    property string more
    property string details
    default property alias actions: row.data

    Layout.fillWidth: true
    spacing: Look.e2
    Accessible.role: Accessible.AlertMessage
    Accessible.name: title
    Accessible.description: text

    IconImage {
        name: error.icon
        sourceSize: Qt.size(Look.errorIcon, Look.errorIcon)
        color: Look.coral
    }

    Text {
        Layout.fillWidth: true
        text: error.title
        wrapMode: Text.WordWrap
        font.family: Look.titleFont
        font.pixelSize: Look.tAside
        font.weight: Font.Bold
        color: Look.frost
    }

    Text {
        Layout.fillWidth: true
        visible: error.text !== ""
        text: error.text
        wrapMode: Text.WordWrap
        lineHeight: 1.3
        font.family: Look.textFont
        font.pixelSize: Look.tBody
        color: Look.mist
    }

    Text {
        Layout.fillWidth: true
        visible: error.more !== ""
        text: error.more
        wrapMode: Text.WordWrap
        lineHeight: 1.3
        font.family: Look.textFont
        font.pixelSize: Look.tBody
        color: Look.mist
    }

    RowLayout {
        id: row

        Layout.topMargin: Look.e2
        spacing: Look.e3
    }

    Details {
        Layout.fillWidth: true
        visible: error.details !== ""
        details: error.details
    }
}
