import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.reiwand.app

T.ToolButton {
    id: control

    readonly property bool chosen: checked || highlighted
    readonly property color ink: !enabled ? Look.mistLow : chosen ? Look.night : hovered || visualFocus || down ? Look.frost : Look.mist

    z: visualFocus ? 1 : 0
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    // In the editor's denser measures every tool button is exactly as high, with text or without.
    implicitHeight: Look.desk ? Look.toolHeight : Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    padding: (Look.toolHeight - Look.icon) / 2
    leftPadding: display === T.AbstractButton.IconOnly ? padding : 10
    rightPadding: display === T.AbstractButton.IconOnly ? padding : 10
    spacing: 6
    font.family: Look.textFont
    font.pixelSize: Look.tTool
    font.weight: chosen ? Font.DemiBold : Font.Medium
    icon.width: Look.icon
    icon.height: Look.icon
    icon.color: ink

    contentItem: IconLabel {
        spacing: control.spacing
        mirrored: control.mirrored
        display: control.display
        icon: control.icon
        text: control.text
        font: control.font
        color: control.ink
    }

    background: Rectangle {
        implicitWidth: Look.toolHeight
        implicitHeight: Look.toolHeight
        radius: Look.radiusSmall
        color: {
            if (!control.enabled)
                return control.chosen ? Look.nightLine : "transparent";
            if (control.chosen)
                return control.down ? Look.mist : Look.frost;
            return control.down ? Look.night : control.hovered ? Look.nightLine : "transparent";
        }

        FocusRing {
            corner: Look.radiusSmall
            visible: control.visualFocus
        }
    }
}
