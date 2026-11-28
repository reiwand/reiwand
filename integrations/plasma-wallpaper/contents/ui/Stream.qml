import QtQuick
import QtQuick.Window
import org.kde.pipewire as PipeWire
import org.kde.plasma.workspace.dbus as DBus

Item {
    id: stream

    required property bool covered
    required property string output
    readonly property real ratio: Window.window?.devicePixelRatio || Screen.devicePixelRatio
    readonly property size pixels: Qt.size(Math.max(1, Math.round(width * ratio)), Math.max(1, Math.round(height * ratio)))
    readonly property var place: [Screen.virtualX, Screen.virtualY, Screen.width, Screen.height, ratio, Number.isFinite(Screen.pixelDensity) ? Screen.pixelDensity * ratio : 0, [Screen.manufacturer, Screen.model, Screen.serialNumber].filter(part => part).join(" ")]
    readonly property bool wanted: output !== "" && width > 0 && height > 0
    readonly property bool broken: error !== "" || (view.item?.state ?? 0) <= PipeWire.PipeWireSourceItem.Unconnected
    readonly property bool streaming: view.item?.state === PipeWire.PipeWireSourceItem.Streaming
    readonly property bool ready: view.item?.ready ?? false
    readonly property var lifetime: ({
            "alive": true
        })
    property string owned: ""
    property string error: ""
    property bool requesting: false
    property int failures: 0

    function message(member, args) {
        const signatures = {
            "RequestStream": "(suu)",
            "ReleaseStream": "(s)",
            "SetOutputCovered": "(sb)",
            "SetOutputInfo": "(siiuudds)"
        };
        return {
            "service": "io.github.reiwand.Daemon",
            "path": "/io/github/reiwand/Daemon",
            "iface": "io.github.reiwand.Daemon1",
            "member": member,
            "signature": signatures[member],
            "arguments": args
        };
    }

    function send(member, args) {
        DBus.SessionBus.asyncCall(message(member, args));
    }

    function release() {
        if (owned === "")
            return;
        send("SetOutputCovered", [owned, false]);
        send("ReleaseStream", [owned]);
        owned = "";
    }

    function request() {
        if (requesting || !wanted)
            return;
        const target = output, w = pixels.width, h = pixels.height, life = lifetime, bus = DBus.SessionBus;
        const abandon = message("ReleaseStream", [target]);
        requesting = true;
        bus.asyncCall(message("RequestStream", [target, w, h]), reply => life.alive ? stream.received(target, w, h, reply) : bus.asyncCall(abandon), reply => life.alive && stream.failed(reply));
    }

    function received(target, w, h, reply) {
        requesting = false;
        error = "";
        if (target !== output) {
            send("ReleaseStream", [target]);
            resize.restart();
            return;
        }
        // The stream of the screen this wallpaper showed before goes back only now that the new one is
        // here: when Plasma swaps the desktops of two screens, the other wallpaper asks for it first
        // and the stream stays open instead of closing and starting again.
        if (owned === target)
            send("ReleaseStream", [target]);
        else
            release();
        owned = target;
        send("SetOutputInfo", [target].concat(place));
        const values = (reply.values ?? []).map(v => Number(v?.value ?? v));
        show(values[0] ?? 0, values[1] ?? 0, broken);
        send("SetOutputCovered", [target, covered]);
        if (w !== pixels.width || h !== pixels.height)
            resize.restart();
    }

    function show(node, serial, reconnect) {
        if (!reconnect && node === view.node && serial === view.serial)
            return;
        view.node = node;
        view.serial = serial;
        view.active = false;
        view.active = true;
    }

    function failed(reply) {
        requesting = false;
        if (owned !== output)
            release();
        error = reply?.error?.message || i18nd("plasma_wallpaper_io.github.reiwand", "the daemon (reiwandd) is not running");
    }

    onPixelsChanged: resize.restart()
    onPlaceChanged: {
        if (owned !== "" && owned === output)
            send("SetOutputInfo", [owned].concat(place));
    }
    onOutputChanged: {
        if (output === "")
            release();
        resize.restart();
    }
    onCoveredChanged: {
        if (owned !== "" && owned === output)
            send("SetOutputCovered", [owned, covered]);
    }
    onStreamingChanged: {
        if (streaming)
            failures = 0;
    }
    Component.onCompleted: resize.restart()
    Component.onDestruction: {
        lifetime.alive = false;
        release();
    }

    Loader {
        id: view

        property int node: 0
        property real serial: 0

        anchors.fill: parent
        active: false

        sourceComponent: PipeWire.PipeWireSourceItem {
            id: source

            Component.onCompleted: {
                if (view.serial > 0 && source.objectSerial !== undefined)
                    source.objectSerial = view.serial;
                else
                    source.nodeId = view.node;
            }
        }
    }

    DBus.SignalWatcher {
        service: "io.github.reiwand.Daemon"
        path: "/io/github/reiwand/Daemon"
        iface: "io.github.reiwand.Daemon1"

        function dbusStreamChanged(target, node, serial) {
            if (target === stream.owned)
                stream.show(Number(node), Number(serial), true);
        }
    }

    DBus.DBusServiceWatcher {
        watchedService: "io.github.reiwand.Daemon"
        onRegisteredChanged: {
            if (!registered) {
                stream.owned = "";
                return;
            }
            stream.failures = 0;
            view.active = false;
            resize.restart();
        }
    }

    Timer {
        id: resize

        interval: 250
        onTriggered: stream.request()
    }

    Timer {
        interval: [1000, 2000, 5000, 10000][Math.min(stream.failures, 3)]
        running: stream.broken && stream.wanted && !stream.requesting
        repeat: true
        onTriggered: {
            stream.failures++;
            stream.request();
        }
    }
}
