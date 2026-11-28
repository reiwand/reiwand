import QtQuick
import QtQuick.Layouts
import "../look"

ColumnLayout {
    id: head

    required property var daemon
    property bool titled: false
    readonly property var current: daemon.current
    readonly property bool paused: current.paused === true
    readonly property bool muted: current.muted === true
    property alias pause: pausing

    enabled: daemon.pollError === ""
    spacing: Look.e3

    RowLayout {
        Layout.fillWidth: true
        visible: head.titled

        Words {
            text: "Reiwand"
            title: true
            font.pixelSize: Look.tRow
        }

        Item {
            Layout.fillWidth: true
        }

        Status {
            paused: head.paused
        }
    }

    Rectangle {
        Layout.preferredHeight: Look.controlHeight
        Layout.preferredWidth: group.implicitWidth + 2 * Look.line
        radius: Look.radius
        color: "transparent"
        border.width: Look.line
        border.color: Look.nightLine

        Command {
            id: pauseProbe

            visible: false
            quiet: true
            glyph: "pause-fill"
            text: i18n("Pause")
        }

        Command {
            id: resumeProbe

            visible: false
            quiet: true
            glyph: "play-fill"
            text: i18n("Resume")
        }

        RowLayout {
            id: group

            anchors.fill: parent
            anchors.margins: Look.line
            spacing: 0

            Command {
                id: pausing

                Layout.fillHeight: true
                // As wide as the longer of its two labels, so the volume beside it stays put on a pause.
                Layout.preferredWidth: Math.max(pauseProbe.implicitWidth, resumeProbe.implicitWidth)
                quiet: true
                glyph: head.paused ? "play-fill" : "pause-fill"
                text: head.paused ? i18n("Resume") : i18n("Pause")
                onClicked: head.daemon.act("SetPaused", [!head.paused])
            }

            Rectangle {
                Layout.fillHeight: true
                implicitWidth: Look.line
                color: Look.nightLine
            }

            Command {
                Layout.fillHeight: true
                quiet: true
                glyph: head.muted ? "speaker-slash" : "speaker-high"
                text: ""
                Accessible.name: head.muted ? i18n("Unmute") : i18n("Mute all")
                onClicked: head.daemon.act("SetMuted", [!head.muted])
            }

            Level {
                id: volume

                Layout.preferredWidth: 84
                Layout.rightMargin: Look.e3
                from: 0
                to: 1
                Accessible.name: i18n("Master volume")
                onMoved: debounce.restart()

                // While it is held, or what it sent is still on its way back, a state that comes in does
                // not move it under the pointer.
                Binding on value {
                    value: head.current.volume
                    when: !volume.pressed && !debounce.running && !settle.running
                    restoreMode: Binding.RestoreNone
                }
            }
        }
    }

    Timer {
        id: debounce

        interval: 200
        onTriggered: {
            head.daemon.act("SetVolume", [volume.value]);
            if (head.muted)
                head.daemon.act("SetMuted", [false]);
            settle.restart();
        }
    }

    Timer {
        id: settle

        interval: 600
    }
}
