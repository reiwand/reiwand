import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The light of the chosen lamp against the distance, in the Curves panel; its radius and falloff are set in the
// Inspector's Light section, where the same graph is.
ColumnLayout {
    id: falloff

    required property var page
    required property string lamp
    property var curve: null

    function load() {
        curve = lamp ? JSON.parse(page.editor.rig(lamp, JSON.stringify({
            "falloff": {
                "layer": lamp,
                "time": page.editor.time
            }
        })) || "null") : null;
    }

    onLampChanged: load()
    Component.onCompleted: load()

    Connections {
        function onViewChanged() {
            if (falloff.lamp)
                falloff.load();
        }

        target: falloff.page.editor
    }

    QQC2.Label {
        text: App.tr("Light against the distance")
    }

    SceneCurvePlot {
        Layout.fillWidth: true
        Layout.fillHeight: true
        curve: falloff.curve?.curve ?? null
        marks: falloff.curve?.marks ?? []
    }
}
