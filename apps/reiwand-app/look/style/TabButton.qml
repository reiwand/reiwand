import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.reiwand.app

T.TabButton {
    id: control

    // Above the other tabs, which a TabBar's list puts at 1, so its focus ring shows whole.
    z: visualFocus ? 2 : 0
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    padding: 0
    leftPadding: Look.e3
    rightPadding: Look.e3
    spacing: 7
    font.family: Look.titleFont
    font.pixelSize: Look.tFilter
    font.weight: checked ? Font.Bold : Font.Medium
    icon.width: Look.icon
    icon.height: Look.icon
    icon.color: control.ink

    readonly property color ink: !enabled ? Look.mistLow : checked || hovered || visualFocus ? Look.frost : Look.mist

    contentItem: IconLabel {
        spacing: control.spacing
        mirrored: control.mirrored
        display: control.display
        alignment: Qt.AlignLeft | Qt.AlignVCenter
        icon: control.icon
        text: control.text
        font: control.font
        color: control.ink
    }

    background: Item {
        implicitHeight: Look.controlHeight

        Rectangle {
            x: control.mirrored ? parent.width - width - control.rightPadding : control.leftPadding
            y: parent.height - height - 2
            width: Look.e4
            height: Look.e1
            radius: 1
            visible: control.checked
            color: control.enabled ? Look.moon : Look.mistLow
        }

        FocusRing {
            corner: Look.radiusSmall
            visible: control.visualFocus
        }
    }
}
