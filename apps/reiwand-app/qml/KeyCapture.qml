import QtQuick
import io.github.reiwand.app

Item {
    id: capture

    property var keys: []
    property bool lit: false
    readonly property bool listening: activeFocus
    readonly property color ink: lit ? Look.night : Look.frost

    signal captured(var keys)
    signal done

    function listen() {
        forceActiveFocus(Qt.TabFocusReason);
    }

    function named(event) {
        const special = {
            [Qt.Key_Space]: "Space",
            [Qt.Key_Tab]: "Tab",
            [Qt.Key_Return]: "Enter",
            [Qt.Key_Backspace]: "Backspace",
            [Qt.Key_Delete]: "Delete",
            [Qt.Key_Home]: "Home",
            [Qt.Key_End]: "End",
            [Qt.Key_Left]: "Left",
            [Qt.Key_Right]: "Right",
            [Qt.Key_Up]: "Up",
            [Qt.Key_Down]: "Down"
        };
        if (special[event.key])
            return special[event.key];
        if (event.key >= Qt.Key_F1 && event.key <= Qt.Key_F12)
            return "F" + (event.key - Qt.Key_F1 + 1);
        if (event.key > 0x20 && event.key < 0x7f)
            return String.fromCharCode(event.key);
        return "";
    }

    implicitWidth: line.implicitWidth + 2 * Look.e3
    implicitHeight: Look.controlHeight
    Accessible.role: Accessible.Button
    Accessible.name: listening || keys.length === 0 ? App.tr("Press the combination…") : keys.join("+")
    Keys.onPressed: event => {
        event.accepted = true;
        if (event.key === Qt.Key_Escape)
            return done();
        const key = named(event);
        if (key === "")
            return;
        const held = [[Qt.MetaModifier, "Meta"], [Qt.ControlModifier, "Ctrl"], [Qt.AltModifier, "Alt"], [Qt.ShiftModifier, "Shift"]];
        const combination = held.filter(pair => event.modifiers & pair[0]).map(pair => pair[1]).concat([key]);
        captured(combination);
        done();
    }

    Canvas {
        id: edge

        readonly property color tone: capture.lit ? Look.night : Look.mistLow

        anchors.fill: parent
        onToneChanged: requestPaint()
        onWidthChanged: requestPaint()
        onPaint: {
            const draw = getContext("2d");
            draw.reset();
            draw.strokeStyle = tone;
            draw.lineWidth = Look.line;
            draw.setLineDash([3, 3]);
            draw.beginPath();
            draw.roundedRect(0.5, 0.5, width - 1, height - 1, Look.radiusSmall, Look.radiusSmall);
            draw.stroke();
        }
    }

    Row {
        id: line

        anchors.centerIn: parent
        spacing: Look.listGap

        Text {
            visible: capture.listening || capture.keys.length === 0
            anchors.verticalCenter: parent.verticalCenter
            text: App.tr("Press the combination…")
            font.family: Look.textFont
            font.pixelSize: Look.tBody
            color: capture.ink
        }

        Repeater {
            model: capture.listening ? [] : capture.keys

            Row {
                required property string modelData
                required property int index

                anchors.verticalCenter: parent.verticalCenter
                spacing: Look.listGap

                Text {
                    visible: parent.index > 0
                    anchors.verticalCenter: parent.verticalCenter
                    text: "+"
                    font.family: Look.textFont
                    font.pixelSize: Look.tBody
                    color: capture.ink
                }

                Rectangle {
                    implicitWidth: kbd.implicitWidth + 2 * 7
                    implicitHeight: kbd.implicitHeight + 2 * 3
                    radius: Look.radiusSmall
                    color: capture.lit ? Look.night : Look.nightHigh
                    border.width: Look.line
                    border.color: capture.lit ? Look.night : Look.nightLine

                    Text {
                        id: kbd

                        anchors.centerIn: parent
                        text: parent.parent.modelData
                        font.family: Look.textFont
                        font.pixelSize: Look.tKey
                        font.weight: Font.DemiBold
                        color: Look.frost
                    }
                }
            }
        }
    }
}
