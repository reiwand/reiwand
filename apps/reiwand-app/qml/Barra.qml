import QtQuick
import io.github.reiwand.app

Rectangle {
    id: barra

    property real value: -1

    visible: value >= 0
    implicitHeight: Look.linea * 2
    height: implicitHeight
    radius: height / 2
    color: Look.veloColor
    Accessible.role: Accessible.ProgressBar

    Rectangle {
        width: parent.width * Math.max(0, Math.min(1, barra.value))
        height: parent.height
        radius: parent.radius
        color: Look.ambar
    }
}
