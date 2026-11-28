import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.reiwand.app

T.TextArea {
    id: control

    // Its focus ring is drawn over the neighbours that touch it.
    z: activeFocus ? 1 : 0
    implicitWidth: Math.max(contentWidth + leftPadding + rightPadding, implicitBackgroundWidth + leftInset + rightInset, placeholder.implicitWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(contentHeight + topPadding + bottomPadding, implicitBackgroundHeight + topInset + bottomInset, placeholder.implicitHeight + topPadding + bottomPadding)
    topPadding: Look.e2
    bottomPadding: Look.e2
    leftPadding: Look.e3
    rightPadding: Look.e3
    font.family: Look.textFont
    font.pixelSize: Look.tBody
    color: enabled ? Look.frost : Look.mistLow
    selectionColor: Look.moon
    selectedTextColor: Look.night
    placeholderTextColor: enabled ? Look.mist : Look.mistLow

    PlaceholderText {
        id: placeholder

        x: control.leftPadding
        y: control.topPadding
        width: control.width - (control.leftPadding + control.rightPadding)
        height: control.height - (control.topPadding + control.bottomPadding)
        text: control.placeholderText
        font: control.font
        color: control.placeholderTextColor
        verticalAlignment: control.verticalAlignment
        visible: !control.length && !control.preeditText && (!control.activeFocus || control.horizontalAlignment !== Qt.AlignHCenter)
        wrapMode: control.wrapMode
        elide: Text.ElideRight
        renderType: control.renderType
    }

    background: Field {
        lit: control.activeFocus && !control.readOnly
        over: control.hovered
        live: control.enabled && !control.readOnly
    }
}
