import QtQuick
import io.github.reiwand.app

Timer {
    id: check

    required property var host
    property var shown: null
    property int step: 0
    property int tries: 0
    readonly property var empty: ({
            "model": 1,
            "serial": 2,
            "output": "DP-1",
            "active": false,
            "ended": null,
            "source": "library",
            "count": 1,
            "candidate": null,
            "now": null,
            "ahead": [],
            "behind": [],
            "plays": {
                "kind": "fade",
                "why": "general",
                "duration_ms": 600
            }
        })
    readonly property var calm: Object.assign({}, empty, {
        "active": true,
        "count": 2,
        "candidate": {
            "uid": "01J9Z0X9Z6Q7X4F3K5E1V0R1VD",
            "kind": "video",
            "title": "Waves"
        },
        "plays": {
            "kind": "fade",
            "why": "calm",
            "duration_ms": 600
        }
    })
    readonly property var steps: [[() => {}, () => shown.began && shown.live && shown.mode === "normal" && shown.candidate.title === "Aurora" && shown.panel.reel.cards.count === 3 && shown.position === App.tr("%1 of %2", 2, 3) && shown.panel.hints.model.length === 3], [() => shown.press(Qt.Key_Right), () => shown.log.slice(-1)[0] === 'Browse ["DP-1","library",1]' && shown.candidate.title === "Waves" && shown.panel.reel.card(0).item.title === "Waves"], [() => shown.press(Qt.Key_Left), () => shown.log.slice(-1)[0] === 'Browse ["DP-1","library",-1]'], [() => {
                shown.scroll(-60);
                shown.scroll(-60);
            }, () => shown.log.slice(-1)[0] === 'Browse ["DP-1","library",1]' && shown.wheel === 0], [() => shown.scroll(120), () => shown.log.slice(-1)[0] === 'Browse ["DP-1","library",-1]'], [() => shown.panel.reel.card(1).clicked(1), () => shown.log.slice(-1)[0] === 'Browse ["DP-1","library",1]'], [() => shown.panel.reel.card(0).clicked(0), () => shown.log.slice(-1)[0] === 'BrowseApply ["DP-1",""]' && shown.left === 0], [() => {
                shown.left = -1;
                shown.heard("DP-1", calm);
            }, () => shown.calm && shown.candidate.title === "Waves"], [() => {
                shown.left = -1;
                shown.press(Qt.Key_Escape);
            }, () => shown.log.slice(-1)[0] === 'BrowseCancel ["DP-1"]' && shown.left === 0], [() => shown.take(empty), () => shown.mode === "vacio" && shown.panel.hints.model[0].text === App.tr("Open the library") && !shown.press(Qt.Key_Left) && shown.press(Qt.Key_Return) && shown.left === 3], [() => shown.fail({
                    "message": "no answer"
                }), () => shown.mode === "error" && shown.failure === "no answer" && shown.panel.hints.model.length === 2], [() => shown.press(Qt.Key_Return), () => shown.failure === "" && shown.live && shown.candidate.title === "Aurora" && shown.log.slice(-1)[0] === 'GetBrowse ["DP-1"]'], [() => {
                shown.left = -1;
                shown.press(Qt.Key_Return);
            }, () => shown.log.slice(-1)[0] === 'BrowseApply ["DP-1",""]' && shown.left === 0], [() => {
                shown.left = -1;
                shown.heard("DP-1", {
                    "output": "DP-1",
                    "active": false,
                    "ended": "expired"
                });
            }, () => shown.left === 0]]

    function start() {
        host.waiting++;
        shown = Qt.createComponent("Preview.qml").createObject(null, {
            "output": "DP-1"
        });
        host.held = host.held.concat([check]);
        running = true;
    }

    function finish(problem) {
        running = false;
        host.waiting--;
        shown.destroy();
        host.held = host.held.filter(c => c !== check);
        if (problem)
            throw new Error("preview window: " + problem);
    }

    interval: 20
    repeat: true
    onTriggered: {
        if (!shown)
            return finish("the window did not load");
        if (!steps[step][1]())
            return ++check.tries > 200 ? finish("step " + step + " (" + shown.mode + ") did not settle") : undefined;
        check.tries = 0;
        if (++step < steps.length)
            return steps[step][0]();
        finish("");
    }
}
