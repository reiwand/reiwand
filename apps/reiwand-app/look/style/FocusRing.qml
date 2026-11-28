import QtQuick
import io.github.reiwand.app

Rectangle {
    required property real corner

    anchors.fill: parent
    anchors.margins: -(Look.ringGap + Look.ring)
    radius: corner + Look.ringGap + Look.ring
    color: "transparent"
    border.width: Look.ring
    border.color: Look.frost
}
