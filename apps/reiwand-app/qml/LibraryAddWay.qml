import QtQuick
import QtQuick.Controls.impl
import io.github.reiwand.app

Rectangle {
    id: way

    property string text
    property string reason
    property bool marked

    signal picked

    width: parent ? parent.width : implicitWidth
    implicitHeight: words.implicitHeight + 2 * Look.e3
    radius: Look.radius
    color: "transparent"
    border.width: Look.line
    border.color: marked ? Look.frost : Look.nightLine
    activeFocusOnTab: true
    Accessible.role: Accessible.RadioButton
    Accessible.name: text
    Accessible.description: reason
    Accessible.checkable: true
    Accessible.checked: marked
    Accessible.onPressAction: picked()
    Keys.onSpacePressed: picked()
    Keys.onReturnPressed: picked()
    Keys.onEnterPressed: picked()

    FocusHalo {
        visible: way.marked || way.activeFocus
        corner: way.radius
    }

    IconImage {
        id: mark

        x: Look.e3
        y: Look.e3 + 1
        name: way.marked ? "radio-button-fill" : "radio-button"
        sourceSize: Qt.size(Look.toastIcon, Look.toastIcon)
        color: way.marked ? Look.frost : Look.mist
    }

    Column {
        id: words

        x: mark.x + mark.width + Look.e3
        y: Look.e3
        width: way.width - x - Look.e3

        Text {
            width: parent.width
            text: way.text
            wrapMode: Text.WordWrap
            font.family: Look.textFont
            font.pixelSize: Look.tBody
            font.weight: Font.DemiBold
            color: Look.frost
        }

        Text {
            width: parent.width
            text: way.reason
            wrapMode: Text.WordWrap
            lineHeight: 1.3
            font.family: Look.textFont
            font.pixelSize: Look.tFoot
            color: Look.mist
        }
    }

    TapHandler {
        onTapped: way.picked()
    }
}
