import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

FocusScope {
    id: panel

    required property var page
    readonly property alias list: scripts

    signal edit(string path)

    QQC2.ScrollView {
        id: scroll

        anchors.fill: parent
        anchors.margins: Look.e2
        contentWidth: availableWidth

        ScrollColumn {
            view: scroll
            spacing: Look.e2

            SceneFold {
                Layout.fillWidth: true
                key: "scripts"
                title: scripts.scripts.length > 0 ? App.tr("Scene scripts · %1", scripts.scripts.length) : App.tr("Scene scripts")

                QQC2.Label {
                    Layout.fillWidth: true
                    visible: scripts.scripts.length === 0
                    text: App.tr("No script moves this scene yet. A script reads the scene's values and inputs and moves its layers.")
                    color: Look.mist
                    wrapMode: Text.Wrap
                }

                SceneScriptsList {
                    id: scripts

                    Layout.fillWidth: true
                    page: panel.page
                    onEdit: path => panel.edit(path)
                }

                SceneScriptAdd {
                    Layout.fillWidth: true
                    page: panel.page
                }
            }

            SceneUses {
                Layout.fillWidth: true
                page: panel.page
            }

            SceneFold {
                Layout.fillWidth: true
                key: "templates"
                title: tree.templates.length > 0 ? App.tr("scene.spawn templates · %1", tree.templates.length) : App.tr("scene.spawn templates")

                QQC2.Label {
                    Layout.fillWidth: true
                    visible: tree.templates.length === 0
                    text: App.tr("A layer becomes a template with “Make a template” in its menu.")
                    color: Look.mist
                    wrapMode: Text.Wrap
                }

                SceneTemplateTree {
                    id: tree

                    Layout.fillWidth: true
                    page: panel.page
                }
            }
        }
    }
}
