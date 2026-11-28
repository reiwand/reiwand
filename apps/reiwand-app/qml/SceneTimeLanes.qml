import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The lanes of the timeline that hold no keys: each sound layer with the waveform of its sounds,
// one after the other from where it starts, and the events of every timeline as marks at their
// second. A click on a mark takes the playhead there.
ColumnLayout {
    id: lanes

    required property var timeline
    readonly property var page: timeline.page
    readonly property var found: (page.view, JSON.parse(page.editor.rig("", JSON.stringify({
            "sounds": {}
        })) || "null") ?? {})
    readonly property var sounds: found.layers ?? []
    readonly property var events: (page.view.timelines ?? []).reduce((all, line) => all.concat(((JSON.parse(page.editor.value(line.pointer) || "null") ?? {}).events ?? []).map(event => ({
                    "t": event.t,
                    "name": event.name,
                    "line": line.id
                }))), [])
    property var waves: ({})

    function key(asset) {
        return asset + ":" + ((found.assets ?? []).find(a => a.name === asset)?.bytes ?? 0);
    }

    function wave(asset) {
        return waves[key(asset)] ?? ({});
    }

    function fetch() {
        const next = Object.assign({}, waves);
        let asked = false;
        for (const asset of found.assets ?? []) {
            if (next[key(asset.name)] === undefined) {
                next[key(asset.name)] = {};
                asked = true;
                page.editor.wave(asset.name, 256);
            }
        }
        if (asked)
            waves = next;
    }

    function at(seconds, width) {
        return Math.max(0, Math.min(1, seconds / timeline.length)) * width;
    }

    spacing: 0
    visible: sounds.length > 0 || events.length > 0
    onFoundChanged: fetch()
    Component.onCompleted: fetch()

    Connections {
        target: lanes.page.editor

        function onWaved(asset, drawn) {
            const next = Object.assign({}, lanes.waves);
            next[lanes.key(asset)] = JSON.parse(drawn);
            lanes.waves = next;
        }
    }

    Repeater {
        model: lanes.sounds

        delegate: SceneTimeSound {
            owner: lanes
        }
    }

    RowLayout {
        Layout.fillWidth: true
        Layout.preferredHeight: DeskLook.row
        visible: lanes.events.length > 0
        spacing: Look.e2

        StatusLine {
            Layout.preferredWidth: lanes.timeline.nameWidth + lanes.timeline.lineWidth + Look.e2
            leftPadding: Look.e2
            clip: true
            icon: "lightning"
            text: App.tr("Events")
            size: DeskLook.statusIcon
            pixels: DeskLook.tStatus
            ink: Look.frost
        }

        Item {
            id: marks

            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

            Repeater {
                model: lanes.events

                delegate: QQC2.AbstractButton {
                    id: mark

                    required property var modelData

                    x: lanes.at(modelData.t, marks.width)
                    height: marks.height
                    width: chip.implicitWidth + Look.e2
                    text: App.tr("%1 · %2, at %3 s of it", modelData.name, modelData.line, Number(modelData.t.toFixed(2)))
                    Accessible.name: text
                    QQC2.ToolTip.text: text
                    QQC2.ToolTip.visible: hovered
                    onClicked: lanes.timeline.seek(Math.min(lanes.timeline.length, modelData.t), false)

                    contentItem: Item {
                        Rectangle {
                            width: Look.line
                            height: parent.height
                            color: Look.moon
                        }

                        StatusLine {
                            id: chip

                            x: Look.e1
                            anchors.verticalCenter: parent.verticalCenter
                            icon: "lightning"
                            text: mark.modelData.name
                            size: DeskLook.statusIcon
                            pixels: DeskLook.tCap
                            ink: mark.hovered ? Look.frost : Look.mist
                        }
                    }

                    background: null
                }
            }

            Rectangle {
                x: lanes.at(lanes.page.clock, marks.width)
                width: Look.line
                height: parent.height
                color: Look.coral
            }
        }
    }
}
