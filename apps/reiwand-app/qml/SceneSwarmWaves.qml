import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

ColumnLayout {
    id: waves

    required property var tools
    readonly property var list: tools.info?.waves ?? []
    readonly property var whats: ["alpha", "size"]
    readonly property var bases: ["top", "floor"]
    readonly property var fresh: ({
            "oscillate": {
                "what": "alpha",
                "from": "floor",
                "freq": [0.3, 0.3],
                "scale": [0.2, 1],
                "phase": [0, 1]
            }
        })

    function force(index) {
        return JSON.parse(tools.page.editor.value(SceneSwarmKnobs.pointer(tools.system, "/forces/" + index)) || "null");
    }

    function patch(wave, change, gesture) {
        const v = force(wave.index);
        change(v.oscillate);
        tools.edit("force", {
                "index": wave.index,
                "force": v
            }, gesture);
    }

    SceneRowFit {
        PanelHeading {
            Layout.fillWidth: true
            level: 4
            text: App.tr("Fades and pulses")
        }

        QQC2.Button {
            objectName: "waveAdd"
            text: App.tr("Add a pulse")
            onClicked: waves.tools.edit("force", {
                "force": waves.fresh
            })
        }
    }

    SceneRepeater {
        id: listed

        rows: waves.list

        delegate: ColumnLayout {
            id: row

            readonly property var modelData: listed.rows[index]
            required property int index

            Layout.fillWidth: true

            SceneRowFit {
                QQC2.ComboBox {
                    objectName: "waveWhat"
                    model: [App.tr("Opacity"), App.tr("Size")]
                    currentIndex: Math.max(0, waves.whats.indexOf(row.modelData.what))
                    onActivated: i => waves.patch(row.modelData, o => o.what = waves.whats[i])
                }

                QQC2.ComboBox {
                    objectName: "waveFrom"
                    model: [App.tr("Dive from the top"), App.tr("Rise from the floor")]
                    currentIndex: Math.max(0, waves.bases.indexOf(row.modelData.from))
                    onActivated: i => waves.patch(row.modelData, o => o.from = waves.bases[i])
                }

                QQC2.CheckBox {
                    objectName: "waveShared"
                    text: App.tr("One random number for speed, size and phase")
                    checked: row.modelData.shared
                    onToggled: waves.patch(row.modelData, o => {
                        if (checked)
                            o.random = "shared";
                        else
                            delete o.random;
                    })
                }
            }

            SceneRangeSlider {
                objectName: "waveScale"
                Layout.fillWidth: true
                editor: waves.tools.page.editor
                from: 0
                to: 1
                first.value: row.modelData.scale[0]
                second.value: row.modelData.scale[1]
                first.onMoved: waves.patch(row.modelData, o => o.scale = [first.value, Math.max(first.value, second.value)].map(v => Math.round(v * 100) / 100), true)
                second.onMoved: waves.patch(row.modelData, o => o.scale = [Math.min(first.value, second.value), second.value].map(v => Math.round(v * 100) / 100), true)
            }

            SceneSwarmWavePlot {
                Layout.fillWidth: true
                wave: row.modelData
            }
        }
    }
}
