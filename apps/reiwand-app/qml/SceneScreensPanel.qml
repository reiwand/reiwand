import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The Screens panel: the canvas over one, two or three screens or the author's own, spread over
// every screen, with a map of the scene's thumbnail cut into what each screen shows, and the guide
// of a screen shape (the canvas bar's "Several screens" and "Guide").
Rectangle {
    id: screens

    required property var page
    readonly property var bar: page.docks.stage.canvas.bar
    readonly property var spans: bar.spans
    readonly property var guides: bar.guides

    color: Look.night
    clip: true

    QQC2.ScrollView {
        id: scroll

        anchors.fill: parent
        anchors.leftMargin: Look.e3
        anchors.rightMargin: Look.e2
        anchors.topMargin: Look.e2
        contentWidth: availableWidth

        ScrollColumn {
            view: scroll

            Repeater {
                model: screens.spans.names

                delegate: QQC2.RadioButton {
                    required property int index
                    required property string modelData

                    Layout.fillWidth: true
                    text: modelData
                    checked: screens.spans.current === index
                    onClicked: screens.spans.current = index
                }
            }

            SceneScreensMap {
                Layout.fillWidth: true
                Layout.topMargin: Look.e2
                Layout.bottomMargin: Look.e2
                page: screens.page
                layout: screens.spans.layout
            }

            QQC2.Switch {
                Layout.fillWidth: true
                visible: screens.spans.current > 0
                text: App.tr("Spread over every screen")
                checked: screens.page.editor.value("/manifest/display/span") === "\"all-outputs\""
                onToggled: screens.page.run("manifest", {
                    "display": checked ? {
                        "values": {
                            "span": "all-outputs"
                        }
                    } : {
                        "clear": ["span"]
                    }
                })
            }

            QQC2.Label {
                Layout.topMargin: Look.e2
                text: App.tr("Guide")
                font.weight: Font.DemiBold
            }

            Repeater {
                model: screens.guides.model

                delegate: QQC2.RadioButton {
                    required property int index
                    required property string modelData

                    Layout.fillWidth: true
                    text: modelData
                    checked: screens.guides.currentIndex === index
                    onClicked: screens.guides.currentIndex = index
                }
            }
        }
    }
}
