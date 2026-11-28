import QtQuick
import QtQuick.Templates as T
import "../look"

T.Slider {
    id: level

    implicitWidth: Look.railWidth
    implicitHeight: Look.optionHeight
    focusPolicy: Qt.StrongFocus

    background: Rectangle {
        x: level.leftPadding
        y: level.topPadding + (level.availableHeight - height) / 2
        width: level.availableWidth
        height: Look.railHeight
        radius: height / 2
        color: Look.nightLine

        Rectangle {
            width: level.visualPosition * parent.width
            height: parent.height
            radius: height / 2
            color: Look.frost
        }
    }

    handle: Rectangle {
        x: level.leftPadding + level.visualPosition * (level.availableWidth - width)
        y: level.topPadding + (level.availableHeight - height) / 2
        width: Look.railDot
        height: Look.railDot
        radius: width / 2
        color: Look.frost

        Ring {
            visible: level.visualFocus
            corner: width / 2
        }
    }
}
