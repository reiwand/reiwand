import QtQuick
import io.github.reiwand.app

Item {
    id: dim

    required property var preview
    property real shade: 1
    property alias panel: panel

    focus: true
    Keys.onPressed: event => event.accepted = preview.press(event.key)

    Rectangle {
        anchors.fill: parent
        color: Qt.rgba(0, 0, 0, PreviewLook.velo * dim.shade)
    }

    MouseArea {
        anchors.fill: parent
        onClicked: dim.preview.press(Qt.Key_Escape)
    }

    WheelHandler {
        acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
        onWheel: event => dim.preview.scroll(event.angleDelta.y)
    }

    PreviewPanel {
        id: panel

        anchors.centerIn: parent
        anchors.verticalCenterOffset: Math.round((1 - dim.shade) * 14 * dim.preview.unit)
        opacity: dim.shade
        visible: dim.preview.began
        preview: dim.preview

        MouseArea {
            anchors.fill: parent
            z: -1
        }
    }
}
