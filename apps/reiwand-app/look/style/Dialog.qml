import QtQuick
import QtQuick.Templates as T
import io.github.reiwand.app

T.Dialog {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding, implicitHeaderWidth, implicitFooterWidth)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding + (implicitHeaderHeight > 0 ? implicitHeaderHeight + spacing : 0) + (implicitFooterHeight > 0 ? implicitFooterHeight + spacing : 0))
    padding: Look.e4
    font.family: Look.textFont
    font.pixelSize: Look.tBody

    background: Panel {
    }

    header: T.Label {
        text: control.title
        visible: control.title
        elide: Text.ElideRight
        padding: Look.e4
        bottomPadding: 0
        font.family: Look.titleFont
        font.pixelSize: Look.tRow
        font.weight: Font.Bold
        color: Look.frost
    }

    footer: DialogButtonBox {
        visible: count > 0
    }

    T.Overlay.modal: Rectangle {
        color: Look.veilColor
    }

    T.Overlay.modeless: Rectangle {
        color: "transparent"
    }
}
