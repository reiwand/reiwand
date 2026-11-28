import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

SceneGrid {
    id: aspect

    required property var tools
    readonly property var info: tools.info?.sprite ?? ({
            "aspect": 1,
            "side": 0,
            "width": 0,
            "height": 0,
            "cover": 1,
            "texture": null
        })
    readonly property real least: 0.05
    readonly property real most: 20

    function setAspect(number, gesture) {
        tools.edit("aspect", {
                "aspect": Math.min(most, Math.max(least, Math.round(number * 100) / 100))
            }, gesture);
    }


    QQC2.Label {
        Layout.columnSpan: 2
        text: App.tr("Proportion of the sprite")
        font.bold: true
    }

    QQC2.Label {
        text: App.tr("Width over height")
    }

    SceneSlider {
        objectName: "aspectSlider"
        Layout.fillWidth: true
        editor: aspect.tools.page.editor
        from: Math.log(aspect.least)
        to: Math.log(aspect.most)
        value: Math.log(aspect.info.aspect)
        QQC2.ToolTip.visible: hovered
        QQC2.ToolTip.text: Math.round(aspect.info.aspect * 100) / 100
        Accessible.name: App.tr("Width over height")
        onMoved: aspect.setAspect(Math.exp(value), true)
    }

    SceneRowFit {
        Layout.columnSpan: 2
        Layout.fillWidth: true

        QQC2.Button {
            objectName: "aspectTake"
            enabled: aspect.info.texture !== null
            text: App.tr("Take from the texture")
            onClicked: aspect.tools.edit("aspect", {
                "from_texture": true
            })
        }

        QQC2.Button {
            objectName: "aspectSquare"
            text: App.tr("Square")
            onClicked: aspect.setAspect(1, false)
        }
    }

    Item {
        id: preview

        readonly property real unit: Math.min(width, height) * 0.8 / Math.max(aspect.info.side, 1)

        Layout.columnSpan: 2
        Layout.fillWidth: true
        implicitHeight: Kirigami.Units.gridUnit * 5

        Rectangle {
            anchors.centerIn: parent
            width: aspect.info.side * preview.unit
            height: width
            color: "transparent"
            border.color: Kirigami.Theme.disabledTextColor
        }

        Rectangle {
            objectName: "aspectSprite"
            anchors.centerIn: parent
            width: Math.max(aspect.info.width * preview.unit, 1)
            height: Math.max(aspect.info.height * preview.unit, 1)
            color: Kirigami.Theme.highlightColor
            opacity: 0.8
        }

        QQC2.Label {
            anchors.left: parent.left
            anchors.top: parent.top
            anchors.margins: Kirigami.Units.smallSpacing
            text: App.tr("%1 by %2 px, counts %3% of the square", Math.round(aspect.info.width * 10) / 10, Math.round(aspect.info.height * 10) / 10, Math.round(aspect.info.cover * 100))
        }
    }
}
