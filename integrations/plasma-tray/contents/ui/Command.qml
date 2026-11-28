import QtQuick
import QtQuick.Layouts
import QtQuick.Templates as T
import "../look"

T.AbstractButton {
    id: command

    property string glyph
    property bool quiet: false
    property bool danger: false

    implicitWidth: row.implicitWidth + leftPadding + rightPadding
    implicitHeight: Look.controlHeight
    leftPadding: Look.e3
    rightPadding: Look.e3
    focusPolicy: Qt.StrongFocus
    Accessible.name: text
    Accessible.role: Accessible.Button

    background: Rectangle {
        radius: Look.radius
        border.width: command.quiet ? 0 : Look.line
        border.color: Look.nightLine
        color: !command.enabled ? "transparent" : command.down ? Look.night : command.hovered || command.visualFocus ? Look.nightLine : command.quiet ? "transparent" : Look.nightHigh

        Ring {
            visible: command.visualFocus
            corner: Look.radius
        }
    }

    contentItem: RowLayout {
        id: row

        spacing: Look.buttonGap

        Glyph {
            visible: command.glyph !== ""
            name: command.glyph
            tone: command.enabled ? command.danger ? Look.coral : Look.mist : Look.mistLow
        }

        Words {
            visible: command.text !== ""
            text: command.text
            font.weight: Font.Medium
            color: !command.enabled ? Look.mistLow : command.danger ? Look.coral : Look.frost
        }
    }
}
