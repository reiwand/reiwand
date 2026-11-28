import QtQuick
import QtQuick.Controls.impl
import io.github.reiwand.app

Item {
    id: control

    property string text
    property string icon
    property bool active: false
    property bool off: false
    property bool first: false
    readonly property bool hovered: hover.hovered
    default property alias extra: more.data

    signal clicked

    implicitWidth: row.implicitWidth + Look.e4 * 2 - Look.e1
    implicitHeight: Look.controlHeight
    activeFocusOnTab: !off
    Accessible.role: Accessible.Button
    Accessible.name: text
    Keys.onReturnPressed: !off && clicked()
    Keys.onEnterPressed: !off && clicked()
    Keys.onSpacePressed: !off && clicked()

    HoverHandler {
        id: hover

        enabled: !control.off
        cursorShape: Qt.PointingHandCursor
    }

    TapHandler {
        id: tap

        enabled: !control.off
        onTapped: control.clicked()
    }

    Rectangle {
        anchors.fill: parent
        radius: Look.radius
        color: tap.pressed ? Look.nightLine : control.hovered ? Look.nightHigh : "transparent"
    }

    Rectangle {
        visible: !control.first
        width: Look.line
        height: parent.height
        color: Look.nightLine
    }

    Row {
        id: row

        x: Look.e4 - Look.e1
        anchors.verticalCenter: parent.verticalCenter
        spacing: Look.e2

        IconImage {
            visible: control.icon !== ""
            anchors.verticalCenter: parent.verticalCenter
            name: control.icon
            sourceSize: Qt.size(Look.icon, Look.icon)
            color: control.off ? Look.mistLow : control.active ? Look.frost : Look.mist
        }

        Text {
            visible: control.text !== "" && more.children.length === 0
            anchors.verticalCenter: parent.verticalCenter
            text: control.text
            font.family: Look.textFont
            font.pixelSize: Look.tBody
            font.weight: Font.Medium
            color: control.off ? Look.mistLow : Look.frost
        }

        Row {
            id: more

            anchors.verticalCenter: parent.verticalCenter
            spacing: Look.e2
        }
    }

    FocusHalo {
        visible: control.activeFocus
        gap: 0
    }
}
