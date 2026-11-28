import QtQuick
import QtTest
import io.github.reiwand.app

TestEvent {
    id: events

    required property var page
    property var stage: null

    function made(name, props) {
        if (!stage)
            stage = Qt.createQmlObject("import QtQuick; Window { width: 900; height: 700; visible: true }", page);
        stage.requestActivate();
        const item = Qt.createComponent(name).createObject(stage.contentItem, props);
        if (!item)
            throw new Error(name + " did not load");
        return item;
    }

    function words(menu) {
        return menu.options.filter(option => !option.separator).map(option => option.text);
    }

    function expect(menu, has, lacks, where) {
        const shown = words(menu);
        if (!menu.visible || has.some(text => !shown.includes(text)) || lacks.some(text => shown.includes(text)))
            throw new Error("menu " + where + ": " + (menu.visible ? JSON.stringify(shown) : "did not open"));
    }

    function row(panel, id) {
        const found = CheckKit.find(panel, child => child.modelData?.id === id && child.opensMenu !== undefined);
        if (!found)
            throw new Error("menu: no row " + id);
        return found;
    }

    function layers() {
        return (page.view.layers ?? []).length;
    }

    function three(ids) {
        const before = layers();
        page.run("steps", ids.map(id => ({
                            "add_layer": {
                                "kind": "solid",
                                "id": id
                            }
                        })));
        if (layers() !== before + 3)
            throw new Error("menu: three layers were not added in one go: " + page.editor.error);
        page.picked = [];
        page.editor.select("/scene/canvas");
        return before;
    }

    function removeThree(popup, before, where) {
        expect(popup, [App.tr("Group the layers"), App.tr("Remove")], [App.tr("Rename the layer"), App.tr("Raise")], "of three layers " + where);
        const steps = page.view.history.done.length;
        if (!CheckKit.find(popup.contentItem, child => child.text === App.tr("Remove") && child.clicked !== undefined))
            throw new Error("menu: no Remove entry shown " + where);
        popup.close();
        popup.options.find(option => option.text === App.tr("Remove")).act();
        if (layers() !== before || page.view.history.done.length !== steps + 1)
            throw new Error("menu: remove did not take the three layers in one step " + where + ": " + layers() + page.editor.error);
        page.run("undo");
        if (layers() !== before + 3)
            throw new Error("menu: one undo did not bring the three layers back " + where);
    }

    function list() {
        const panel = made("SceneLayers.qml", {
                "page": page,
                "width": 360,
                "height": 640
            });
        const popup = panel.menu.popup;
        const before = three(["menu_a", "menu_b", "menu_c"]);
        mouseClick(row(panel, "menu_a"), 30, 8, Qt.LeftButton, Qt.NoModifier, -1);
        mouseClick(row(panel, "menu_b"), 30, 8, Qt.LeftButton, Qt.ControlModifier, -1);
        mouseClick(row(panel, "menu_c"), 30, 8, Qt.LeftButton, Qt.ControlModifier, -1);
        if (page.picked.length !== 3)
            throw new Error("menu: Ctrl+click on three rows picked " + JSON.stringify(page.picked));
        mouseClick(row(panel, "menu_b"), 30, 8, Qt.RightButton, Qt.NoModifier, -1);
        if (panel.menu.chosen().length !== 3)
            throw new Error("menu: a right click on one of three picked layers lost the others: " + JSON.stringify(page.picked));
        removeThree(popup, before, "in the list");
        page.picked = [];
        page.editor.select("/scene/canvas");
        mouseClick(row(panel, "menu_a"), 30, 8, Qt.RightButton, Qt.NoModifier, -1);
        if (panel.current?.id !== "menu_a")
            throw new Error("menu: a right click did not select the layer under it");
        expect(popup, [App.tr("Rename the layer"), App.tr("Duplicate"), App.tr("Hide"), App.tr("Lock"), App.tr("Copy the layers"), App.tr("Remove")], [App.tr("Ungroup")], "of one layer");
        popup.close();
        panel.destroy();
    }

    function shrink(gizmo, dx) {
        gizmo.commit("scale", [0.2, 0.2]);
        gizmo.commit("position", [gizmo.modelData.position[0] + dx, gizmo.modelData.position[1]]);
        page.editor.hold(false);
    }

    function canvas() {
        const box = Qt.createQmlObject("import QtQuick; Item { width: 900; height: 700 }", stage.contentItem);
        const before = three(["menu_bg", "menu_l", "menu_r"]);
        const make = id => Qt.createComponent("SceneGizmo.qml").createObject(box, {
                    "page": page,
                    "index": (page.gizmos ?? []).findIndex(g => g.id === id),
                    "ratio": 0.3
                });
        const bg = make("menu_bg"), left = make("menu_l"), right = make("menu_r");
        shrink(left, -400);
        shrink(right, 400);
        const inside = left.mapToItem(bg, left.width / 2, left.height / 2);
        if (!bg.contains(inside))
            throw new Error("menu: the small layer is not over the background: " + JSON.stringify([bg.width, left.x, left.width]));
        mouseClick(bg, 4, 4, Qt.LeftButton, Qt.NoModifier, -1);
        if (page.editor.selected !== bg.modelData.pointer)
            throw new Error("menu: a click on the background did not select it");
        mouseClick(left, left.width / 2, left.height / 2, Qt.LeftButton, Qt.NoModifier, -1);
        if (page.editor.selected !== left.modelData.pointer)
            throw new Error("menu: a click on a layer above the selected background did not select it: " + page.editor.selected);
        mouseClick(right, right.width / 2, right.height / 2, Qt.LeftButton, Qt.ControlModifier, -1);
        mouseClick(bg, 4, 4, Qt.LeftButton, Qt.ControlModifier, -1);
        if (page.picked.length !== 3)
            throw new Error("menu: Ctrl+click on three layers of the canvas picked " + JSON.stringify(page.picked));
        mouseClick(right, right.width / 2, right.height / 2, Qt.RightButton, Qt.NoModifier, -1);
        removeThree(page.layers.menu.popup, before, "on the canvas");
        box.destroy();
    }

    function exercise() {
        list();
        canvas();
        stage.destroy();
    }
}
