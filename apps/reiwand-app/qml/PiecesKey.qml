import QtQuick
import io.github.reiwand.app

Rectangle {
    property alias text: words.text

    implicitWidth: Math.max(Look.shortcutsHeight + Look.e1, words.implicitWidth + 2 * Look.e2)
    implicitHeight: Look.shortcutsHeight
    radius: Look.radiusSmall
    color: Look.nightHigh
    border.width: Look.line
    border.color: Look.nightLine

    Text {
        id: words

        anchors.centerIn: parent
        font.family: Look.textFont
        font.pixelSize: Look.tKey
        font.weight: Font.DemiBold
        color: Look.frost
    }
}
