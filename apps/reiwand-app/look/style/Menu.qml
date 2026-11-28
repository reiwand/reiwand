import QtQuick
import QtQuick.Templates as T
import io.github.reiwand.app

T.Menu {
    id: control

    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    margins: 0
    overlap: 6
    padding: 6
    font.family: Look.textFont
    font.pixelSize: Look.tBody

    delegate: MenuItem {
    }

    contentItem: ListView {
        implicitHeight: contentHeight
        model: control.contentModel
        interactive: Window.window ? contentHeight + control.topPadding + control.bottomPadding > control.height : false
        clip: true
        currentIndex: control.currentIndex

        T.ScrollIndicator.vertical: ScrollIndicator {
        }
    }

    background: Panel {
        implicitWidth: 220
    }

    T.Overlay.modal: Rectangle {
        color: Look.veilColor
    }

    T.Overlay.modeless: Rectangle {
        color: "transparent"
    }
}
