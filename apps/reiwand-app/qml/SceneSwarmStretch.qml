import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

GridLayout {
    id: stretch

    required property var tools
    readonly property var info: tools.info?.stretch ?? null
    readonly property bool relative: info?.mode === "relative"
    readonly property var modes: ["add", "relative"]
    readonly property var fresh: ({
            "add": {
                "mode": "add",
                "time": 0.05,
                "min": 0,
                "max": 60
            },
            "relative": {
                "mode": "relative",
                "time": 0.05,
                "min": 0,
                "max": 10
            }
        })
    readonly property real reach: relative ? 10 : 200

    function put(change, gesture) {
        const value = Object.assign({
                "mode": info.mode,
                "time": info.time,
                "min": info.min,
                "max": info.max
            }, change);
        value.max = Math.max(value.min, value.max);
        tools.edit("render", {
                "stretch": value
            }, gesture);
    }

    columns: 2

    QQC2.CheckBox {
        objectName: "stretchOn"
        Layout.columnSpan: 2
        text: App.tr("Stretch by speed")
        checked: stretch.info !== null
        onToggled: stretch.tools.edit("render", {
            "stretch": checked ? stretch.fresh.add : "none"
        })
    }

    QQC2.ComboBox {
        objectName: "stretchMode"
        Layout.columnSpan: 2
        Layout.fillWidth: true
        visible: stretch.info !== null
        model: [App.tr("Add pixels to the side"), App.tr("Multiply the side")]
        currentIndex: stretch.relative ? 1 : 0
        onActivated: index => stretch.tools.edit("render", {
                "stretch": Object.assign({}, stretch.fresh[stretch.modes[index]], {
                    "time": stretch.info.time
                })
            })
    }

    Repeater {
        model: stretch.info === null ? [] : [
            {
                "key": "time",
                "name": App.tr("Seconds of motion"),
                "to": 1
            },
            {
                "key": "min",
                "name": stretch.relative ? App.tr("Shortest, in sides") : App.tr("Shortest, in pixels"),
                "to": stretch.reach
            },
            {
                "key": "max",
                "name": stretch.relative ? App.tr("Longest, in sides") : App.tr("Longest, in pixels"),
                "to": stretch.reach
            }
        ]

        delegate: RowLayout {
            id: row

            required property var modelData

            Layout.columnSpan: 2
            Layout.fillWidth: true

            QQC2.Label {
                Layout.preferredWidth: Kirigami.Units.gridUnit * 8
                text: row.modelData.name
            }

            SceneSlider {
                objectName: "stretch_" + row.modelData.key
                Layout.fillWidth: true
                editor: stretch.tools.page.editor
                from: 0
                to: row.modelData.to
                value: (stretch.info ?? ({}))[row.modelData.key] ?? 0
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: Math.round(value * 100) / 100
                Accessible.name: row.modelData.name
                onMoved: stretch.put({
                    [row.modelData.key]: Math.round(value * 100) / 100
                }, true)
            }
        }
    }

    Item {
        id: preview

        readonly property real side: stretch.info?.side ?? 0
        readonly property real span: stretch.info?.long ?? 0
        readonly property real unit: Math.min(width * 0.9 / Math.max(span, side, 1), height * 0.8 / Math.max(side, 1))

        Layout.columnSpan: 2
        Layout.fillWidth: true
        visible: stretch.info !== null
        implicitHeight: Kirigami.Units.gridUnit * 4

        Rectangle {
            anchors.fill: parent
            color: "transparent"
            border.color: Kirigami.Theme.disabledTextColor
        }

        Rectangle {
            width: preview.side * preview.unit
            height: width
            x: parent.width * 0.95 - width
            anchors.verticalCenter: parent.verticalCenter
            color: "transparent"
            border.color: Kirigami.Theme.disabledTextColor
        }

        Rectangle {
            objectName: "stretchSprite"
            width: preview.span * preview.unit
            height: preview.side * preview.unit
            x: parent.width * 0.95 - width
            anchors.verticalCenter: parent.verticalCenter
            radius: height / 2
            color: Kirigami.Theme.highlightColor
            opacity: 0.8
        }

        QQC2.Label {
            anchors.left: parent.left
            anchors.top: parent.top
            anchors.margins: Kirigami.Units.smallSpacing
            text: App.tr("%1 px long at %2 px/s, side %3 px", Math.round(preview.span * 10) / 10, Math.round(stretch.info?.speed ?? 0), Math.round(preview.side * 10) / 10)
        }
    }
}
