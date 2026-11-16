import QtQuick
import io.github.reiwand.app

Item {
    required property var area
    required property string unit
    readonly property string place: area.layout.placeOf(unit)

    parent: area.dock(place).body
    anchors.fill: parent
    visible: area.layout.current(place) === unit
}
