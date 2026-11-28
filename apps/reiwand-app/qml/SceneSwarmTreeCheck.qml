import QtQuick
import io.github.reiwand.app

// The bar of the Particles panel: "+ Module" adds a part to the system in hand and opens it in the
// Inspector, "+ System" adds a system and opens it, and the panel lists the layers that draw the
// system.
QtObject {
    required property var page

    function fail(what) {
        throw new Error("particles panel: " + what);
    }

    function exercise() {
        const steps = page.view.history.done.length;
        const tree = Qt.createComponent("SceneSwarmTree.qml").createObject(page, {
            "page": page,
            "width": 400,
            "height": 600
        });
        page.editor.select(SceneSwarmKnobs.pointer("flakes", ""));
        const bar = CheckKit.find(tree, item => item.objectName === "addModule")?.parent;
        if (!bar || !bar.menu)
            fail("there is no + Module in the bar");
        const forces = () => (tree.parsed.forces ?? []).length;
        const before = forces();
        CheckKit.find(bar, item => item.objectName === "addModule").clicked();
        if (!bar.menu.opened)
            fail("+ Module did not open its menu");
        const at = bar.modules.findIndex(module => module[0] === App.tr("Add a force"));
        bar.menu.itemAt(at).triggered();
        if (forces() !== before + 1)
            fail("Add a force left " + forces() + " forces of " + (before + 1));
        if (page.editor.selected !== SceneSwarmKnobs.pointer("flakes", "/forces/" + before))
            fail("the new force did not open in the Inspector: " + page.editor.selected);
        if (tree.users.length !== 1 || tree.users[0].id !== "flakes")
            fail("the layers that use the system are " + JSON.stringify(tree.users));
        const ids = () => Object.keys(JSON.parse(page.editor.value("/scene/particles") || "null") ?? {});
        const systems = ids().length;
        CheckKit.find(bar, item => item.objectName === "addSystem").clicked();
        if (ids().length !== systems + 1 || tree.system === "flakes" || !ids().includes(tree.system))
            fail("+ System did not add a system and open it: " + tree.system);
        if (tree.users.length !== 0)
            fail("a new system is drawn by " + JSON.stringify(tree.users));
        while (page.view.history.done.length > steps)
            page.run("undo");
        tree.destroy();
    }
}
