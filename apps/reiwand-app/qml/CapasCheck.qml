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
        const running = Capas.onScreen(check.screen, [job("running", 42)]);
        must(Capas.note(running) === App.tr("Generating layers · %1 %", 42), "a running job does not show its percent: " + Capas.note(running));
        must(Math.abs(Capas.bar(running) - 0.42) < 0.001, "the bar of a running job is not at its progress: " + Capas.bar(running));
        const queued = Capas.onScreen(check.screen, [job("queued", 0)]);
        must(Capas.note(queued) === App.tr("Waiting to generate layers") && Capas.bar(queued) === 0, "a queued job does not say it waits: " + Capas.note(queued));
        const failed = Capas.onScreen(still("failed"), []);
        must(Capas.note(failed) === App.tr("Could not generate the layers") && Capas.tone(failed) === "coral" && Capas.bar(failed) < 0, "a failed set is not shown as an error: " + Capas.note(failed));
        must(Capas.onScreen(still(null), [job("running", 10, "/pictures/other.png")]) === null, "a picture whose layers are built keeps a layer note");
        must(Capas.note(Capas.onScreen(still("cancelled"), [])) === "", "a cancelled set shows a note on its cover");
        const cover = Qt.createComponent("Tapa.qml").createObject(frame, {
            "width": Look.tapa,
            "note": Capas.note(running),
            "progress": Capas.bar(running)
        });
        must(Math.abs(cover.bar.value - 0.42) < 0.001 && cover.bar.width > 0, "a cover whose layers are being built shows no bar: value " + cover.bar.value + ", width " + cover.bar.width);
        cover.progress = Capas.bar(failed);
        must(cover.bar.value < 0, "a cover whose layers failed still shows a bar: value " + cover.bar.value);
        cover.destroy();
        const card = Qt.createComponent("AhoraCapas.qml").createObject(frame, {
            "info": check.screen,
            "jobs": [job("running", 42)]
        });
        must(card.note === Capas.note(running) && card.layers.text === card.note, "the screen's card does not show the layer state: " + card.layers.text);
        must(card.bar.value > 0.41, "the screen's card shows no layer bar: value " + card.bar.value);
        card.info = still(null);
        card.jobs = [];
        must(card.note === "" && card.bar.value < 0, "the screen's card keeps the layer state once the layers are built: " + card.note);
        card.destroy();
        const tasks = Qt.createComponent("AhoraTarea.qml").createObject(frame);
        must(tasks.pick([job("running", 42)]).length === 0, "the task line still lists the layer job");
        tasks.destroy();
    }
}
