import QtQuick
import io.github.reiwand.app

// The timeline's header Key and Trigger, and its lane of events: Key keys the chosen track at the
// playhead with the value it has there, Trigger adds an event there, and the lane marks it.
QtObject {
    required property var page

    function fail(what) {
        throw new Error("timeline lanes: " + what);
    }

    function exercise(line) {
        const pointer = "/scene/layers/0/opacity";
        page.run("keyframe", {
            "pointer": pointer,
            "time": 0,
            "value": 1
        });
        page.run("keyframe", {
            "pointer": pointer,
            "time": 2,
            "value": 0
        });
        page.clock = 1;
        page.editor.seek(1, false);
        const keys = Qt.createComponent("SceneTimeHeadKeys.qml").createObject(page, {
            "page": page,
            "timeline": {
                "chosen": {
                    "pointer": pointer,
                    "index": 0
                }
            }
        });
        const now = keys.value(pointer);
        if (typeof now !== "number" || now <= 0 || now >= 1)
            fail("the value at the playhead is " + JSON.stringify(now));
        if (!keys.key())
            fail("Key added nothing");
        const added = (JSON.parse(page.editor.value(pointer + "/keys") || "[]") ?? []).find(k => k[0] === 1);
        if (!added || Math.abs(added[1] - now) > 1e-6)
            fail("Key did not key the value at 1 s: " + page.editor.value(pointer + "/keys"));
        keys.trigger(line);
        const lanes = Qt.createComponent("SceneTimeLanes.qml").createObject(page, {
            "timeline": {
                "page": page,
                "length": page.view.length ?? 10,
                "nameWidth": 100,
                "lineWidth": 60,
                "seek": (time, playing) => page.clock = time
            }
        });
        const marked = lanes.events.filter(e => e.line === line.id).map(e => e.name + "@" + e.t);
        if (!marked.includes("event_2@1") || !marked.includes("boom@3") || !lanes.visible)
            fail("the lane marks " + marked.join(", "));
        lanes.destroy();
        keys.destroy();
    }
}
