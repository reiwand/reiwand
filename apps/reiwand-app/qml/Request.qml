import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.impl
import io.github.reiwand.app

RowLayout {
    id: request

    property string icon
    property string text
    property string reason

    spacing: Look.e3
    Accessible.role: Accessible.StaticText
    Accessible.name: text
    Accessible.description: reason

    IconImage {
        Layout.alignment: Qt.AlignTop
        Layout.topMargin: 1
        name: request.icon
        sourceSize: Qt.size(Look.toastIcon, Look.toastIcon)
        color: Look.mist
    }

    Column {
        Layout.fillWidth: true

        Text {
            width: parent.width
            text: request.text
            wrapMode: Text.WordWrap
            font.family: Look.textFont
            font.pixelSize: Look.tBody
            font.weight: Font.DemiBold
            color: Look.frost
        }

        Text {
            width: parent.width
            text: request.reason
            wrapMode: Text.WordWrap
            font.family: Look.textFont
            font.pixelSize: Look.tFoot
            color: Look.mist
        }
    }
}
