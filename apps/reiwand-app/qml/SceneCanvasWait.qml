import QtQuick

// A stream that gives no picture leaves the canvas on its color for good: after a while without
// one, connect to it again once, then ask the service for a new stream, and say so in the log.
Timer {
    id: wait

    required property var canvas
    property int tries: 0
    readonly property var watched: canvas.stream

    onWatchedChanged: watched && restart()
    interval: 4000
    onTriggered: {
        if (!canvas.stream || canvas.stream.ready !== false) {
            tries = 0;
            return;
        }
        console.warn("the stream of the canvas gave no picture in " + interval / 1000 + " s; " + (tries === 0 ? "connecting again" : "asking for a new one"));
        tries++;
        if (tries === 1)
            canvas.connect();
        else {
            tries = 0;
            canvas.page.editor.restream();
        }
    }
}
