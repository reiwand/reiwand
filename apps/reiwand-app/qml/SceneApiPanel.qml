import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

FocusScope {
    id: api

    required property var page
    property var parts: []
    readonly property alias find: find
    readonly property string sought: find.text.trim().toLowerCase()
    readonly property var shown: parts.map(part => Object.assign({}, part, {
                "items": part.items.filter(item => sought === "" || (item.name + " " + item.text).toLowerCase().includes(sought))
            })).filter(part => part.items.length > 0)
    readonly property var headings: ({
            "scene": App.tr("Scene and layers"),
            "inputs": App.tr("What the scene gives a script"),
            "tools": App.tr("Timers, storage and other tools"),
            "events": App.tr("Functions a module exports"),
            "values": App.tr("Functions a value follows")
        })

    signal inserted(string code)

    function load() {
        if (api.parts.length === 0)
            api.parts = JSON.parse(page.editor.code("reference", "{}") || "[]");
    }

    onVisibleChanged: if (visible)
        load()
    Component.onCompleted: if (visible)
        load()

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Look.e2
        spacing: Look.e2

        SearchBox {
            id: find

            Layout.fillWidth: true
            placeholder: App.tr("Search the API")
        }

        QQC2.ScrollView {
            id: scroll

            Layout.fillWidth: true
            Layout.fillHeight: true
            contentWidth: availableWidth

            ColumnLayout {
                width: scroll.availableWidth
                spacing: Kirigami.Units.smallSpacing

                QQC2.Label {
                    Layout.fillWidth: true
                    visible: api.shown.length === 0 && api.sought !== ""
                    text: App.tr("Nothing in the API matches “%1”.", find.text.trim())
                    color: Look.mist
                    wrapMode: Text.Wrap
                }

                Repeater {
                    model: api.shown

                    delegate: SceneFold {
                        id: part

                        required property var modelData

                        Layout.fillWidth: true
                        key: "api-" + part.modelData.part
                        title: api.headings[part.modelData.part] ?? part.modelData.part

                        Repeater {
                            model: part.modelData.items

                            delegate: ColumnLayout {
                                id: item

                                required property var modelData

                                Layout.fillWidth: true
                                spacing: 0

                                QQC2.Label {
                                    Layout.fillWidth: true
                                    text: item.modelData.name
                                    font.family: "monospace"
                                    font.bold: true
                                    wrapMode: Text.Wrap
                                }

                                QQC2.Label {
                                    Layout.fillWidth: true
                                    text: item.modelData.text
                                    wrapMode: Text.Wrap
                                    color: Look.mist
                                }
                            }
                        }
                    }
                }

                SceneFold {
                    Layout.fillWidth: true
                    key: "api-rig"
                    title: App.tr("Bones, shapes and impulses")

                    SceneScriptRigPick {
                        Layout.fillWidth: true
                        page: api.page
                        onInserted: code => api.inserted(code)
                    }
                }
            }
        }
    }
}
