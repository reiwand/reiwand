import QtQuick
import io.github.reiwand.app

Timer {
    id: check

    required property var host
    property int tries: 0
    property bool moved: false
    property int shows: 0
    readonly property var frame: host.shell.titleFrame

    function must(ok, text) {
        if (!ok)
            throw new Error("now block: " + text);
    }

    function start() {
        host.waiting++;
        host.held = host.held.concat([check]);
        host.showPage(0);
        host.showPassiveNotification("check");
        running = true;
    }

    function empty() {
        const block = Qt.createComponent("Now.qml").createObject(frame, {
            "none": true
        });
        must(block.title === App.tr("You have no wallpapers yet"), "with no wallpaper the block does not say so");
        must(!block.controls.visible, "with no wallpaper the block still offers the controls");
        block.stocked = 7;
        must(block.title === App.tr("You have not set any wallpaper yet"), "with wallpapers in the library and none set the block says the library is empty");
        block.destroy();
        const tasks = Qt.createComponent("NowTask.qml").createObject(frame);
        const failed = (id, target) => ({
                "id": id,
                "kind": "provider_sync",
                "target": target,
                "state": "failed"
            });
        const shown = tasks.pick([failed(10, "daily"), failed(11, "space"), failed(12, "daily"), failed(13, "space"), failed(14, "daily")]).map(job => job.id);
        const running = [1, 2, 3, 4, 5, 6, 7].map(id => ({
                    "id": id,
                    "kind": "install",
                    "target": "p" + id,
                    "state": "running"
                }));
        const capped = tasks.rows(tasks.pick(running)).length;
        tasks.destroy();
        must(capped === 4, "a burst of jobs shows a few lines and counts the rest, not " + capped);
        must(shown.join(",") === "13,14", "a task that keeps failing shows one line per attempt: " + shown.join(","));
        const layers = Qt.createComponent("LayersCheck.qml").createObject(frame, {
            "frame": frame
        });
        layers.exercise();
        layers.destroy();
    }

    function notice() {
        const notices = host.shell.notices;
        must(notices.text === "check" && notices.height > 0, "a notice does not show");
        must(Math.abs(notices.x + notices.width - frame.noticeAt.x) < 1, "a notice left the top right corner");
        must(frame.nowBlock.mapToItem(notices.parent, 0, frame.nowBlock.height).y <= notices.y + 1, "a notice covers the job line");
        const importer = host.pageStack.currentItem?.tools?.dialogs?.importer;
        must(!!importer, "the library has no importer");
        importer.tell({
                "imported": [{
                        "path": "/x/dawn.wpak",
                        "uid": "U1"
                    }],
                "kept": [],
                "failed": []
            }, "Dawn");
        must(notices.text === App.tr("“%1” was added.", "Dawn") && notices.action === App.tr("See"), "one added wallpaper is not named: " + notices.text);
        must(!notices.passive.stacked, "a short notice moves its action to a second row");
        importer.tell({
                "imported": [],
                "kept": [{
                        "path": "/x/old.wpak",
                        "uid": "U2",
                        "version": "2.0.0",
                        "newer": true
                    }, {
                        "path": "/x/same.wpak",
                        "uid": "U3",
                        "version": "1.0.0",
                        "newer": false
                    }],
                "failed": [{
                        "path": "/x/broken.wpak",
                        "error": "the package is damaged"
                    }]
            }, "");
        must(notices.action === App.tr("Details"), "an import with failures does not offer its details: " + notices.text);
        must(notices.passive.stacked, "a long notice keeps its action in the text row");
        notices.callback();
        must(importer.report.visible && importer.report.rows.length === 2, "the details do not list the failed and older files");
        importer.report.close();
        host.hidePassiveNotification();
        importer.receive(["/x/a photo.jpg"]);
        must(importer.opened && importer.paths.length === 1 && importer.paths[0] === "/x/a photo.jpg", "a picture opened with the app does not ask how to add it, as Add does");
        importer.close();
        const before = App.backend.lastError;
        App.backend.lastError = "refused probe\n    at fetch (main.ts:1:1)";
        must(notices.refused.text === "refused probe", "a refusal shows more than its first line: " + notices.refused.text);
        App.backend.lastError = "refused probe";
        must(notices.refusing && Math.abs(notices.x + notices.width - frame.noticeAt.x) < 1, "a refused action does not show as a notice in the notices' place");
        notices.waved = "refused probe";
        must(!notices.refusing, "a refused action's notice does not close");
        App.backend.lastError = before;
    }

    interval: Look.pace * 6
    onTriggered: {
        if (host.kind !== "title" && host.waiting > 1 && ++tries < 200)
            return restart();
        if (host.kind === "options" && !moved) {
            moved = true;
            host.showPage(0);
            const page = host.pageStack.currentItem;
            if (page && page.opened)
                page.opened = "";
            host.showPassiveNotification("check");
            return restart();
        }
        if (host.kind === "title" && host.shell.notices.text !== "check" && ++shows < 5) {
            host.showPassiveNotification("check");
            return restart();
        }
        try {
            if (host.kind === "title") {
                must(frame.bandLow > 0, "the library does not use the low band");
                const tasks = frame.nowBlock.tasks;
                must(tasks.shown.length === 0 || tasks.height > 0 || frame.nowBlock.compact, "the job line is hidden on a page with the band");
                notice();
            }
            empty();
            if (host.kind === "title")
                Qt.createComponent("NowTapCheck.qml").createObject(host, {
                    "host": host
                }).start();
        } finally {
            host.waiting--;
            host.held = host.held.filter(c => c !== check);
        }
    }
}
