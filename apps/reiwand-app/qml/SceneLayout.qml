import QtQuick
import io.github.reiwand.app

QtObject {
    id: layout

    property real width: 1600
    property real height: 900
    property real foldedHigh: 0
    property real openHigh: 0
    readonly property Workbench bench: Workbench {}
    readonly property Layouts desks: Layouts {}
    property string desk: desks.fresh()
    readonly property var parsed: JSON.parse(desk)
    // Panels a page does not have (a package that is not a scene has no layers, effects or
    // timeline): left out of the tree it draws; it saves only in a place of its own (desks.place).
    property var absent: []
    readonly property string whole: desks.root(desk)
    readonly property string root: absent.reduce((tree, unit) => bench.close(tree, unit) || tree, whole)
    readonly property string usual: desks.usual(desk)
    readonly property var kits: ["scene", "particles", "scripts", "simulation", "animation", "wallpaper"]
    readonly property var layouts: kits.map((kit, at) => ({
                "chosen": {
                    "factory": kit
                },
                "name": BenchWords.layout(kit),
                "root": JSON.stringify(parsed.factory[kit].root),
                "key": "Ctrl+" + (at + 1),
                "own": -1
            })).concat(parsed.own.map((own, at) => ({
                "chosen": {
                    "own": own.name
                },
                "name": own.name,
                "root": JSON.stringify(own.view.root),
                "key": at < 3 ? "Ctrl+" + (at + 7) : "",
                "own": at
            })))
    readonly property var current: layouts.find(item => JSON.stringify(item.chosen) === JSON.stringify(parsed.chosen)) ?? layouts[0]
    property bool held: false
    property string refused: ""
    readonly property var units: ["layers", "assets", "options", "try", "properties", "effects", "timeline", "curves", "problems", "viewas", "inputs", "screens", "sheet", "texts", "cover", "review", "particles", "cost", "scripts", "code", "reference", "console"]
    readonly property real canvasFloor: Look.sideRight
    // The room the canvas wants before sides fold into rails (1366 × 768 folds, 1600 × 900 not).
    readonly property var canvasRoom: [Look.sideRight * 2.5, Look.sideRight * 2.2]
    readonly property real footFloor: Look.canvasBarHeight * 2
    readonly property SceneView view: SceneView {
        layout: layout
    }
    readonly property var measure: ({
            // Each panel's floor: the canvas, the inspector and the sheet are wider than the rest,
            // and the timeline while unfolded needs its whole bar.
            "floors": ["canvas"].concat(units).reduce((all, unit) => Object.assign(all, {
                    [unit]: unit === "canvas" ? [canvasFloor, canvasFloor / 2] : [["properties", "sheet"].includes(unit) ? Look.sideRight : Look.sideLeft, unit === "timeline" ? Math.max(footFloor, openHigh) : footFloor]
                }), {}),
            "fixed": foldedHigh > 0 ? {
                "timeline": foldedHigh
            } : {}
        })
    readonly property string measured: JSON.stringify(view.measureWith(view.rails))
    readonly property var placed: JSON.parse(bench.places(view.drawn, measured, width, height) || "{\"groups\": [], \"edges\": []}")
    readonly property var groups: view.adjust(placed.groups)
    readonly property var edges: view.edges(placed.edges)
    readonly property var closed: JSON.parse(bench.closed(root) || "[]")
    readonly property var canvasBox: groups.find(group => group.panels.includes("canvas")) ?? Qt.rect(0, 0, width, height)
    readonly property real rightWide: Math.max(0, width - canvasBox.x - canvasBox.width)
    readonly property Timer saving: Timer {
        interval: 400
        onTriggered: layout.flush()
    }

    function store(next) {
        if (!next || next === desk)
            return false;
        desk = next;
        saving.restart();
        return true;
    }

    function choose(chosen) {
        return store(desks.choose(desk, JSON.stringify(chosen)));
    }

    function chooseKey(number) {
        const chosen = desks.keyed(desk, number);
        return chosen !== "" && choose(JSON.parse(chosen));
    }

    function saveAs(name) {
        return store(desks.saveAs(desk, name));
    }

    function rename(at, name) {
        return store(desks.rename(desk, at, name));
    }

    function remove(at) {
        return store(desks.remove(desk, at));
    }

    function opening(kind, fresh) {
        return store(desks.opening(desk, kind, fresh));
    }

    function groupOf(unit) {
        return groups.find(group => group.panels.includes(unit) && !group.rail) ?? groups.find(group => group.panels.includes(unit)) ?? null;
    }

    function isShown(unit) {
        return groupOf(unit)?.shown === unit || view.windowOf(unit)?.shown === unit;
    }

    function change(next) {
        if (!next || next === root)
            return false;
        return store(desks.put(desk, next));
    }

    function reveal(unit) {
        return view.rail.includes(unit) ? (view.unfolded = unit) === unit : change(bench.show(root, unit)) || (closed.includes(unit) && !absent.includes(unit) && reopen(unit));
    }

    function canMove(unit, side) {
        return bench.step(root, unit, side) !== "";
    }

    function move(unit, side) {
        return change(bench.step(root, unit, side));
    }

    function stack(unit, beside) {
        return change(bench.stack(root, unit, beside));
    }

    function canClose(unit) {
        return bench.close(root, unit) !== "";
    }

    function close(unit) {
        return change(bench.close(root, unit));
    }

    function reopen(unit) {
        return change(bench.reopen(root, usual, unit));
    }

    function placing(unit, choice) {
        if (!choice)
            return "";
        if (choice.kind === "stack")
            return bench.stack(root, unit, choice.beside);
        if (choice.kind === "side")
            return bench.beside(root, unit, choice.beside, choice.side);
        return bench.edge(root, unit, choice.side);
    }

    function zone(next, unit) {
        const folded = view.railsOf(next);
        const found = JSON.parse(bench.places(next, JSON.stringify(view.measureWith(folded)), width, height) || "{\"groups\": []}");
        const side = folded.find(one => one.panels.includes(unit));
        return side ? view.folding.strip(side, found.groups.filter(group => group.panels.every(one => side.panels.includes(one)))) : found.groups.find(group => group.panels.includes(unit)) ?? null;
    }

    function resize(edge, by, from) {
        return change(bench.resize(from ?? root, measured, JSON.stringify([width, height]), JSON.stringify(edge), by));
    }

    function even(edge) {
        return change(bench.even(root, usual, JSON.stringify(edge)));
    }

    function reset() {
        held = false;
        refused = "";
        desk = desks.reset(desk);
        flush();
    }

    function flush() {
        saving.stop();
        if (held || (absent.length > 0 && desks.place === ""))
            return;
        const said = desks.save(desk);
        if (said)
            console.warn("the editor layout was not saved: " + said);
    }

    function restore() {
        const read = JSON.parse(desks.load());
        held = read.refused !== "";
        refused = read.refused || read.mended;
        desk = read.desk || desks.fresh();
    }

    Component.onCompleted: restore()
    Component.onDestruction: {
        if (saving.running)
            flush();
    }
}
