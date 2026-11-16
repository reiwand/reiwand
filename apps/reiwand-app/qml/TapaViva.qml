import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Item {
    id: viva

    property string uid
    property bool wanted
    property string output
    property var stream: null
    readonly property bool asked: wanted && !(QQC2.Overlay.overlay?.visibleChildren.some(c => String(c).startsWith("QQuickPopupItem")) ?? false)

    function start() {
        stop();
        if (App.checking || uid === "")
            return;
        const name = "preview:card-" + Date.now();
        const edge = Math.max(64, Math.ceil(width * Screen.devicePixelRatio));
        output = name;
        App.query("RequestStream", [name, edge, Math.round(edge * 9 / 16)], "Daemon1", reply => viva && viva.output === name && viva.apply(reply), () => viva && viva.output === name && viva.stop());
    }

    function apply(reply) {
        const found = typeof reply === "string" ? JSON.parse(reply) : reply;
        const name = output;
        App.act("Apply", [uid, name, "", ""], "Library1", () => viva && viva.output === name && viva.show(found), () => viva && viva.output === name && viva.stop());
    }

    function show(found) {
        try {
            stream = Qt.createQmlObject("import QtQuick; import org.kde.pipewire as PipeWire; PipeWire.PipeWireSourceItem { anchors.fill: parent }", viva);
            if (found[1] > 0 && stream.objectSerial !== undefined)
                stream.objectSerial = found[1];
            else
                stream.nodeId = found[0];
        } catch (error) {
            stop();
        }
    }

    function stop() {
        rest.stop();
        stream?.destroy();
        stream = null;
        if (output !== "")
            App.act("ReleaseStream", [output], "Daemon1");
        output = "";
    }

    onAskedChanged: asked ? rest.restart() : stop()
    onUidChanged: stop()
    Component.onDestruction: stop()

    Timer {
        id: rest

        interval: 300
        onTriggered: viva.asked && viva.start()
    }
}
