import QtQuick
import QtQuick.Controls.impl
import io.github.reiwand.app

Row {
    id: status

    property string text
    property string tone
    property string icon: Tone.icon(tone)
    property bool lit: false
    property real wrap: 0
    property real size: Look.statusIcon
    property real pixels: Look.tBody
    property color ink: Look.frost

    spacing: Look.statusGap
    Accessible.role: Accessible.StaticText
    Accessible.name: text

    IconImage {
        anchors.verticalCenter: parent.verticalCenter
        name: status.icon
        sourceSize: Qt.size(status.size, status.size)
        color: Tone.of(status.tone, status.lit, status.lit ? Look.night : Look.mist)
    }

    Text {
        anchors.verticalCenter: parent.verticalCenter
        width: status.wrap > 0 ? Math.min(implicitWidth, status.wrap - status.size - Look.statusGap) : implicitWidth
        text: status.text
        wrapMode: Text.WordWrap
        font.family: Look.textFont
        font.pixelSize: status.pixels
        font.weight: Font.Medium
        color: status.lit ? Look.night : status.ink
    }
}
