import QtQuick
import io.github.reiwand.app

// A canvas whose stream never gives a picture connects to it again, then asks for a new stream; a
// stream that gives pictures is left alone.
QtObject {
    required property var page

    function fail(what) {
        throw new Error("canvas stream wait: " + what);
    }

    function exercise() {
        const made = ready => Qt.createQmlObject("import QtQml; QtObject { property int connects: 0; property int asks: 0; property var stream: ({ ready: " + ready + " }); property var page: ({ editor: { restream: () => asks++ } }); function connect() { connects++; } }", page);
        const silent = made(false);
        const wait = Qt.createComponent("SceneCanvasWait.qml").createObject(page, {
            "canvas": silent
        });
        wait.triggered();
        if (silent.connects !== 1 || silent.asks !== 0)
            fail("a stream with no picture was not connected again first: " + silent.connects + " " + silent.asks);
        wait.triggered();
        if (silent.connects !== 1 || silent.asks !== 1)
            fail("a stream still without a picture did not ask for a new one: " + silent.connects + " " + silent.asks);
        const playing = made(true);
        wait.canvas = playing;
        wait.triggered();
        if (playing.connects !== 0 || playing.asks !== 0)
            fail("a stream that gives pictures was touched");
        wait.destroy();
    }
}
