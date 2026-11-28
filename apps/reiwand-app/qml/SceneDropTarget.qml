import QtQuick
import io.github.reiwand.app

Rectangle {
    id: place

    required property var drop
    required property var modelData
    readonly property bool lit: drop.choice?.key === modelData.key
    readonly property bool whole: modelData.kind === "stack"
    readonly property bool across: modelData.side === "up" || modelData.side === "down"
    readonly property real room: width - 2 * Look.e1
    readonly property real part: modelData.kind === "edge" ? room / 4 : room / 2

    x: modelData.x
    y: modelData.y
    width: modelData.width
    height: modelData.height
    radius: Look.radiusSmall
    color: Look.nightHigh
    border.color: lit ? Look.moon : Look.mistLow
    border.width: lit ? Look.ring : Look.line

    Rectangle {
        x: Look.e1 + (place.modelData.side === "right" ? place.room - place.part : 0)
        y: Look.e1 + (place.modelData.side === "down" ? place.room - place.part : 0)
        width: place.whole || place.across ? place.room : place.part
        height: place.whole || !place.across ? place.room : place.part
        radius: Look.line
        color: place.lit ? Look.moon : Look.mistLow
    }
}
