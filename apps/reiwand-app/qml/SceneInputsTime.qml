import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The time of day the preview sees, and the triggers the scene starts on, fired by hand: the two
// inputs a scene gets from the moment it is shown and from what happens around it.
ColumnLayout {
    id: time

    required property var page
    readonly property bool chosen: SceneLook.timeOfDay >= 0
    readonly property var cues: {
        page.view;
        return JSON.parse(page.editor.rig("", JSON.stringify({
            "cues": {}
        })) || "[]") ?? [];
    }

    function set(seconds) {
        SceneLook.timeOfDay = seconds;
        SceneLook.again(page);
    }

    spacing: Look.e1

    PanelHeading {
        Layout.fillWidth: true
        Layout.topMargin: Look.e2
        level: 4
        text: App.tr("Time of day")
    }

    QQC2.Switch {
        Layout.fillWidth: true
        text: App.tr("Choose the hour the scene sees")
        checked: time.chosen
        onToggled: time.set(checked ? 12 * 3600 : -1)
    }

    RowLayout {
        visible: time.chosen
        spacing: Look.e1

        QQC2.SpinBox {
            id: hour

            from: 0
            to: 23
            value: Math.floor(Math.max(0, SceneLook.timeOfDay) / 3600)
            Accessible.name: App.tr("Hour")
            onValueModified: time.set(value * 3600 + minute.value * 60)
        }

        QQC2.Label {
            text: ":"
        }

        QQC2.SpinBox {
            id: minute

            from: 0
            to: 59
            value: Math.floor(Math.max(0, SceneLook.timeOfDay) % 3600 / 60)
            Accessible.name: App.tr("Minute")
            onValueModified: time.set(hour.value * 3600 + value * 60)
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        text: time.chosen ? App.tr("The preview starts at this hour and runs on from it; the scene and the desktop keep the clock of the computer.") : App.tr("The preview sees the clock of the computer.")
        color: Look.mist
        font.pixelSize: Look.tFoot
        wrapMode: Text.Wrap
    }

    PanelHeading {
        Layout.fillWidth: true
        Layout.topMargin: Look.e2
        level: 4
        text: App.tr("Triggers")
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: time.cues.length === 0
        text: App.tr("No timeline or sound of the scene starts on a trigger yet.")
        color: Look.mist
        font.pixelSize: Look.tFoot
        wrapMode: Text.Wrap
    }

    Repeater {
        model: time.cues

        delegate: RowLayout {
            id: cue

            required property var modelData
            readonly property var started: (modelData.timelines ?? []).map(t => App.tr("the timeline %1", t)).concat((modelData.sounds ?? []).map(s => App.tr("the sound %1", s)))

            Layout.fillWidth: true
            spacing: Look.e1

            QQC2.Label {
                Layout.fillWidth: true
                Layout.minimumWidth: 0
                text: SceneWords.label(cue.modelData.trigger)
                elide: Text.ElideRight
                QQC2.ToolTip.text: App.tr("Starts %1", cue.started.join(", "))
                QQC2.ToolTip.visible: hoverArea.hovered

                HoverHandler {
                    id: hoverArea
                }
            }

            QQC2.ToolButton {
                icon.name: "media-playback-start"
                text: App.tr("Fire")
                Accessible.name: App.tr("Fire %1 in the preview", SceneWords.label(cue.modelData.trigger))
                onClicked: time.page.editor.fire(cue.modelData.trigger)
            }
        }
    }
}
