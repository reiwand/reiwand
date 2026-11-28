import QtQuick
import io.github.reiwand.app

// Pasting a megabyte with no space into the text of a text layer: it is pasted and applied within
// seconds (wrapping it at word boundaries froze the window for half a minute), text with words
// still wraps between them, and the field takes Ctrl+V before any window shortcut.
QtObject {
    required property var page

    function fail(what) {
        throw new Error("text paste: " + what);
    }

    function field() {
        const input = Qt.createComponent("SceneInput.qml").createObject(page, {
            "page": page,
            "field": page.fields.find(one => one.key === "text"),
            "width": 400
        });
        return [input, CheckKit.find(input, item => item.hasOwnProperty("unbroken"))];
    }

    function exercise() {
        page.run("add_layer", {
            "kind": "text",
            "name": "Paste"
        });
        const at = page.editor.count("/scene/layers") - 1;
        page.editor.select("/scene/layers/" + at);
        const clipboard = Qt.createQmlObject("import QtQuick; TextEdit {}", page);
        const copy = text => {
            clipboard.text = text;
            clipboard.selectAll();
            clipboard.copy();
        };
        let made = field();
        let input = made[0], text = made[1];
        text.area.forceActiveFocus();
        if (!App.backend.claims(Qt.Key_V, Qt.ControlModifier))
            fail("the field does not take Ctrl+V before the window's shortcuts");
        copy("a".repeat(1000000));
        text.area.selectAll();
        const started = Date.now();
        text.paste();
        text.area.editingFinished();
        const took = Date.now() - started;
        const kept = String(JSON.parse(page.editor.value("/scene/layers/" + at + "/text") || "\"\""));
        if (kept.length !== 1000000 || took > 10000)
            fail("a pasted megabyte gave " + kept.length + " characters after " + took + " ms");
        if (text.area.wrapMode !== TextEdit.WrapAnywhere)
            fail("a megabyte with no space is not wrapped anywhere");
        input.destroy();
        made = field();
        input = made[0];
        text = made[1];
        copy("words wrap between them ".repeat(20));
        text.area.selectAll();
        text.paste();
        if (text.area.wrapMode !== TextEdit.Wrap || text.area.text.length !== 480)
            fail("text with words does not wrap between them");
        input.destroy();
        clipboard.destroy();
        // a megabyte step can trim the history, so the layer is removed, not undone
        const id = JSON.parse(page.editor.value("/scene/layers/" + at + "/id"));
        page.run("remove_layer", {
            "id": id
        });
        if (page.editor.count("/scene/layers") !== at)
            fail("the pasted layer was not removed");
    }
}
