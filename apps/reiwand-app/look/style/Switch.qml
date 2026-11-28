import QtQuick
import QtQuick.Templates as T
import io.github.reiwand.app

T.Switch {
    id: control

    // Its focus ring is drawn over the neighbours that touch it.
    z: visualFocus ? 1 : 0
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding, implicitIndicatorHeight + topPadding + bottomPadding)
    padding: Look.e1
    spacing: Look.e3
    font.pixelSize: Look.tBody

    indicator: Rectangle {
        x: control.text ? (control.mirrored ? control.width - width - control.rightPadding : control.leftPadding) : control.leftPadding + (control.availableWidth - width) / 2
        y: control.topPadding + (control.availableHeight - height) / 2
        implicitWidth: Look.switchWidth
        implicitHeight: Look.switchHeight
        radius: height / 2
        color: control.checked ? (control.enabled ? Look.frost : Look.mist) : Look.nightLine

        Rectangle {
            readonly property real gap: (parent.height - height) / 2

            x: control.checked ? parent.width - width - gap : gap
            y: gap
            width: Look.switchDot
            height: Look.switchDot
            radius: height / 2
            color: control.checked ? Look.night : control.enabled ? Look.mist : Look.mistLow

            Behavior on x {
                enabled: control.visible
                NumberAnimation {
                    duration: Look.pace
                    easing.type: Easing.Bezier
                    easing.bezierCurve: Look.paceCurve
                }
            }
        }

        FocusRing {
            corner: parent.radius
            visible: control.visualFocus
        }
    }

    contentItem: Text {
        leftPadding: control.indicator && !control.mirrored ? control.indicator.width + control.spacing : 0
        rightPadding: control.indicator && control.mirrored ? control.indicator.width + control.spacing : 0
        text: control.text
        font: control.font
        color: control.enabled ? Look.frost : Look.mistLow
        elide: Text.ElideRight
        verticalAlignment: Text.AlignVCenter
    }
}
