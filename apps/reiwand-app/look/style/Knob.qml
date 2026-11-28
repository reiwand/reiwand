import QtQuick
import io.github.reiwand.app

Rectangle {
    id: knob

    property bool live: true
    property bool lit: false
    property bool held: false

    implicitWidth: Look.railDot
    implicitHeight: Look.railDot
    radius: width / 2
    color: live ? (held ? Look.mist : Look.frost) : Look.mistLow
    border.width: live ? 0 : Look.line
    border.color: Look.nightLine

    FocusRing {
        corner: knob.radius
        visible: knob.lit
    }
}
