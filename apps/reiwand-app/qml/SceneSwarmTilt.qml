import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

GridLayout {
    id: tilt

    required property var tools
    readonly property var init: {
        tools.info;
        return JSON.parse(tools.page.editor.value(SceneSwarmKnobs.pointer(tools.system, "/init")) || "null") ?? {};
    }
    readonly property var value: init.tilt ?? null
    readonly property var rows: [["angle", 0, App.tr("Turn about the horizontal line"), 180], ["angle", 1, App.tr("Turn about the vertical line"), 180], ["spin", 0, App.tr("Spin about the horizontal line, per second"), 720], ["spin", 1, App.tr("Spin about the vertical line, per second"), 720]]

    function put(group, which, end, number) {
        const next = JSON.parse(JSON.stringify(value));
        next[group][end][which] = number;
        if (end === 0)
            next[group][1][which] = Math.max(next[group][1][which], number);
        else
            next[group][0][which] = Math.min(next[group][0][which], number);
        tools.edit("start", {
                "ranges": {
                    "tilt": next
                }
            }, true);
    }

    function across(angle) {
        return Math.abs(Math.cos(angle * Math.PI / 180));
    }

    columns: 2

    QQC2.CheckBox {
        objectName: "tiltOn"
        Layout.columnSpan: 2
        text: App.tr("Turn sprites in depth")
        checked: tilt.value !== null
        onToggled: tilt.tools.edit("start", checked ? {
            "ranges": {
                "tilt": {
                    "angle": [[0, 0], [0, 0]],
                    "spin": [[0, 0], [0, 0]]
                }
            }
        } : {
            "clear": ["tilt"]
        })
    }

    Repeater {
        model: tilt.value === null ? [] : tilt.rows

        delegate: ColumnLayout {
            id: row

            required property var modelData

            Layout.columnSpan: 2
            Layout.fillWidth: true

            QQC2.Label {
                text: row.modelData[2]
            }

            Repeater {
                model: [0, 1]

                delegate: SceneSlider {
                    id: slider

                    required property int modelData

                    objectName: "tilt_" + row.modelData[0] + row.modelData[1] + slider.modelData
                    Layout.fillWidth: true
                    editor: tilt.tools.page.editor
                    from: -row.modelData[3]
                    to: row.modelData[3]
                    value: tilt.value[row.modelData[0]][slider.modelData][row.modelData[1]]
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: (slider.modelData === 0 ? App.tr("From %1", Math.round(value)) : App.tr("To %1", Math.round(value)))
                    Accessible.name: row.modelData[2]
                    onMoved: tilt.put(row.modelData[0], row.modelData[1], slider.modelData, Math.round(value))
                }
            }
        }
    }

    RowLayout {
        id: preview

        Layout.columnSpan: 2
        Layout.fillWidth: true
        visible: tilt.value !== null
        implicitHeight: Kirigami.Units.gridUnit * 4

        Repeater {
            model: [0, 0.25, 0.5, 0.75, 1]

            delegate: Item {
                id: step

                required property real modelData
                readonly property var ends: tilt.value?.angle ?? [[0, 0], [0, 0]]

                Layout.fillWidth: true
                Layout.fillHeight: true

                Rectangle {
                    objectName: "tiltSprite" + step.modelData
                    readonly property real full: Math.min(step.width, step.height) * 0.8

                    anchors.centerIn: parent
                    width: Math.max(full * tilt.across(step.ends[0][1] + (step.ends[1][1] - step.ends[0][1]) * step.modelData), 1)
                    height: Math.max(full * tilt.across(step.ends[0][0] + (step.ends[1][0] - step.ends[0][0]) * step.modelData), 1)
                    color: Kirigami.Theme.highlightColor
                    opacity: 0.8
                }
            }
        }
    }
}
