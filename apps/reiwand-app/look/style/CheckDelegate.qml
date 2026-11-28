import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.reiwand.app

T.CheckDelegate {
    id: control

    // Its focus ring is drawn over the neighbours that touch it.
    z: visualFocus ? 1 : 0
    readonly property bool lit: checkState !== Qt.Unchecked

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding, implicitIndicatorHeight + topPadding + bottomPadding)
    leftPadding: Look.e3
    rightPadding: Look.e3
    spacing: Look.e3
    font.pixelSize: Look.tBody
    icon.width: Look.icon
    icon.height: Look.icon
    icon.color: enabled ? Look.mist : Look.mistLow

    contentItem: IconLabel {
        leftPadding: control.mirrored ? control.indicator.width + control.spacing : 0
        rightPadding: !control.mirrored ? control.indicator.width + control.spacing : 0
        spacing: control.spacing
        mirrored: control.mirrored
        display: control.display
        alignment: Qt.AlignLeft | Qt.AlignVCenter
        icon: control.icon
        text: control.text
        font: control.font
        color: control.enabled ? Look.frost : Look.mistLow
    }

    indicator: Rectangle {
        x: control.mirrored ? control.leftPadding : control.width - width - control.rightPadding
        y: control.topPadding + (control.availableHeight - height) / 2
        implicitWidth: Look.mark
        implicitHeight: Look.mark
        radius: Look.radiusSmall
        color: control.lit ? (control.enabled ? Look.frost : Look.mist) : Look.nightLine

        IconImage {
            anchors.centerIn: parent
            width: Look.mark - Look.e1
            height: Look.mark - Look.e1
            sourceSize: Qt.size(width, height)
            visible: control.lit
            name: control.checkState === Qt.PartiallyChecked ? "minus-bold" : "check-bold"
            color: Look.night
        }
    }

    background: Rectangle {
        implicitWidth: 200
        implicitHeight: Look.optionHeight
        radius: Look.radiusSmall
        visible: control.down || control.hovered || control.visualFocus || control.highlighted
        color: control.down ? Look.night : Look.nightHigh

        FocusRing {
            corner: parent.radius
            visible: control.visualFocus
        }
    }
}
