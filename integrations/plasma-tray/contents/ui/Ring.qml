import QtQuick
import "../look"

Rectangle {
    property real corner: Look.radiusSmall

    anchors.fill: parent
    anchors.margins: -Look.ringGap
    color: "transparent"
    radius: corner + Look.ringGap
    border.width: Look.ring
    border.color: Look.frost
}
