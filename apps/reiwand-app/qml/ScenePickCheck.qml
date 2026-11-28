import QtQuick
import io.github.reiwand.app

QtObject {
    required property var page

    function inputs(item, out) {
        for (const child of item?.children ?? []) {
            if (typeof child.commit === "function" && child.field !== undefined && child.page === page)
                out.push(child);
            inputs(child, out);
        }
        return out;
    }

    function pointer(id) {
        return (page.view.layers ?? []).find(layer => layer.id === id)?.pointer ?? "";
    }

    function pick(at) {
        page.editor.select(at);
        page.shown.settle();
    }

    function exercise() {
        const steps = page.view.history.done.length;
        page.run("add_layer", {
            "kind": "solid",
            "id": "pick_one"
        });
        page.run("add_layer", {
            "kind": "solid",
            "id": "pick_two"
        });
        page.change(pointer("pick_two") + "/opacity", 0.25);
        pick("/scene/canvas");
        pick(pointer("pick_one"));
        const before = inputs(page, []);
        page.editor.select("/scene/canvas");
        if (page.shown.at === "/scene/canvas")
            throw new Error("pick: the properties were built before the new selection could be drawn");
        page.shown.settle();
        let stale = 0;
        const watch = () => stale += page.fields.some(field => !field.pointer.startsWith(page.editor.selected)) ? 1 : 0;
        page.editor.selectedChanged.connect(watch);
        pick(pointer("pick_two"));
        page.editor.selectedChanged.disconnect(watch);
        const after = inputs(page, []);
        if (stale > 0)
            throw new Error("pick: the selection changed before the properties of the new layer were ready");
        if (before.length === 0 || after.length !== before.length || after.some(input => !before.includes(input)))
            throw new Error("pick: going through the scene to a layer of the same kind built the properties again: " + before.length + " rows before, " + after.filter(input => !before.includes(input)).length + " new");
        if (!after.some(input => input.field.pointer === pointer("pick_two") + "/opacity" && input.value === 0.25))
            throw new Error("pick: the properties do not show the values of the newly selected layer");
        page.run("add_layer", {
            "kind": "text",
            "id": "pick_word"
        });
        pick(pointer("pick_word"));
        const kinds = inputs(page, []);
        pick(pointer("pick_one"));
        pick(pointer("pick_word"));
        if (inputs(page, []).some(input => !kinds.includes(input)))
            throw new Error("pick: coming back to a kind of layer already shown built its rows again");
        Qt.createComponent("SceneGestureCheck.qml").createObject(page, {
            "page": page,
            "inputs": inputs
        }).exercise();
        while (page.view.history.done.length > steps)
            page.run("undo");
    }
}
