import QtQuick
import io.github.reiwand.app

Item {
    id: grip

    required property var layout
    required property int index
    readonly property var edge: layout?.edges[index] ?? null
    readonly property bool across: edge?.dir === "column"
    property string base: ""
    property real from: 0

    function spot(mouse) {
        const at = drag.mapToItem(null, mouse.x, mouse.y);
        return across ? at.y : at.x;
    }

    function step(event) {
        return event.modifiers & Qt.ShiftModifier ? Look.e1 * 10 : Look.e1;
    }

    function nudge(by) {
        layout.resize(edge, by);
    }

    x: (edge?.x ?? 0) - (across ? 0 : width / 2)
    y: (edge?.y ?? 0) - (across ? height / 2 : 0)
    width: across ? edge?.length ?? 0 : Look.e2
    height: across ? Look.e2 : edge?.length ?? 0
    visible: !!edge && !edge.fixed
    activeFocusOnTab: true
    Accessible.role: Accessible.Separator
    Accessible.name: App.tr("Panel edge")
    Accessible.description: App.tr("Drag or use the arrow keys to resize, with Shift ten times as far; double click to return to the default size")
    // An arrow moves the edge a little; with Shift, ten times as far.
    Keys.onLeftPressed: event => across ? event.accepted = false : nudge(-step(event))
    Keys.onRightPressed: event => across ? event.accepted = false : nudge(step(event))
    Keys.onUpPressed: event => across ? nudge(-step(event)) : event.accepted = false
    Keys.onDownPressed: event => across ? nudge(step(event)) : event.accepted = false
    Keys.onReturnPressed: layout.even(edge)

    Rectangle {
        readonly property bool lit: drag.containsMouse || drag.pressed || grip.activeFocus

        anchors.centerIn: parent
        width: grip.across ? parent.width : (lit ? Look.ring : Look.line)
        height: grip.across ? (lit ? Look.ring : Look.line) : parent.height
        color: lit ? Look.moon : Look.nightLine
    }

    MouseArea {
        id: drag

        anchors.fill: parent
        hoverEnabled: true
        cursorShape: grip.across ? Qt.SizeVerCursor : Qt.SizeHorCursor
        onPressed: mouse => {
            grip.base = grip.layout.root;
            grip.from = grip.spot(mouse);
        }
        onPositionChanged: mouse => {
            if (pressed)
                grip.layout.resize(grip.edge, grip.spot(mouse) - grip.from, grip.base);
        }
        onDoubleClicked: grip.layout.even(grip.edge)
    }
}
