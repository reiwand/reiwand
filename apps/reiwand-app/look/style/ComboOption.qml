import QtQuick
import QtQuick.Templates as T
import io.github.reiwand.app

T.ItemDelegate {
    id: control

    property bool chosen: false

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    leftPadding: Look.e3
    rightPadding: Look.e3
    spacing: Look.e3
    font.family: Look.letraTexto
    font.pixelSize: Look.tCuerpo
    font.weight: highlighted || chosen ? Font.DemiBold : Font.Normal
    T.ToolTip.visible: hovered && label.truncated
    T.ToolTip.text: text
    T.ToolTip.delay: Qt.styleHints.mousePressAndHoldInterval

    contentItem: Text {
        id: label

        text: control.text
        font: control.font
        elide: Text.ElideMiddle
        verticalAlignment: Text.AlignVCenter
        color: control.highlighted ? Look.noche : control.enabled ? Look.escarcha : Look.brumaBaja
    }

    background: Option {
        lit: control.highlighted
        over: control.hovered
        down: control.down
    }
}
