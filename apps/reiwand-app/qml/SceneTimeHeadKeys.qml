import QtQuick
import QtQml
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// Key and Trigger in the timeline's header. Key adds a key at the playhead to the chosen track,
// with the value it has there. Trigger adds an event at the playhead to a timeline, named
// event_1, event_2…, which other parts of the scene can start with ("event:<name>").
RowLayout {
    id: keys

    required property var page
    required property var timeline
    readonly property var chosen: timeline.chosen
    readonly property var lines: page.view.timelines ?? []

    // The value a track has at the playhead, as the inspector shows it.
    function value(pointer) {
        let owner = pointer;
        while (owner.split("/").length > 3) {
            owner = owner.split("/").slice(0, -1).join("/");
            const field = (JSON.parse(page.editor.inspect(owner) || "[]") ?? []).find(f => f.pointer === pointer);
            if (field)
                return field.shown ?? field.value ?? field.default;
        }
        return undefined;
    }

    function key() {
        const now = value(chosen.pointer);
        if (now === undefined)
            return false;
        page.run("keyframe", {
            "pointer": chosen.pointer,
            "time": page.editor.time,
            "value": now
        });
        return true;
    }

    function fresh(line) {
        const taken = (JSON.parse(page.editor.value(line.pointer) || "null") ?? {}).events ?? [];
        let n = taken.length + 1;
        while (taken.some(e => e.name === "event_" + n))
            n++;
        return "event_" + n;
    }

    function trigger(line) {
        page.run("timeline", {
            "id": line.id,
            "add_events": [
                {
                    "t": Math.round(Math.min(Number(line.duration) || 0, page.clock) * 1000) / 1000,
                    "name": fresh(line)
                }
            ]
        });
    }

    spacing: Look.e1

    QQC2.ToolButton {
        icon.name: "diamond"
        text: App.tr("Key")
        enabled: keys.chosen !== null
        QQC2.ToolTip.text: enabled ? App.tr("Add a key at the playhead to the chosen track") : App.tr("Choose a key of a track first")
        QQC2.ToolTip.visible: hovered
        onClicked: keys.key()
    }

    QQC2.ToolButton {
        id: cause

        icon.name: "lightning"
        text: App.tr("Trigger")
        enabled: keys.lines.length > 0
        QQC2.ToolTip.text: enabled ? App.tr("Add an event at the playhead to a timeline") : App.tr("Events belong to a timeline: make one in Change… first")
        QQC2.ToolTip.visible: hovered
        onClicked: keys.lines.length === 1 ? keys.trigger(keys.lines[0]) : pick.popup(cause, 0, cause.height)

        QQC2.Menu {
            id: pick

            Instantiator {
                model: keys.lines
                onObjectAdded: (index, object) => pick.insertItem(index, object)
                onObjectRemoved: (index, object) => pick.removeItem(object)

                delegate: QQC2.MenuItem {
                    required property var modelData

                    text: App.tr("Add an event to %1", modelData.id)
                    onTriggered: keys.trigger(modelData)
                }
            }
        }
    }
}
