import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

Rectangle {
    id: step

    property int number
    property string title
    property string done
    property string mode: "closed"
    property bool changeable: true
    readonly property bool open: mode === "open"
    readonly property alias changer: change
    default property alias body: content.data
    property alias after: tail.data

    signal change

    function plain(text) {
        return String(text).replace(/<[^>]*>/g, "");
    }

    Layout.fillWidth: true
    implicitHeight: column.implicitHeight + 2 * (open ? Look.e4 : Look.e3)
    radius: Look.radius
    color: Look.night
    border.width: Look.line
    border.color: open ? Look.mistLow : Look.nightLine
    opacity: mode === "closed" ? 0.7 : 1
    Accessible.role: Accessible.Grouping
    Accessible.name: mode === "done" ? step.plain(done) : title

    ColumnLayout {
        id: column

        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.leftMargin: Look.e5
        anchors.rightMargin: Look.e5
        anchors.topMargin: step.open ? Look.e4 : Look.e3
        spacing: Look.e3

        RowLayout {
            Layout.fillWidth: true
            spacing: Look.e4

            Item {
                Layout.preferredWidth: PublishLook.stepSide
                Layout.preferredHeight: PublishLook.stepNumber

                Rectangle {
                    width: PublishLook.stepNumber
                    height: PublishLook.stepNumber
                    radius: PublishLook.stepNumber / 2
                    color: step.open ? Look.frost : "transparent"
                    border.width: Look.line
                    border.color: step.open ? Look.frost : Look.nightLine

                    Text {
                        anchors.centerIn: parent
                        visible: step.mode !== "done"
                        text: step.number
                        font.family: Look.titleFont
                        font.weight: Font.Bold
                        font.pixelSize: Look.tBody
                        color: step.open ? Look.night : Look.mist
                    }

                    IconImage {
                        anchors.centerIn: parent
                        visible: step.mode === "done"
                        name: "check-bold"
                        sourceSize: Qt.size(Look.noteIcon, Look.noteIcon)
                        color: Look.sprout
                    }
                }
            }

            Text {
                Layout.fillWidth: true
                text: step.mode === "done" ? step.done : step.title
                textFormat: step.mode === "done" ? Text.StyledText : Text.PlainText
                elide: Text.ElideRight
                font.family: step.mode === "done" ? Look.textFont : Look.titleFont
                font.pixelSize: step.mode === "done" ? Look.tBody : Look.tSection
                font.weight: step.mode === "done" ? Font.Medium : Font.Bold
                color: step.mode === "done" ? Look.mist : Look.frost
                linkColor: Look.frost
            }

            QQC2.Button {
                id: change

                visible: step.mode === "done" && step.changeable
                flat: true
                text: App.tr("Change")
                Accessible.name: App.tr("Change: %1", step.plain(step.done))
                onClicked: step.change()
            }
        }

        ColumnLayout {
            id: content

            Layout.fillWidth: true
            Layout.leftMargin: PublishLook.stepSide + Look.e4
            visible: step.open
            spacing: Look.e3
        }

        ColumnLayout {
            id: tail

            Layout.fillWidth: true
            Layout.leftMargin: PublishLook.stepSide + Look.e4
            visible: step.mode === "done" && children.length > 0
            spacing: Look.e2
        }
    }
}
