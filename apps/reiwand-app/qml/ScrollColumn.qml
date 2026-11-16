import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

ColumnLayout {
    id: column

    required property Item view

    x: Look.anilloLugar
    y: Look.anilloLugar
    width: view.availableWidth - 2 * Look.anilloLugar

    Binding {
        target: column.view
        property: "contentHeight"
        value: column.implicitHeight + 2 * Look.anilloLugar
    }
}
