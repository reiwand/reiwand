import QtQuick
import QtQuick.Controls.impl
import io.github.reiwand.app

Item {
    id: glyph

    property string icon

    clip: true

    Rectangle {
        anchors.fill: parent
        radius: Look.radius

        gradient: Gradient {
            GradientStop {
                position: 0
                color: Look.nightLine
            }

            GradientStop {
                position: 0.55
                color: Look.nightHigh
            }

            GradientStop {
                position: 1
                color: Look.night
            }
        }
    }

    Repeater {
        model: 3

        Rectangle {
            required property int index
            readonly property real side: glyph.height * (0.7 + index * 0.45)

            x: glyph.width * 0.78 - side / 2
            y: glyph.height * 0.22 - side / 2
            width: side
            height: side
            radius: side / 2
            color: "transparent"
            border.width: Look.inset
            border.color: index === 0 ? Look.moonDeep : Look.nightLine
        }
    }

    IconImage {
        anchors.centerIn: parent
        name: glyph.icon
        sourceSize: Qt.size(Look.icon * 2, Look.icon * 2)
        color: Look.moon
    }
}
