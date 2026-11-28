import QtQuick
import io.github.reiwand.app

Rectangle {
    property real corner: Look.radius
    property int gap: Look.ringGap

    anchors.fill: parent
    anchors.margins: -(gap + Look.ring)
    radius: corner + gap + Look.ring
    color: "transparent"
    border.width: Look.ring
    border.color: Look.frost
}
