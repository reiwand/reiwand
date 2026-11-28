import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The lane of a sound layer in the timeline: its name, what starts it and the waveform of its
// sounds one after the other from 0 s.
RowLayout {
    id: sound

    required property var owner
    required property var modelData
    readonly property var starts: modelData.sounds.reduce((all, name) => all.concat([all[all.length - 1] + Number(owner.wave(name).seconds ?? 0)]), [0])

    Layout.fillWidth: true
    Layout.preferredHeight: DeskLook.row
    spacing: Look.e2

    StatusLine {
        Layout.preferredWidth: owner.timeline.nameWidth
        leftPadding: Look.e2
        clip: true
        icon: "speaker-high"
        text: sound.modelData.id
        size: DeskLook.statusIcon
        pixels: DeskLook.tStatus
        ink: Look.frost
    }

    QQC2.Label {
        Layout.preferredWidth: owner.timeline.lineWidth
        text: sound.modelData.play_on ? App.tr("Starts with %1", sound.modelData.play_on) : App.tr("Starts with the scene")
        color: Look.mist
        font.pixelSize: DeskLook.tCap
        elide: Text.ElideRight
    }

    Item {
        id: lane

        Layout.fillWidth: true
        Layout.fillHeight: true
        clip: true

        Repeater {
            model: sound.modelData.sounds

            delegate: SceneWave {
                required property int index
                required property string modelData

                x: owner.at(sound.starts[index], lane.width)
                width: owner.at(sound.starts[index + 1], lane.width) - x
                height: lane.height
                fades: false
                wave: owner.wave(modelData)
                QQC2.ToolTip.text: modelData
                QQC2.ToolTip.visible: hover.hovered

                HoverHandler {
                    id: hover
                }
            }
        }

        Rectangle {
            x: owner.at(owner.page.clock, lane.width)
            width: Look.line
            height: parent.height
            color: Look.coral
        }
    }
}
