import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var page
    // Given by SceneCheck, the only one that reaps (after the page that opened it returned).
    property var reap: () => {}

    function fail(what) {
        throw new Error("docks: " + what);
    }

    function whole(layout, what) {
        // Each move rebuilds the groups; let the ones it dropped go now, not at the end of hundreds of moves.
        reap();
        const all = [].concat(...layout.groups.map(group => group.panels), layout.closed);
        const every = ["canvas"].concat(layout.units);
        if (all.length !== every.length || every.some(unit => all.indexOf(unit) !== all.lastIndexOf(unit) || !all.includes(unit)))
            fail(what + " lost or doubled a panel: " + layout.root);
        if (layout.groups.some(group => !group.rail && !group.panels.includes(group.shown)))
            fail(what + " left a group that shows none of its panels: " + layout.root);
        const canvas = layout.groupOf("canvas");
        const beside = layout.groups.filter(group => group !== canvas && group.y < canvas.y + canvas.height && canvas.y < group.y + group.height);
        if (canvas.panels.length !== 1 || (canvas.width < layout.canvasFloor - 1 && beside.some(group => group.width > Look.sideRight + 1)))
            fail(what + " squeezed or shared the canvas: " + JSON.stringify(canvas));
        const area = layout.groups.reduce((sum, group) => sum + group.width * group.height, 0);
        if (Math.abs(area - layout.width * layout.height) > layout.width + layout.height)
            fail(what + " did not fill the editor of " + layout.width + " x " + layout.height + ": " + JSON.stringify(layout.groups));
    }

    function followed(area, what) {
        const layers = page.layers;
        const home = area.groupItem("layers");
        if (layers.parent.parent !== home.body || (home.current === "layers" && layers.width !== home.body.width))
            fail("the layers list did not follow its tab " + what);
    }

    function walked(area, layout) {
        for (const unit of layout.units)
            for (const side of ["left", "right", "up", "down"]) {
                const before = layout.root;
                const could = layout.canMove(unit, side);
                if (layout.move(unit, side) !== could)
                    fail(unit + " moved " + side + " against what its menu offered");
                whole(layout, unit + " moved " + side);
                if (could && (layout.root === before || !layout.isShown(unit)))
                    fail(unit + " did not move " + side + " or is not shown there");
                followed(area, "after " + unit + " moved " + side);
            }
        for (const unit of layout.units)
            for (const beside of layout.groups.map(group => group.panels[0]))
                if (layout.stack(unit, beside)) {
                    whole(layout, unit + " stacked with " + beside);
                    if (layout.groupOf(unit) !== layout.groupOf(beside) && JSON.stringify(layout.groupOf(unit)) !== JSON.stringify(layout.groupOf(beside)))
                        fail(unit + " did not land with " + beside);
                }
        if (layout.stack("layers", "canvas") || layout.stack("canvas", "layers"))
            fail("the canvas was shared");
        const home = layout.root;
        for (const [there, back] of [["left", "right"], ["right", "left"], ["down", "up"]]) {
            if (layout.canMove("canvas", there) && layout.move("canvas", there)) {
                whole(layout, "the canvas moved " + there);
                if (!layout.move("canvas", back) || layout.root !== home)
                    fail("the canvas did not come back after moving " + there + ": " + layout.root);
            }
        }
    }

    function edges(layout) {
        for (let at = 0; at < layout.edges.length; at++) {
            const edge = layout.edges[at];
            if (edge.fixed)
                continue;
            for (const by of [-100000, 100000]) {
                layout.resize(edge, by);
                whole(layout, "the edge " + at + " dragged by " + by);
            }
            layout.even(layout.edges[at]);
        }
        const left = layout.groupOf("assets");
        layout.resize(layout.edges[0], -100000);
        if (layout.groupOf("assets").width !== Look.sideLeft)
            fail("the left side went past its floor: " + layout.groupOf("assets").width);
        layout.even(layout.edges[0]);
        if (layout.groupOf("assets").width !== left.width)
            fail("a double click did not return the edge to its place");
    }

    function exercise() {
        const layout = page.layout;
        layout.view.folds = false;
        try {
            moves(layout);
        } finally {
            layout.view.folds = true;
        }
    }

    function moves(layout) {
        layout.reset();
        whole(layout, "the usual layout");
        const right = layout.groupOf("properties"), below = layout.groupOf("timeline"), canvas = layout.groupOf("canvas");
        if (layout.groups.length !== 5 || JSON.stringify(layout.groupOf("assets").panels) !== JSON.stringify(["assets", "scripts"]) || layout.groupOf("assets").x !== 0 || layout.groupOf("layers").x !== right.x || layout.groupOf("layers").y <= right.y || below.y <= canvas.y || below.x !== canvas.x || JSON.stringify(below.panels) !== JSON.stringify(["timeline", "problems"]))
            fail("the Scene layout does not have Assets on the left, Properties over Layers on the right and the timeline under the canvas: " + layout.root);
        walked(page.docks, layout);
        layout.reset();
        const wide = Qt.createComponent("SceneLayout.qml").createObject(page, {
            "width": 1600,
            "height": 900
        });
        wide.close("properties");
        wide.close("effects");
        wide.close("layers");
        if (wide.rightWide !== 0 || wide.groups.length !== 3)
            fail("an empty side stayed open");
        wide.reopen("properties");
        if (wide.rightWide < Look.sideRight)
            fail("the panel did not open again on the right");
        wide.reset();
        const usual = wide.groups;
        edges(wide);
        const near = (one, other) => ["x", "y", "width", "height"].every(key => Math.abs(one[key] - other[key]) <= 1);
        if (wide.groups.length !== usual.length || wide.groups.some((group, at) => !near(group, usual[at])))
            fail("the edges did not come back to their places: " + JSON.stringify(wide.groups));
        wide.destroy();
        const narrow = Qt.createComponent("SceneLayout.qml").createObject(page, {
            "width": 800,
            "height": 548
        });
        whole(narrow, "a narrow editor");
        if (narrow.canvasBox.width < narrow.canvasFloor - 1)
            fail("the canvas went below its floor in a narrow editor: " + JSON.stringify(narrow.groups));
        narrow.destroy();
        layout.reveal("layers");
        Qt.createComponent("SceneDropCheck.qml").createObject(page, {
            "page": page,
            "checks": check
        }).exercise();
        Qt.createComponent("SceneBenchCheck.qml").createObject(page, {
            "page": page
        }).exercise();
        Qt.createComponent("SceneLayoutsCheck.qml").createObject(page, {
            "page": page,
            "checks": check
        }).exercise();
        Qt.createComponent("SceneWindowMenuCheck.qml").createObject(page, {
            "page": page
        }).exercise();
        Qt.createComponent("SceneBenchKeysCheck.qml").createObject(page, {
            "page": page
        }).exercise();
        Qt.createComponent("SceneViewCheck.qml").createObject(page, {
            "page": page
        }).exercise();
        Qt.createComponent("SceneTonesCheck.qml").createObject(page, {
            "page": page
        }).exercise();
        Qt.createComponent("SceneCoverScreensCheck.qml").createObject(page, {
            "page": page
        }).exercise();
        Qt.createComponent("SceneTypeCheck.qml").createObject(page, {
            "page": page
        }).exercise();
    }
}
