import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The ruler over the lanes of the timeline: a tick every half second, the seconds written every
// second, and the playhead; a click or a drag on it moves the playhead.
Item {
    id: ruler

    required property var timeline
    readonly property real start: timeline.nameWidth + timeline.lineWidth + Look.e2 * 2
    readonly property real span: Math.max(1, width - start)
    readonly property int halves: Math.max(1, Math.round(timeline.length * 2))

    implicitHeight: DeskLook.capHeight + Look.e1

    Repeater {
        model: ruler.halves + 1

        delegate: Item {
            required property int index

            x: ruler.start + index / ruler.halves * ruler.span
            height: ruler.height

            Rectangle {
                y: index % 2 === 0 ? 0 : ruler.height / 2
                width: Look.line
                height: index % 2 === 0 ? ruler.height : ruler.height / 2
                color: Look.nightLine
            }

            QQC2.Label {
                x: Look.e1
                visible: index % 2 === 0 && index < ruler.halves
                text: App.tr("%1 s", index / 2)
                color: Look.mist
                font.pixelSize: DeskLook.tCap
            }
        }
    }

    Rectangle {
        x: ruler.start + ruler.timeline.page.clock / ruler.timeline.length * ruler.span
        width: Look.line
        height: ruler.height
        color: Look.coral
    }

    MouseArea {
        x: ruler.start
        width: ruler.span
        height: ruler.height
        onPressed: mouse => ruler.timeline.seek(Math.max(0, Math.min(1, mouse.x / ruler.span)) * ruler.timeline.length, false)
        onPositionChanged: mouse => pressed && ruler.timeline.seek(Math.max(0, Math.min(1, mouse.x / ruler.span)) * ruler.timeline.length, false)
    }
}
