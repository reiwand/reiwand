import QtQuick
import io.github.reiwand.app

Item {
    id: unit

    required property var area
    required property string unit
    readonly property Item body: area.bodyOf(unit.unit, area.serial)

    parent: body ?? area
    anchors.fill: parent
    visible: !!body && area.layout.isShown(unit.unit)
}
