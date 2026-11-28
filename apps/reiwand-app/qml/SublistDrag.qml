import QtQuick
import io.github.reiwand.app

Item {
    id: drag

    required property Item list
    property int held: -1
    property real tall: 0
    property real start: 0
    property point at
    property real lift: 0
    property Flickable view: null

    signal dropped(int from, int to)

    function scroller() {
        for (let up = list.parent; up; up = up.parent) {
            if (up instanceof Flickable)
                return up;
        }
        return null;
    }

    function begin(index, pressed, height) {
        view = scroller();
        held = index;
        tall = height;
        start = list.mapFromItem(null, pressed.x, pressed.y).y;
        follow(pressed);
    }

    function follow(point) {
        at = point;
        lift = list.mapFromItem(null, at.x, at.y).y - start;
    }

    function scroll() {
        if (!view || view.height <= 0)
            return;
        const y = view.mapFromItem(null, at.x, at.y).y;
        const edge = Math.min(Look.sublistHeight, view.height / 3);
        const over = y < edge ? y - edge : y > view.height - edge ? y - view.height + edge : 0;
        const step = Math.max(-edge, Math.min(edge, over / 3));
        const top = Math.max(0, view.contentHeight - view.height);
        const next = Math.max(0, Math.min(top, view.contentY + step));
        if (next === view.contentY)
            return;
        view.contentY = next;
        follow(at);
    }

    function end() {
        if (held < 0)
            return;
        const from = held;
        const to = from + Math.round(lift / Math.max(1, tall));
        held = -1;
        lift = 0;
        view = null;
        dropped(from, to);
    }

    Timer {
        interval: 16
        repeat: true
        running: drag.held >= 0 && drag.view !== null
        onTriggered: drag.scroll()
    }
}
