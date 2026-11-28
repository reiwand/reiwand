import QtQuick
import QtQuick.Controls.impl
import io.github.reiwand.app

FocusScope {
    id: search

    property alias text: input.text
    property string placeholder

    signal accepted

    implicitWidth: Look.searchWidth
    implicitHeight: Look.buttonHeight
    activeFocusOnTab: true
    Accessible.role: Accessible.EditableText
    Accessible.name: placeholder

    Rectangle {
        anchors.fill: parent
        radius: Look.radius
        color: Look.nightHigh
        border.width: Look.line
        border.color: Look.nightLine

        FocusHalo {
            visible: search.activeFocus
            corner: Look.radius
        }
    }

    IconImage {
        id: glass

        x: Look.searchSide
        anchors.verticalCenter: parent.verticalCenter
        width: Look.icon
        height: Look.icon
        sourceSize.width: width
        sourceSize.height: height
        name: "magnifying-glass"
        color: Look.mist
    }

    TextInput {
        id: input

        anchors.left: glass.right
        anchors.leftMargin: Look.searchGap
        anchors.right: parent.right
        anchors.rightMargin: Look.searchSide
        anchors.verticalCenter: parent.verticalCenter
        focus: true
        clip: true
        font.family: Look.textFont
        font.pixelSize: Look.tBody
        color: Look.frost
        selectionColor: Look.frost
        selectedTextColor: Look.night
        selectByMouse: true
        Accessible.ignored: true
        onAccepted: search.accepted()

        Text {
            visible: !input.text && !input.preeditText
            anchors.verticalCenter: parent.verticalCenter
            text: search.placeholder
            font: input.font
            color: Look.mist
        }
    }

    TapHandler {
        onTapped: search.forceActiveFocus()
    }
}
