import QtQuick

// A stream that gives no picture leaves the wallpaper black for good: when the wallpaper is shown
// and uncovered and its stream gives no picture for a while, connect to it again once, then ask the
// service for a new stream, and say so in the log (plasmashell's journal). While covered the
// service draws nothing, so the wait only counts while the wallpaper can be seen.
Timer {
    id: wait

    required property var stream
    property int tries: 0
    readonly property bool waiting: stream.owned !== "" && stream.wanted && !stream.covered && !stream.ready && !stream.requesting

    interval: 4000
    repeat: true
    running: waiting
    onWaitingChanged: {
        if (!stream.ready)
            return;
        tries = 0;
    }
    onTriggered: {
        if (!waiting)
            return;
        console.warn("Reiwand: the stream of " + stream.owned + " gave no picture in " + interval / 1000 + " s; " + (tries === 0 ? "connecting again" : "asking the service for a new one"));
        tries++;
        if (tries === 1)
            stream.reconnect();
        else {
            tries = 0;
            stream.request();
        }
    }
}
