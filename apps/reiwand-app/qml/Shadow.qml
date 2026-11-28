import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Loader {
    id: caster

    property real corner: Look.radius
    property string kind: "cover"
    readonly property bool deep: GraphicsInfo.api !== GraphicsInfo.Software && GraphicsInfo.api !== GraphicsInfo.Unknown
    readonly property var layers: ({
            "cover": [[18, 40, 0.55, Look.night], [4, 10, 0.4, Look.night]],
            "focus": [[26, 60, 0.7, Look.night], [6, 14, 0.5, Look.night]],
            "panel": [[24, 64, 0.45, "black"], [4, 12, 0.3, "black"]],
            "row": [[10, 28, 0.35, "black"]]
        })[kind] ?? []

    anchors.fill: parent
    z: -1
    active: deep

    sourceComponent: Item {
        Repeater {
            model: caster.layers

            Kirigami.ShadowedRectangle {
                required property var modelData
                readonly property color tone: modelData[3]

                anchors.fill: parent
                radius: caster.corner
                color: "transparent"
                shadow.xOffset: 0
                shadow.yOffset: modelData[0]
                shadow.size: modelData[1]
                shadow.color: Qt.rgba(tone.r, tone.g, tone.b, modelData[2])
            }
        }
    }
}
