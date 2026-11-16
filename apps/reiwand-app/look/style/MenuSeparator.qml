import QtQuick
import QtQuick.Templates as T
import io.github.reiwand.app

T.MenuSeparator {
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    topPadding: 6
    bottomPadding: 6

    contentItem: Rectangle {
        implicitWidth: 188
        implicitHeight: Look.linea
        color: Look.nocheLinea
    }
}
