import QtQuick
import io.github.reiwand.app

Rectangle {
    property bool lit: false
    property bool over: false
    property bool down: false

    implicitWidth: 200
    implicitHeight: Look.optionHeight
    radius: Look.radiusSmall
    visible: lit || over || down
    color: lit ? (down ? Look.mist : Look.frost) : down ? Look.night : Look.nightHigh
}
