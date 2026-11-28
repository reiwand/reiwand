import QtQuick
import io.github.reiwand.app

// The blueprint of a layout: a box per group, drawn from the same tree that builds the workbench;
// the group of the canvas is filled.
Item {
    id: plan

    property string root: ""
    property color ink: Look.mist
    readonly property Workbench bench: Workbench {}
    readonly property var boxes: root ? JSON.parse(bench.blueprint(root, width, height) || "[]") : []

    implicitWidth: DeskLook.planWidth
    implicitHeight: DeskLook.planHeight
    Accessible.ignored: true

    Repeater {
        model: plan.boxes

        delegate: Rectangle {
            required property var modelData

            x: modelData.x
            y: modelData.y
            width: modelData.width
            height: modelData.height
            radius: 1
            color: modelData.canvas ? Qt.rgba(plan.ink.r, plan.ink.g, plan.ink.b, DeskLook.planFill) : "transparent"
            border.width: Look.line
            border.color: plan.ink
        }
    }
}
