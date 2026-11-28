import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var page
    readonly property SceneFit fit: SceneFit {}

    function made(file, args, width) {
        return Qt.createComponent(file).createObject(page, Object.assign({
            "page": page,
            "width": width,
            "height": 900
        }, args));
    }

    function chosen(pointer, wanted) {
        page.editor.select(pointer);
        const side = made("SceneFields.qml", {}, Look.sideRight);
        try {
            fit.fits("normal " + pointer, side, wanted);
        } finally {
            side.destroy();
        }
    }

    function normal() {
        const before = page.editor.selected;
        chosen("/scene/canvas", [App.tr("Bright parts glow"), App.tr("Layers move in depth"), App.tr("Light around"), App.tr("Light from above")]);
        const layer = (page.view.layers ?? []).find(item => /^\/scene\/layers\/\d+$/.test(item.pointer));
        if (!layer)
            throw new Error("normal: the scene has no layer to choose");
        chosen(layer.pointer, [layer.id, SceneWords.kind(layer.type)]);
        page.editor.select(before);
    }

    function seeAs() {
        const panel = made("ScenePreviewPanel.qml", {}, Look.smallPanel - 2 * Look.e3);
        try {
            fit.fits("see as", panel, [App.tr("Device"), App.tr("On battery"), App.tr("Reduced motion"), App.tr("Restart the preview"), App.tr("Effects on this computer")].concat(SceneLook.names));
        } finally {
            panel.destroy();
        }
    }

    function error() {
        const layer = (page.view.layers ?? [])[0];
        const pointer = layer.pointer + "/opacity";
        const row = made("SceneProblemRow.qml", {
            "problem": {
                "pointer": pointer,
                "message": pointer + ": the opacity is out of range",
                "warning": false,
                "fix": {
                    "set": {
                        "pointer": pointer,
                        "value": 1
                    }
                }
            }
        }, Look.sideRight - Look.e4 - Look.e3 - Look.line);
        try {
            fit.fits("error", row, [App.tr("%1: %2", layer.id, "the opacity is out of range"), App.tr("Go"), App.tr("Fix")]);
        } finally {
            row.destroy();
        }
        const list = made("SceneProblems.qml", {}, Look.sideRight - Look.e4 - Look.e3 - Look.line);
        const count = (page.view.problems ?? []).length;
        try {
            fit.fits("problems", list, [count > 0 ? App.tr("Problems (%1)", count) : App.tr("No problems")]);
        } finally {
            list.destroy();
        }
    }

    // The time row at the foot of the canvas: live or paused, a new seed for the preview only, a
    // capture and the time the canvas sees.
    function timeRow() {
        const row = made("SceneCanvasTime.qml", {}, Look.sideRight * 3);
        const before = SceneLook.seed;
        try {
            fit.fits("time row", row, [App.tr("Live"), App.tr("Paused at the playhead"), App.tr("New seed"), App.tr("Capture")]);
            SceneLook.newSeed(page);
            const look = JSON.parse(page.editor.look || "{}");
            if (SceneLook.seed <= 0 || look.seed !== SceneLook.seed)
                throw new Error("time row: New seed did not reach the preview: " + page.editor.look);
            if (JSON.parse(page.editor.value("/scene/clock") || "{}").seed === SceneLook.seed)
                throw new Error("time row: New seed changed the seed of the scene");
        } finally {
            SceneLook.seed = before;
            SceneLook.preview(page);
            row.destroy();
        }
    }

    function exercise() {
        normal();
        seeAs();
        error();
        timeRow();
    }
}
