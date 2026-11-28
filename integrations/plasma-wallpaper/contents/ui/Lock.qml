import QtQuick
import org.kde.plasma.workspace.dbus as DBus

Item {
    id: lock

    required property string screen
    readonly property bool active: Qt.application.name === "kscreenlocker_greet"
    property var chosen: ({})
    property bool known: false
    readonly property var mine: (chosen.outputs ?? {})[screen] ?? null
    // a screen marked to keep its desktop shows the desktop's own stream
    readonly property bool own: mine ? !mine.desktop : !!chosen.every
    readonly property string stream: !active ? screen : !known || screen === "" ? "" : own ? "lock:" + screen : screen
    readonly property var lifetime: ({
            "alive": true
        })

    function load() {
        if (!active)
            return;
        const life = lifetime;
        DBus.SessionBus.asyncCall({
            "service": "io.github.reiwand.Daemon",
            "path": "/io/github/reiwand/Daemon",
            "iface": "io.github.reiwand.Daemon1",
            "member": "GetLock",
            "signature": "()",
            "arguments": []
        }, reply => life.alive && lock.read(String(reply.value ?? "")), () => life.alive && lock.read(""));
    }

    function read(text) {
        chosen = text.startsWith("{") ? JSON.parse(text) : {};
        known = true;
    }

    Component.onCompleted: load()
    Component.onDestruction: lifetime.alive = false

    DBus.SignalWatcher {
        service: "io.github.reiwand.Daemon"
        path: "/io/github/reiwand/Daemon"
        iface: "io.github.reiwand.Daemon1"

        function dbusChanged(topic, ids, serial) {
            if (String(topic) === "lock")
                lock.load();
        }
    }

    DBus.DBusServiceWatcher {
        watchedService: "io.github.reiwand.Daemon"
        onRegisteredChanged: {
            if (registered)
                lock.load();
        }
    }
}
