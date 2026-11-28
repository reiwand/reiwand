import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The Inputs panel: what the scene would get from its user while it is tried here: the pointer
// and clicks over the canvas, the sound, the time of day, its triggers and the simulated player.
Rectangle {
    id: inputs

    required property var page
    readonly property var canvas: page.docks.stage.canvas

    color: Look.night
    clip: true

    QQC2.ScrollView {
        id: scroll

        anchors.fill: parent
        anchors.leftMargin: Look.e3 - Look.ringRoom
        anchors.rightMargin: Look.e2 - Look.ringRoom
        anchors.topMargin: Look.e2 - Look.ringRoom
        contentWidth: availableWidth

        ScrollColumn {
            view: scroll
            spacing: Look.e1

            QQC2.Label {
                text: App.tr("Pointer and clicks")
                font.weight: Font.DemiBold
            }

            QQC2.Switch {
                Layout.fillWidth: true
                text: App.tr("Test the pointer")
                checked: inputs.canvas.testing
                onToggled: {
                    inputs.canvas.testing = checked;
                    if (!checked)
                        inputs.page.editor.touch(-1, -1, 0);
                }
            }

            QQC2.Label {
                Layout.fillWidth: true
                text: App.tr("While it is on, the mouse over the canvas acts as the pointer and its clicks reach the scene.")
                color: Look.mist
                font.pixelSize: Look.tFoot
                wrapMode: Text.Wrap
            }

            SceneProbeTool {
                page: inputs.page
                display: QQC2.AbstractButton.TextBesideIcon
            }

            QQC2.Label {
                Layout.topMargin: Look.e2
                text: App.tr("Sound")
                font.weight: Font.DemiBold
            }

            // The bands of what plays on the computer, as the scene hears them.
            SceneBandMeter {
                Layout.fillWidth: true
                Layout.preferredHeight: Look.e8
                page: inputs.page
                live: inputs.visible
            }

            SceneInputsTime {
                Layout.fillWidth: true
                page: inputs.page
            }

            QQC2.Label {
                Layout.topMargin: Look.e2
                text: App.tr("Simulated player")
                font.weight: Font.DemiBold
            }

            SceneMedia {
                Layout.fillWidth: true
                // Its words start under the heading above; the room for focus rings goes outside.
                Layout.leftMargin: -Look.ringRoom
                Layout.rightMargin: -Look.ringRoom
                page: inputs.page
            }
        }
    }
}
