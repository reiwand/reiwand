import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

Item {
    id: times

    required property var page
    readonly property bool clipped: page.mesh.clip !== ""
    readonly property alias timeline: timeline
    readonly property alias clips: clips
    // What the curve editor draws: the keys of the line in view, those of the chosen value's layer
    // or row first.
    readonly property var sources: clipped ? SceneGraphSources.rows(clips.rows, clips.mesh, clips.chosen) : SceneGraphSources.tracks(timeline.tracks, timeline.chosen)
    readonly property real now: clipped ? clips.now : page.clock

    MouseArea {
        anchors.fill: parent
    }

    StackLayout {
        anchors.fill: parent
        currentIndex: times.clipped ? 1 : 0

        SceneTimeline {
            id: timeline

            page: times.page
        }

        SceneClipLine {
            id: clips

            page: times.page
        }
    }
}
