import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

ColumnLayout {
    id: timeline

    required property var page
    readonly property real length: page.view.length ?? 10
    readonly property var tracks: page.view.tracks ?? []
    readonly property var easings: page.view.easings ?? ["linear"]
    readonly property var names: [""].concat((page.view.timelines ?? []).map(line => line.id))
    property var chosen: null
    readonly property Item line: timeline
    readonly property real nameWidth: Math.max(Kirigami.Units.gridUnit * 8, width * 0.3)
    readonly property real lineWidth: Kirigami.Units.gridUnit * 6
    readonly property var picked: chosen ? (tracks.find(track => track.pointer === chosen.pointer)?.keys ?? [])[chosen.index] ?? null : null

    function seek(time, playing) {
        page.clock = time;
        page.editor.seek(time, playing);
    }

    function drive(track, index) {
        const field = track.pointer + "/timeline";
        if (index > 0)
            page.run("set", {
                "pointer": field,
                "value": names[index]
            });
        else if (track.timeline)
            page.run("remove", {
                "pointer": field
            });
    }

    function edit(ease, remove) {
        page.run("keyframe", {
            "pointer": chosen.pointer,
            "time": picked[0],
            "value": picked[1],
            "ease": ease,
            "remove": remove
        });
    }

    spacing: 0

    Timer {
        interval: 50
        repeat: true
        running: timeline.page.editor.playing && timeline.visible
        onTriggered: timeline.page.clock = (timeline.page.clock + interval / 1000) % timeline.length
    }

    SceneTimeBar {
        id: bar

        Layout.fillWidth: true
        timeline: parent
    }

    SceneKeyMenu {
        id: keyMenu

        timeline: timeline
        bar: bar
    }

    ScenePathLanes {
        id: lanes

        Layout.fillWidth: true
        timeline: timeline
    }

    SceneClockLanes {
        id: clocks

        Layout.fillWidth: true
        timeline: timeline
    }

    Kirigami.PlaceholderMessage {
        id: empty

        Layout.fillWidth: true
        Layout.fillHeight: true
        Layout.margins: Kirigami.Units.largeSpacing
        visible: timeline.tracks.length === 0 && !lanes.visible && !clocks.visible && !held.visible
        text: App.tr("No keyframes yet: the ◇ button next to a value adds one at the current time.")
    }

    SceneTimeRuler {
        Layout.fillWidth: true
        visible: timeline.tracks.length > 0 || held.visible
        timeline: timeline
    }

    SceneTimeLanes {
        id: held

        Layout.fillWidth: true
        timeline: timeline
    }

    QQC2.ScrollView {
        id: listed

        Layout.fillWidth: true
        Layout.fillHeight: true
        visible: timeline.tracks.length > 0

        // One group of lanes per layer, under its name, as in a video editor.
        ListView {
            model: timeline.tracks
            clip: true
            section.property: "layer"
            section.delegate: QQC2.Label {
                required property string section

                width: ListView.view.width
                height: DeskLook.row
                leftPadding: Look.e2
                verticalAlignment: Text.AlignVCenter
                text: section
                font.weight: Font.DemiBold
                elide: Text.ElideRight
            }

            delegate: SceneTimeTrack {
                timeline: line
                menu: keyMenu
            }
        }
    }

    // With neither the list nor its empty message, what shows stays at the top of a tall panel.
    Item {
        Layout.fillHeight: true
        visible: !empty.visible && !listed.visible
    }
}
