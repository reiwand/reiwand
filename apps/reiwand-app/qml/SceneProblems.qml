import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

ColumnLayout {
    id: panel

    required property var page
    readonly property var problems: page.view.problems ?? []
    readonly property string output: page.editor.output
    // A script of the preview that stopped with an error is a problem too, first in the list.
    property string scriptError: SceneScriptFault.of(page)
    readonly property int count: problems.length + (scriptError !== "" ? 1 : 0)
    readonly property real wanted: Kirigami.Units.gridUnit * (count ? 7 : 1.6)
    Layout.preferredHeight: wanted
    property var inputs: ({})
    readonly property var uses: (inputs.shown ?? {})[panel.output] ?? {}
    readonly property var off: panel.unmet(panel.uses)

    function unmet(used) {
        const granted = used.granted ?? [];
        const wanted = ["audio", "pointer", "clicks", "network", "rgb", "media"].filter(i => used[i] === true);
        if (used.media && used.network)
            wanted.push("media_network");
        if (used.audio && used.network)
            wanted.push("audio_network");
        return wanted.filter(i => !granted.includes(i));
    }

    function load() {
        App.query("GetInputs", [], "Inputs1", reply => panel && (panel.inputs = reply));
    }

    Component.onCompleted: load()

    Connections {
        target: App

        function onUpdated(topic) {
            if (topic === "inputs")
                reload.restart();
        }
    }

    Timer {
        id: reload

        interval: 300
        onTriggered: panel.load()
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: panel.count === 0
        text: App.tr("No problems")
        color: Look.mist
    }

    QQC2.Label {
        visible: panel.count > 0
        text: App.tr("Problems (%1)", panel.count)
        font.weight: Font.DemiBold
    }

    Failure {
        objectName: "scriptError"
        Layout.fillWidth: true
        visible: panel.scriptError !== ""
        title: SceneScriptFault.line(panel.scriptError) > 0 ? App.tr("Error in the script, line %1", SceneScriptFault.line(panel.scriptError)) : App.tr("Error in the script")
        text: panel.scriptError
        action: SceneScriptFault.file(panel.scriptError) !== "" ? App.tr("Go to the line") : ""
        onActed: SceneScriptFault.open(panel.page, panel.scriptError)
    }

    QQC2.ScrollView {
        Layout.fillWidth: true
        Layout.fillHeight: true
        Layout.minimumHeight: 0
        Layout.preferredHeight: 0
        visible: panel.problems.length > 0
        contentWidth: availableWidth

        ListView {
            model: panel.problems
            clip: true
            spacing: Look.e1

            delegate: SceneProblemRow {
                required property var modelData

                width: ListView.view.width
                page: panel.page
                problem: modelData
            }
        }
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: panel.off.length > 0
        type: Kirigami.MessageType.Warning
        text: App.tr("A script wants %1 but it is not granted here, so that part of the scene is off while you try it.", panel.off.join(", "))
    }
}
