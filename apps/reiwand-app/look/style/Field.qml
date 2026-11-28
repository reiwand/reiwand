import QtQuick
import io.github.reiwand.app

Rectangle {
    id: field

    property bool lit: false
    property bool over: false
    property bool live: true

    implicitWidth: 200
    implicitHeight: Look.controlHeight
    radius: Look.radiusSmall
    color: live ? Look.nightHigh : "transparent"
    border.width: Look.line
    border.color: live && over && !lit ? Look.mistLow : Look.nightLine

    FocusRing {
        corner: field.radius
        visible: field.lit
    }
}
