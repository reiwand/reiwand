import QtQuick
import "../integrations/plasma-wallpaper/contents/ui"

Item {
    id: check

    property var errors: []

    function made(ready, covered) {
        return Qt.createQmlObject("import QtQuick; QtObject { property string owned: 'DP-5'; property bool wanted: true; property bool requesting: false; property bool ready: " + ready + "; property bool covered: " + covered + "; property int connects: 0; property int asks: 0; function reconnect() { connects++; } function request() { asks++; } }", check);
    }

    function expect(ok, what) {
        if (!ok)
            errors.push(what);
    }

    Component.onCompleted: {
        const silent = made(false, false);
        const wait = Qt.createComponent("../integrations/plasma-wallpaper/contents/ui/StreamWait.qml").createObject(check, {
            "stream": silent
        });
        expect(wait.running, "a shown stream without a picture is not waited for");
        wait.triggered();
        expect(silent.connects === 1 && silent.asks === 0, "it did not connect again first: " + silent.connects + " " + silent.asks);
        wait.triggered();
        expect(silent.connects === 1 && silent.asks === 1, "it did not ask for a new stream next: " + silent.connects + " " + silent.asks);
        silent.covered = true;
        expect(!wait.running, "a covered wallpaper is waited for");
        wait.triggered();
        expect(silent.asks === 1 && silent.connects === 1, "a covered wallpaper was touched");
        silent.covered = false;
        silent.ready = true;
        expect(!wait.running && wait.tries === 0, "a stream with a picture is still waited for");
        wait.destroy();
        console.log(errors.length ? "FAILED: " + errors.join("; ") : "stream wait: ok");
        Qt.exit(errors.length ? 1 : 0);
    }
}
