import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The time of the canvas while the Timeline panel is out of view: live or paused at the playhead,
// the clock, the playhead, a restart, a new seed, a capture, the time of day the canvas sees and
// the canvas clock menu.
Rectangle {
    id: time

    required property var page
    readonly property real length: page.view.length ?? 10
    // A narrow canvas keeps the buttons as icons and drops the time of day, so the row stays inside.
    readonly property bool narrow: width < Look.e8 * 13

    function seek(at, playing) {
        page.clock = at;
        page.editor.seek(at, playing);
    }

    implicitHeight: Look.canvasBarHeight
    color: Look.night

    Timer {
        interval: 50
        repeat: true
        running: time.page.editor.playing && time.visible
        onTriggered: time.page.clock = (time.page.clock + interval / 1000) % time.length
    }

    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: Look.e3
        anchors.rightMargin: Look.e3
        spacing: Look.e2

        QQC2.ToolButton {
            icon.name: "media-playback-start"
            text: App.tr("Live")
            display: time.narrow ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
            checked: time.page.editor.playing
            QQC2.ToolTip.text: App.tr("The canvas runs as it will on the desktop")
            QQC2.ToolTip.visible: hovered
            onClicked: time.seek(time.page.clock, true)
        }

        QQC2.ToolButton {
            icon.name: "media-playback-pause"
            text: App.tr("Paused at the playhead")
            display: time.narrow ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
            checked: !time.page.editor.playing
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered && time.narrow
            onClicked: time.seek(time.page.clock, false)
        }

        QQC2.Label {
            text: App.tr("%1 s", time.page.clock.toFixed(2))
            font.features: {
                "tnum": 1
            }
        }

        SceneSlider {
            Layout.fillWidth: true
            Layout.minimumWidth: Look.e8
            from: 0
            to: time.length
            value: time.page.clock
            onMoved: time.seek(value, false)
        }

        QQC2.ToolButton {
            icon.name: "arrows-clockwise"
            text: App.tr("Restart the preview")
            display: QQC2.AbstractButton.IconOnly
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: SceneLook.again(time.page)
        }

        QQC2.ToolButton {
            icon.name: "roll"
            text: App.tr("New seed")
            display: time.narrow ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
            QQC2.ToolTip.text: App.tr("Start the canvas again with other random values; the scene keeps its own seed")
            QQC2.ToolTip.visible: hovered
            onClicked: SceneLook.newSeed(time.page)
        }

        SceneCapture {
            page: time.page
            above: true
            display: time.narrow ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
        }

        QQC2.Label {
            id: wall

            function tell() {
                const seen = SceneLook.timeOfDay >= 0 ? new Date(new Date().setHours(0, 0, SceneLook.timeOfDay, 0)) : new Date();
                text = App.tr("Canvas time: %1", Qt.formatTime(seen, Qt.locale().timeFormat(Locale.ShortFormat)));
            }

            visible: !time.narrow
            color: Look.mist
            Component.onCompleted: tell()

            Connections {
                target: SceneLook

                function onTimeOfDayChanged() {
                    wall.tell();
                }
            }

            Timer {
                interval: 15000
                repeat: true
                running: wall.visible
                onTriggered: wall.tell()
            }
        }

        SceneCanvasClock {
            page: time.page
        }
    }
}
