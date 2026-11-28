import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// One layout in the top bar of the editor: its blueprint and its name; the layout in use is lit and
// carries the accent mark. A click (or Ctrl and its number) changes the workbench to it.
QQC2.ToolButton {
    id: word

    required property var entry
    required property var layout
    property bool named: true
    readonly property bool lit: entry.chosen && JSON.stringify(entry.chosen) === JSON.stringify(layout.parsed.chosen)
    readonly property color ink: lit || hovered || visualFocus ? Look.frost : Look.mist

    implicitHeight: DeskLook.button
    leftPadding: DeskLook.gap
    rightPadding: DeskLook.gap
    text: entry.name
    Accessible.name: lit ? App.tr("Layout %1, in use", text) : App.tr("Layout %1", text)
    Accessible.role: Accessible.Button
    font.family: Look.titleFont
    font.pixelSize: DeskLook.tLayout
    font.weight: lit ? Font.Bold : Font.Medium
    QQC2.ToolTip.text: entry.key ? App.tr("%1 (%2)", text, entry.key) : text
    QQC2.ToolTip.visible: hovered
    onClicked: layout.choose(entry.chosen)

    background: Rectangle {
        radius: Look.radiusSmall
        color: word.down ? Look.night : word.hovered ? Look.nightLine : "transparent"

        Ring {
            visible: word.visualFocus
        }
    }

    contentItem: Row {
        spacing: DeskLook.gap

        SceneBlueprint {
            anchors.verticalCenter: parent.verticalCenter
            root: word.entry.root
            ink: word.ink
        }

        Text {
            anchors.verticalCenter: parent.verticalCenter
            visible: word.named
            text: word.text
            font: word.font
            color: word.ink
        }
    }

    Rectangle {
        x: word.leftPadding
        y: word.height - height
        width: Look.markWidth
        height: DeskLook.markHeight
        radius: Look.line
        visible: word.lit
        color: Look.moon
    }
}
