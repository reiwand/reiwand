import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.reiwand.app

T.SpinBox {
    id: control

    // Its focus ring is drawn over the neighbours that touch it.
    z: activeFocus ? 1 : 0
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, contentItem.implicitWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding, up.implicitIndicatorHeight, down.implicitIndicatorHeight)
    padding: 0
    leftPadding: control.mirrored ? (up.indicator ? up.indicator.width : 0) : (down.indicator ? down.indicator.width : 0)
    rightPadding: control.mirrored ? (down.indicator ? down.indicator.width : 0) : (up.indicator ? up.indicator.width : 0)
    font.family: Look.textFont
    font.pixelSize: Look.tBody

    validator: IntValidator {
        locale: control.locale.name
        bottom: Math.min(control.from, control.to)
        top: Math.max(control.from, control.to)
    }

    contentItem: TextInput {
        z: 2
        text: control.displayText
        clip: width < implicitWidth
        leftPadding: Look.e1
        rightPadding: Look.e1
        font: control.font
        color: control.enabled ? Look.frost : Look.mistLow
        selectionColor: Look.moon
        selectedTextColor: Look.night
        horizontalAlignment: Qt.AlignHCenter
        verticalAlignment: Qt.AlignVCenter
        readOnly: !control.editable
        validator: control.validator
        inputMethodHints: control.inputMethodHints
    }

    up.indicator: SpinStep {
        x: control.mirrored ? 0 : control.width - width
        height: control.height
        name: control.mirrored ? "caret-left" : "caret-right"
        held: control.up.pressed
        over: control.up.hovered
        live: control.enabled && control.value < Math.max(control.from, control.to)
    }

    down.indicator: SpinStep {
        x: control.mirrored ? control.width - width : 0
        height: control.height
        name: control.mirrored ? "caret-right" : "caret-left"
        held: control.down.pressed
        over: control.down.hovered
        live: control.enabled && control.value > Math.min(control.from, control.to)
    }

    background: Field {
        implicitWidth: 140
        lit: control.activeFocus
        over: control.hovered
        live: control.enabled
    }
}
