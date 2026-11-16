import QtQuick
import io.github.reiwand.app

Item {
    id: grip

    required property var layout
    required property string place
    readonly property bool across: place === "bottom"
    readonly property int sign: place === "left" ? 1 : -1
    readonly property real now: across ? layout.footHigh : place === "left" ? layout.leftWide : layout.rightWide
    property real from: 0

    function nudge(by) {
        layout.size(place, now + by);
    }

    width: across ? 0 : Look.e2
    height: across ? Look.e2 : 0
    visible: layout.places[place].length > 0
    activeFocusOnTab: true
    Accessible.role: Accessible.Separator
    Accessible.name: App.tr("Panel edge")
    Accessible.description: App.tr("Drag or use the arrow keys to resize; double click to return to the default size")
    Keys.onLeftPressed: event => across ? event.accepted = false : nudge(-Look.e4 * sign)
    Keys.onRightPressed: event => across ? event.accepted = false : nudge(Look.e4 * sign)
    Keys.onUpPressed: event => across ? nudge(Look.e4) : event.accepted = false
    Keys.onDownPressed: event => across ? nudge(-Look.e4) : event.accepted = false
    Keys.onReturnPressed: layout.size(place)

    Rectangle {
        anchors.centerIn: parent
        width: grip.across ? parent.width : (lit ? Look.anillo : Look.linea)
        height: grip.across ? (lit ? Look.anillo : Look.linea) : parent.height
        color: lit ? Look.luna : Look.nocheLinea
        readonly property bool lit: drag.containsMouse || drag.pressed || grip.activeFocus
    }

    MouseArea {
        id: drag

        anchors.fill: parent
        hoverEnabled: true
        cursorShape: grip.across ? Qt.SizeVerCursor : Qt.SizeHorCursor
        onPressed: mouse => {
            grip.from = grip.across ? grip.now + mapToItem(null, 0, mouse.y).y : grip.now - grip.sign * mapToItem(null, mouse.x, 0).x;
        }
        onPositionChanged: mouse => {
            if (pressed)
                grip.layout.size(grip.place, grip.across ? grip.from - mapToItem(null, 0, mouse.y).y : grip.from + grip.sign * mapToItem(null, mouse.x, 0).x);
        }
        onDoubleClicked: grip.layout.size(grip.place)
    }
}
