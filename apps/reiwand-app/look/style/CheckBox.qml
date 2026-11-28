import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.reiwand.app

T.CheckBox {
    id: control

    // Its focus ring is drawn over the neighbours that touch it.
    z: visualFocus ? 1 : 0
    readonly property bool lit: checkState !== Qt.Unchecked

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding, implicitIndicatorHeight + topPadding + bottomPadding)
    padding: Look.e1
    spacing: Look.e3
    font.pixelSize: Look.tBody

    indicator: Rectangle {
        x: control.text ? (control.mirrored ? control.width - width - control.rightPadding : control.leftPadding) : control.leftPadding + (control.availableWidth - width) / 2
        y: control.topPadding + (control.availableHeight - height) / 2
        implicitWidth: Look.mark
        implicitHeight: Look.mark
        radius: Look.radiusSmall
        color: control.lit ? (control.enabled ? Look.frost : Look.mist) : control.down ? Look.night : Look.nightLine
        border.width: control.lit ? 0 : Look.line
        border.color: control.enabled && control.hovered ? Look.mist : Look.nightLine

        IconImage {
            anchors.centerIn: parent
            width: Look.mark - Look.e1
            height: Look.mark - Look.e1
            sourceSize: Qt.size(width, height)
            visible: control.lit
            name: control.checkState === Qt.PartiallyChecked ? "minus-bold" : "check-bold"
            color: Look.night
        }

        FocusRing {
            corner: parent.radius
            visible: control.visualFocus
        }
    }

    contentItem: CheckText {
        control: control
    }
}
