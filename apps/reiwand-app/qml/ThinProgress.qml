import QtQuick
import io.github.reiwand.app

Rectangle {
    id: bar

    property real value: -1

    visible: value >= 0
    implicitHeight: Look.line * 2
    height: implicitHeight
    radius: height / 2
    color: Look.veilColor
    Accessible.role: Accessible.ProgressBar

    Rectangle {
        width: parent.width * Math.max(0, Math.min(1, bar.value))
        height: parent.height
        radius: parent.radius
        color: Look.amber
    }
}
