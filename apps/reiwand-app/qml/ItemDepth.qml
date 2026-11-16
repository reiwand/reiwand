import QtQuick
import io.github.reiwand.app

Fila {
    id: row

    required property var host
    readonly property string path: host.item.kind === "image" && !host.item.missing ? (host.item.path ?? "") : ""
    readonly property var missing: JSON.parse(maps.assets || "[]").filter(asset => !asset.installed && !asset.inpaint)
    readonly property string sizes: missing.map(asset => asset.size).join(" + ")
    readonly property var stages: ({
            "download": App.tr("Downloading the model…"),
            "loading": App.tr("Loading the model…"),
            "saving": App.tr("Saving the map…")
        })
    readonly property string trouble: follow.error !== "" ? follow.error : maps.error
    property bool waiting: false

    function load() {
        waiting = false;
        if (path === "")
            return;
        follow.open(path);
        maps.open(path);
    }

    function make() {
        waiting = false;
        if (follow.moving && maps.map === "" && !maps.busy)
            maps.generate(0);
    }

    function turn() {
        if (follow.moving) {
            follow.follow(false);
            return;
        }
        follow.follow(true);
        if (!follow.moving || maps.map !== "")
            return;
        if (maps.ready) {
            make();
        } else {
            waiting = true;
            consent.open();
        }
    }

    visible: path !== ""
    kind: "llave"
    text: App.tr("Depth")
    note: maps.busy ? (stages[maps.stage] ?? App.tr("Estimating depth…")) : trouble !== "" ? trouble : sizes !== "" && !follow.moving ? App.tr("Move with the pointer · %1 downloaded only once, when you turn it on", sizes) : App.tr("Move with the pointer")
    noteTone: !maps.busy && trouble !== "" ? "coral" : ""
    checked: follow.moving
    value: checked ? App.tr("On") : App.tr("Off")
    onToggled: {
        turn();
        checked = Qt.binding(() => follow.moving);
    }
    onPathChanged: load()
    Component.onCompleted: load()

    DepthFollow {
        id: follow
    }

    DepthMaps {
        id: maps

        onBusyChanged: !busy && ready && row.waiting && row.make()
    }

    DepthDialog {
        id: consent

        maps: maps
        onClosed: {
            if (row.waiting && !maps.busy) {
                row.waiting = false;
                follow.follow(false);
            }
        }
    }
}
