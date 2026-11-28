import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

ColumnLayout {
    id: noise

    required property var tools
    readonly property var list: tools.info?.noises ?? []
    readonly property var axes: [App.tr("Across"), App.tr("Down"), App.tr("Depth")]

    function force(index) {
        return JSON.parse(tools.page.editor.value(SceneSwarmKnobs.pointer(tools.system, "/forces/" + index)) || "null");
    }

    function patch(index, change, gesture) {
        const v = force(index);
        change(v.noise_field);
        tools.edit("force", {
                "index": index,
                "force": v
            }, gesture);
    }

    function round(v, step) {
        return Math.round(v / step) * step;
    }

    RowLayout {
        Kirigami.Heading {
            Layout.fillWidth: true
            level: 4
            text: App.tr("Noise fields")
        }

        QQC2.Button {
            objectName: "noiseAdd"
            text: App.tr("Add a noise field")
            QQC2.ToolTip.visible: hovered
            QQC2.ToolTip.text: App.tr("Pushes each particle along a drifting 3D noise, at a strength of its own, so particles gather and part")
            onClicked: noise.tools.edit("force", {
                    "force": {
                        "noise_field": App.fill("Offer.noise_field")
                    }
                })
        }
    }

    SceneRepeater {
        id: listed

        rows: noise.list

        delegate: GridLayout {
            id: row

            readonly property var modelData: listed.rows[index]
            required property int index

            Layout.fillWidth: true
            columns: 2

            QQC2.Label {
                text: App.tr("Strength")
            }

            SceneRangeSlider {
                objectName: "noiseStrength"
                Layout.fillWidth: true
                editor: noise.tools.page.editor
                from: -400
                to: 400
                first.value: row.modelData.strength.min
                second.value: row.modelData.strength.max
                first.onMoved: noise.patch(row.modelData.index, n => n.strength = {
                        "min": Math.round(first.value),
                        "max": Math.round(second.value)
                    }, true)
                second.onMoved: noise.patch(row.modelData.index, n => n.strength = {
                        "min": Math.round(first.value),
                        "max": Math.round(second.value)
                    }, true)
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: App.tr("Pixels per second each second, from %1 to %2", row.modelData.strength.min, row.modelData.strength.max)
            }

            QQC2.Label {
                text: App.tr("Phase")
            }

            SceneRangeSlider {
                objectName: "noisePhase"
                Layout.fillWidth: true
                editor: noise.tools.page.editor
                from: 0
                to: 2000
                first.value: row.modelData.phase.min
                second.value: row.modelData.phase.max
                first.onMoved: noise.patch(row.modelData.index, n => n.phase = {
                        "min": Math.round(first.value),
                        "max": Math.round(second.value)
                    }, true)
                second.onMoved: noise.patch(row.modelData.index, n => n.phase = {
                        "min": Math.round(first.value),
                        "max": Math.round(second.value)
                    }, true)
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: App.tr("Pixels each particle reads the noise ahead of where it is, picked with its strength")
            }

            QQC2.Label {
                text: App.tr("Size of the swirls")
            }

            SceneSlider {
                objectName: "noiseScale"
                Layout.fillWidth: true
                editor: noise.tools.page.editor
                from: Math.log(0.0005)
                to: Math.log(0.1)
                value: Math.log(row.modelData.scale)
                onMoved: noise.patch(row.modelData.index, n => n.scale = Number(Math.exp(value).toPrecision(2)), true)
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: App.tr("Swirls about %1 pixels across", Math.round(1 / row.modelData.scale))
            }

            QQC2.Label {
                text: App.tr("Drift")
            }

            SceneSlider {
                objectName: "noiseDrift"
                Layout.fillWidth: true
                editor: noise.tools.page.editor
                from: -200
                to: 200
                value: row.modelData.drift
                onMoved: noise.patch(row.modelData.index, n => n.drift = Math.round(value), true)
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: App.tr("The noise slides %1 pixels a second along every axis", row.modelData.drift)
            }

            QQC2.Label {
                text: App.tr("Axes")
            }

            RowLayout {
                Repeater {
                    model: 3

                    delegate: SceneSlider {
                        required property int index

                        objectName: "noiseMask" + index
                        Layout.fillWidth: true
                        editor: noise.tools.page.editor
                        from: 0
                        to: 1
                        value: row.modelData.mask[index]
                        onMoved: noise.patch(row.modelData.index, n => n.mask[index] = noise.round(value, 0.05), true)
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: App.tr("%1: %2%", noise.axes[index], Math.round(row.modelData.mask[index] * 100))
                    }
                }
            }

            QQC2.Button {
                Layout.columnSpan: 2
                icon.name: "edit-delete"
                text: App.tr("Remove the noise field")
                onClicked: noise.tools.edit("force", {
                        "index": row.modelData.index,
                        "remove": true
                    })
            }
        }
    }
}
