import QtQuick
import io.github.reiwand.app

Rectangle {
    id: rail

    property real from: 0
    property real to: 1
    property bool upright: false
    property bool live: true

    implicitWidth: upright ? Look.railHeight : 120
    implicitHeight: upright ? 120 : Look.railHeight
    radius: Look.railHeight / 2
    color: Look.nightLine

    Rectangle {
        x: rail.upright ? 0 : rail.from * rail.width
        y: rail.upright ? (1 - rail.to) * rail.height : 0
        width: rail.upright ? rail.width : (rail.to - rail.from) * rail.width
        height: rail.upright ? (rail.to - rail.from) * rail.height : rail.height
        radius: rail.radius
        color: rail.live ? Look.frost : Look.mistLow
    }
}
