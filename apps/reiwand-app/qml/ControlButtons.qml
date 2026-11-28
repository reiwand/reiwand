import QtQuick
import io.github.reiwand.app

Rectangle {
    id: group

    property string label
    default property alias controls: row.data

    implicitWidth: row.implicitWidth + Look.line * 2
    implicitHeight: Look.controlHeight
    radius: Look.radius
    color: Look.night
    border.width: Look.line
    border.color: Look.nightLine
    Accessible.role: Accessible.Grouping
    Accessible.name: label

    Row {
        id: row

        x: Look.line
        height: parent.height
    }
}
