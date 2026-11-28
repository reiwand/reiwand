import QtQuick
import io.github.reiwand.app

Rectangle {
    property real corner: Look.radiusSmall

    anchors.fill: parent
    anchors.margins: -(Look.ringGap + Look.ring)
    radius: corner + Look.ringGap + Look.ring
    color: "transparent"
    border.width: Look.ring
    border.color: Look.frost
}
