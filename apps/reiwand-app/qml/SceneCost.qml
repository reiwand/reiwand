import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The Cost panel: what the scene costs to draw, gathered from where each cost was shown before.
// Scene: the quality levels and the scene's budget; Particles: the budget of each system; Lights:
// the lights and lit layers per level; 3D: the models; Effects: the effects of the layer in hand.
// The number of a single layer stays in the Inspector.
ColumnLayout {
    id: cost

    required property var page
    readonly property string held: (page.view.layers ?? []).find(item => item.pointer === page.editor.selected)?.pointer ?? ""
    readonly property var systems: {
        page.view;
        return Object.keys(JSON.parse(page.editor.value("/scene/particles") || "null") ?? {});
    }
    readonly property var effects: {
        page.view;
        return held ? JSON.parse(page.editor.value(held + "/effects") || "[]") ?? [] : [];
    }

    spacing: 0

    QQC2.TabBar {
        id: tabs

        // The tabs scroll in a narrow panel instead of widening it.
        Layout.fillWidth: true
        Layout.minimumWidth: 0
        Layout.preferredWidth: 0

        Repeater {
            model: [App.tr("Scene"), App.tr("Particles"), App.tr("Lights"), App.tr("3D"), App.tr("Effects")]

            delegate: QQC2.TabButton {
                required property string modelData

                text: modelData
                width: implicitWidth
            }
        }
    }

    StackLayout {
        Layout.fillWidth: true
        Layout.fillHeight: true
        currentIndex: tabs.currentIndex

        CostPage {
            SceneQuality {
                Layout.fillWidth: true
                page: cost.page
            }
        }

        CostPage {
            Repeater {
                model: cost.systems

                delegate: ColumnLayout {
                    id: one

                    required property string modelData
                    readonly property QtObject tools: QtObject {
                        readonly property var page: cost.page
                        readonly property string system: one.modelData
                    }

                    Layout.fillWidth: true

                    PanelHeading {
                        Layout.fillWidth: true
                        level: 4
                        text: one.modelData
                    }

                    SceneSwarmBudget {
                        Layout.fillWidth: true
                        tools: one.tools
                    }
                }
            }

            QQC2.Label {
                Layout.fillWidth: true
                visible: cost.systems.length === 0
                wrapMode: Text.Wrap
                color: Look.mist
                text: App.tr("The scene has no particle systems.")
            }
        }

        CostPage {
            SceneLightsPanel {
                id: lights

                Layout.fillWidth: true
                page: cost.page
            }
        }

        CostPage {
            SceneModelCost {
                Layout.fillWidth: true
                info: lights.cost?.models
            }

            QQC2.Label {
                Layout.fillWidth: true
                visible: (lights.cost?.models ?? []).length === 0
                wrapMode: Text.Wrap
                color: Look.mist
                text: App.tr("The scene has no 3D models.")
            }
        }

        CostPage {
            Repeater {
                model: cost.effects

                delegate: SceneEffectCost {
                    required property var modelData

                    Layout.fillWidth: true
                    page: cost.page
                    effect: String(modelData.effect ?? "")
                    optional: modelData.optional ?? false
                }
            }

            QQC2.Label {
                Layout.fillWidth: true
                visible: cost.effects.length === 0
                wrapMode: Text.Wrap
                color: Look.mist
                text: cost.held ? App.tr("This layer has no effects.") : App.tr("Choose a layer to see what its effects cost.")
            }
        }
    }

    // One tab of the panel: its pieces in a column that scrolls.
    component CostPage: QQC2.ScrollView {
        id: scroll

        default property alias pieces: column.data

        contentWidth: availableWidth
        clip: true

        ScrollColumn {
            id: column

            view: scroll
        }
    }
}
