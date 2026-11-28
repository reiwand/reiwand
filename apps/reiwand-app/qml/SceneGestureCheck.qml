import QtQuick

QtObject {
    required property var page
    required property var inputs

    function gizmo(id) {
        return page.gizmos.find(g => g.id === id) ?? null;
    }

    function exercise() {
        const steps = page.view.history.done.length;
        page.run("add_layer", {
            "kind": "solid",
            "id": "drag_one"
        });
        const pointer = page.view.layers.find(layer => layer.id === "drag_one").pointer;
        page.editor.select(pointer);
        page.shown.settle();
        const start = gizmo("drag_one").pivot;
        const rows = inputs(page, []);
        const done = page.view.history.done.length;
        const seen = {
            "views": 0,
            "fields": 0,
            "gizmos": 0
        };
        const counters = ["views", "fields", "gizmos"].map(name => () => seen[name] += 1);
        const signals = [page.editor.viewChanged, page.editor.fieldsChanged, page.editor.gizmosChanged];
        signals.forEach((signal, i) => signal.connect(counters[i]));
        page.editor.hold(true);
        for (let step = 1; step <= 12; step++)
            page.run("layer", {
                "grip": {
                    "layer": "drag_one",
                    "grip": "move",
                    "to": [start[0] + step * 5, start[1]]
                }
            }, true);
        const during = Object.assign({}, seen);
        const moved = gizmo("drag_one").pivot;
        page.editor.hold(false);
        signals.forEach((signal, i) => signal.disconnect(counters[i]));
        page.shown.settle();
        if (during.views !== 0 || during.fields !== 0 || during.gizmos !== 12)
            throw new Error("drag: a step of the drag rebuilt the scene outline or the properties: " + JSON.stringify(during));
        if (Math.abs(moved[0] - start[0] - 60) > 0.5 || inputs(page, []).some(input => !rows.includes(input)))
            throw new Error("drag: the handles did not follow the drag, or the drag built properties rows: " + moved);
        if (seen.views !== 1 || page.view.history.done.length !== done + 1 || Math.abs(gizmo("drag_one").pivot[0] - moved[0]) > 0.01)
            throw new Error("drag: the end of the drag did not refresh the scene once into one step: " + JSON.stringify(seen));
        while (page.view.history.done.length > steps)
            page.run("undo");
    }
}
