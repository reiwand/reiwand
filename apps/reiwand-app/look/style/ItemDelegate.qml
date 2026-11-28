import QtQuick
import QtQuick.Controls.Basic as Basic
import io.github.reiwand.app

Basic.ItemDelegate {
    id: control

    // Chosen but not lit: the item chosen in a panel that does not have the keyboard.
    property bool marked: false

    background: Rectangle {
        implicitWidth: 100
        implicitHeight: 40
        visible: control.down || control.highlighted || control.marked || control.visualFocus || control.hovered
        radius: Look.radiusSmall
        color: control.highlighted ? (control.down ? Look.moonDeep : Look.moon) : control.down ? Look.night : control.hovered ? Look.nightLine : control.marked ? Look.nightHigh : Look.nightLine
        border.width: control.marked && !control.highlighted ? Look.line : 0
        border.color: Look.nightLine
    }
}
