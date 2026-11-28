import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var frame
    readonly property var screen: ({
            "item": "01J9Z0X9Z6Q7X4F3K5E1V0R1MS",
            "image": "/pictures/hill.png",
            "still": "building"
        })

    function must(ok, text) {
        if (!ok)
            throw new Error("layer state: " + text);
    }

    function job(state, done, target) {
        return {
            "id": 40,
            "kind": "layers",
            "target": target ?? check.screen.image,
            "state": state,
            "done": done,
            "total": 100
        };
    }

    function still(word) {
        return Object.assign({}, check.screen, {
            "still": word
        });
    }

    function exercise() {
        const running = Layers.onScreen(check.screen, [job("running", 42)]);
        must(Layers.note(running) === App.tr("Generating layers · %1 %", 42), "a running job does not show its percent: " + Layers.note(running));
        must(Math.abs(Layers.bar(running) - 0.42) < 0.001, "the bar of a running job is not at its progress: " + Layers.bar(running));
        const queued = Layers.onScreen(check.screen, [job("queued", 0)]);
        must(Layers.note(queued) === App.tr("Waiting to generate layers") && Layers.bar(queued) === 0, "a queued job does not say it waits: " + Layers.note(queued));
        const failed = Layers.onScreen(still("failed"), []);
        must(Layers.note(failed) === App.tr("Could not generate the layers") && Layers.tone(failed) === "coral" && Layers.bar(failed) < 0, "a failed set is not shown as an error: " + Layers.note(failed));
        must(Layers.onScreen(still(null), [job("running", 10, "/pictures/other.png")]) === null, "a picture whose layers are built keeps a layer note");
        must(Layers.note(Layers.onScreen(still("cancelled"), [])) === "", "a cancelled set shows a note on its cover");
        const cover = Qt.createComponent("Cover.qml").createObject(frame, {
            "width": Look.cover,
            "note": Layers.note(running),
            "progress": Layers.bar(running)
        });
        must(Math.abs(cover.bar.value - 0.42) < 0.001 && cover.bar.width > 0, "a cover whose layers are being built shows no bar: value " + cover.bar.value + ", width " + cover.bar.width);
        cover.progress = Layers.bar(failed);
        must(cover.bar.value < 0, "a cover whose layers failed still shows a bar: value " + cover.bar.value);
        cover.destroy();
        const card = Qt.createComponent("NowLayers.qml").createObject(frame, {
            "info": check.screen,
            "jobs": [job("running", 42)]
        });
        must(card.note === Layers.note(running) && card.layers.text === card.note, "the screen's card does not show the layer state: " + card.layers.text);
        must(card.bar.value > 0.41, "the screen's card shows no layer bar: value " + card.bar.value);
        card.info = still(null);
        card.jobs = [];
        must(card.note === "" && card.bar.value < 0, "the screen's card keeps the layer state once the layers are built: " + card.note);
        card.destroy();
        const tasks = Qt.createComponent("NowTask.qml").createObject(frame);
        must(tasks.pick([job("running", 42)]).length === 0, "the task line still lists the layer job");
        tasks.destroy();
    }
}
