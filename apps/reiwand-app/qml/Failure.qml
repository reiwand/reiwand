import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

RowLayout {
    id: failure

    property string title
    property string text
    property string details
    property string action
    property bool big: false
    property string icon: "warning-circle"
    property string actionIcon
    property bool primary: false
    property string other
    readonly property alias button: main
    property alias detailsOpen: fold.open

    signal acted
    signal otherActed

    width: Math.min(big ? Look.errorLargeWidth : Look.errorWidth, parent ? parent.width : Look.errorWidth)
    spacing: Look.e4
    Accessible.role: Accessible.AlertMessage
    Accessible.name: title
    Accessible.description: text

    IconImage {
        Layout.alignment: Qt.AlignTop
        Layout.topMargin: failure.big ? Look.e2 : 2
        name: failure.icon
        sourceSize: failure.big ? Qt.size(Look.errorIconLarge, Look.errorIconLarge) : Qt.size(Look.errorIcon, Look.errorIcon)
        color: Look.coral
    }

    ColumnLayout {
        Layout.fillWidth: true
        Layout.alignment: Qt.AlignTop
        spacing: Look.e2

        Text {
            Layout.fillWidth: true
            text: failure.title
            wrapMode: Text.WordWrap
            lineHeight: 1.1
            font.family: Look.titleFont
            font.pixelSize: failure.big ? Look.tPage : Look.tRow
            font.weight: Font.Bold
            font.letterSpacing: failure.big ? -0.03 * Look.tPage : -0.015 * Look.tRow
            color: Look.frost
        }

        Text {
            Layout.fillWidth: true
            visible: failure.text !== ""
            text: failure.text
            wrapMode: Text.WordWrap
            lineHeight: 1.3
            font.family: Look.textFont
            font.pixelSize: failure.big ? Look.tExplain : Look.tBody
            color: Look.mist
        }

        RowLayout {
            Layout.topMargin: Look.e2
            visible: failure.action !== "" || failure.other !== ""
            spacing: Look.e3

            QQC2.Button {
                id: main

                visible: failure.action !== ""
                highlighted: failure.primary
                text: failure.action
                icon.name: failure.actionIcon
                onClicked: failure.acted()
            }

            PiecesKey {
                visible: failure.primary && main.visible
                text: App.trc("key", "Enter")
            }

            QQC2.Button {
                visible: failure.other !== ""
                text: failure.other
                onClicked: failure.otherActed()
            }
        }

        Details {
            id: fold

            Layout.fillWidth: true
            visible: failure.details !== ""
            details: failure.details
        }
    }
}
