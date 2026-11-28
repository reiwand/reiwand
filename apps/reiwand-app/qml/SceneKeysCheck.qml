import QtQuick
import io.github.reiwand.app

Timer {
    id: keys

    required property var host
    required property KeyWalk walk
    property bool finished: !App.backend.claim("keys.editor")
    property var page: null
    property int waited: 0
    // The factory layout walked next with Tab (-1 before the walk of the editor's own pieces) and the
    // arrangement to give back after the six.
    property int kit: -1
    property string was: ""
    readonly property SceneEditor helper: SceneEditor {}
    // The regions the Tab key reaches; a panel never shown is not made yet and is left out.
    readonly property var regions: ["SceneGroup", "SceneLayers", "SceneCanvasBar", "SceneOverlays", "SceneFields", "SceneProblems", "SceneFoot"]

    function begin() {
        if (finished)
            return;
        host.work(Qt.createComponent("ScenePage.qml"), {
            "folder": helper.folderFor("keys"),
            "starter": "parallax",
            "heading": "",
            "exercised": false
        });
        page = host.pageStack.currentItem;
        start();
    }

    function found(name) {
        return walk.gather(page, []).find(item => String(item).startsWith(name + "_") || String(item).startsWith(name + "("));
    }

    function tab(at) {
        const home = page.docks.groupItem("assets");
        if (!home)
            throw new Error("no group holds the assets: " + page.layout.root + " in " + JSON.stringify(page.layout.groups) + " (" + page.docks.serial + ")");
        const button = walk.gather(home, []).find(item => item.text === App.tr(["Assets", "Scripts"][at]) && item.checkable !== undefined);
        if (!button)
            throw new Error("the editor tab " + at + " is not in the group of the assets");
        button.forceActiveFocus(Qt.TabFocusReason);
        walk.press(Qt.Key_Space, 0);
        if (page.layout.groupOf("assets").shown !== ["assets", "scripts"][at])
            throw new Error("the Space key on the editor tab " + at + " does not open it");
    }

    function canvas(seen) {
        const overlays = found("SceneOverlays");
        const chosen = page.gizmos.find(gizmo => gizmo.pointer.startsWith("/scene/layers/") && !gizmo.locked);
        page.editor.select(chosen.pointer);
        walk.host.contentItem.forceActiveFocus();
        for (let n = 0; n < seen.length && !overlays.activeFocus; n++)
            walk.press(Qt.Key_Tab, 0);
        if (!overlays.activeFocus || !overlays.marked)
            throw new Error("the Tab key does not reach the canvas with its focus mark");
        const before = JSON.stringify(page.gizmos.find(gizmo => gizmo.pointer === chosen.pointer));
        const steps = page.view.history.done.length;
        walk.press(Qt.Key_Right, 0);
        walk.press(Qt.Key_Down, Qt.ShiftModifier);
        const after = JSON.stringify(page.gizmos.find(gizmo => gizmo.pointer === chosen.pointer));
        if (after === before || page.view.history.done.length !== steps + 2)
            throw new Error("the arrow keys on the canvas do not move the chosen layer");
        walk.press(Qt.Key_Escape, 0);
        if (overlays.activeFocus)
            throw new Error("the Escape key does not leave the canvas");
    }

    // Each of the six layouts, chosen on one tick and walked with Tab on the next, once its groups
    // are made: every control is reached, the canvas, the lists and the foot among them.
    function layouts() {
        const layout = page.layout, name = layout.kits[kit];
        try {
            const seen = walk.check("editor's " + name + " layout");
            const lost = regions.filter(region => found(region)?.visible && !walk.reached(seen, found(region)));
            if (lost.length > 0)
                throw new Error("the Tab key does not reach the " + lost.join(", ") + " of the " + name + " layout");
        } catch (error) {
            layout.store(was);
            throw error;
        }
        if (++kit < layout.kits.length) {
            layout.chooseKey(kit + 1);
            return true;
        }
        layout.store(was);
        return false;
    }

    function next() {
        if (host.pageStack.currentItem !== page)
            throw new Error("the editor page was replaced before its keyboard walk");
        if (kit >= 0)
            return layouts();
        if (!page.editor.ready || (page.gizmos ?? []).length === 0)
            return ++waited < 40;
        page.change("/manifest/title", "");
        if ((page.view.problems ?? []).length === 0)
            throw new Error("the editor's keyboard walk found no problem to reach");
        for (let at = 1; at >= 0; at--) {
            tab(at);
            const seen = walk.check("editor tab " + at);
            const lost = regions.filter(name => found(name)?.visible && !walk.reached(seen, found(name)));
            if (lost.length > 0)
                throw new Error("the Tab key does not reach the editor's " + lost.join(", "));
        }
        page.run("undo");
        canvas(walk.tabs());
        was = page.layout.desk;
        kit = 0;
        page.layout.chooseKey(1);
        return true;
    }

    function finish() {
        stop();
        finished = true;
    }

    interval: 250
    repeat: true
    onTriggered: {
        try {
            if (!next()) {
                if (waited >= 40)
                    throw new Error("the editor did not become ready for its keyboard walk");
                finish();
            }
        } catch (error) {
            finish();
            throw error;
        }
    }
}
