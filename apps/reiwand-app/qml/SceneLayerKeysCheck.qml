import QtQuick
import io.github.reiwand.app

QtObject {
    id: events

    required property var page

    function must(ok, what) {
        if (!ok)
            throw new Error("layer keys: " + what);
    }

    function key(code) {
        App.backend.press(code, 0);
    }

    function inside(item, owner) {
        for (let at = item; at; at = at.parent)
            if (at === owner)
                return true;
        return false;
    }

    function exercise() {
        const stage = page.Window.window;
        const panel = Qt.createComponent("SceneLayers.qml").createObject(stage.contentItem, {
                "z": 100,
                "page": page,
                "width": 360,
                "height": 640
            });
        page.run("steps", ["keys_a", "keys_b", "keys_c"].map(id => ({
                        "add_layer": {
                            "kind": "solid",
                            "id": id
                        }
                    })));
        page.picked = [];
        page.editor.select("/scene/canvas");
        const list = panel.list, shown = () => panel.shown.map(layer => layer.id);
        list.forceActiveFocus(Qt.TabFocusReason);
        must(list.activeFocus, "the list does not take the focus");
        key(Qt.Key_Down);
        must(panel.current?.id === shown()[0], "Down from nothing did not select the first row: " + panel.current?.id);
        key(Qt.Key_Down);
        key(Qt.Key_Down);
        must(panel.current?.id === shown()[2], "two Down did not reach the third row: " + panel.current?.id);
        must(list.currentItem?.ringed === true, "the row reached shows no focus ring");
        key(Qt.Key_Up);
        must(panel.current?.id === shown()[1], "Up did not go back one row");
        key(Qt.Key_End);
        must(panel.current?.id === shown()[shown().length - 1], "End did not reach the last row");
        key(Qt.Key_Home);
        must(panel.current?.id === "keys_c", "Home did not reach the top row: " + panel.current?.id);
        const count = shown().length, steps = page.view.history.done.length;
        key(Qt.Key_Delete);
        must(shown().length === count - 1 && !shown().includes("keys_c") && page.view.history.done.length === steps + 1, "Delete did not remove the row in one step: " + page.editor.error);
        page.run("undo");
        must(shown().includes("keys_c"), "undo did not bring the removed row back");
        panel.reach(0);
        list.forceActiveFocus(Qt.TabFocusReason);
        let stops = 0;
        key(Qt.Key_Tab);
        while (inside(stage.activeFocusItem, list) && stops < 40) {
            stops++;
            key(Qt.Key_Tab);
        }
        must(stops === 3, "Tab does not leave the list after the three buttons of its row: " + stops + " stops");
        page.run("steps", ["keys_a", "keys_b", "keys_c"].map(id => ({
                        "remove_layer": {
                            "id": id
                        }
                    })));
        panel.destroy();
    }
}
