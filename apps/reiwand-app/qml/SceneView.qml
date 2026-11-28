import QtQuick
import QtCore as Core
import io.github.reiwand.app

// What the workbench shows that is not saved: the panel enlarged to the whole workbench (F11), the
// panels in windows of their own, and the sides folded into rails when the canvas would go below
// its floor, with the panel of a rail unfolded over the workbench. Only the size of each window is
// kept (`editor.conf`, `[windows] sizes`), for the next time the same panels are detached.
QtObject {
    id: view

    required property var layout
    property string maximised: ""
    // Each window holds one panel or a whole group, with the panel it shows.
    property var windows: []
    readonly property var detached: windows.reduce((all, window) => all.concat(window.units), [])
    property var sizes: ({})
    property var saved: null
    readonly property Component keeper: Component {
        Core.Settings {
            location: Core.StandardPaths.writableLocation(Core.StandardPaths.ConfigLocation) + "/reiwand/editor.conf"
            category: "windows"
        }
    }
    property string unfolded: ""
    // Checks that move every panel everywhere turn folding off; the rails have a check of their own.
    property bool folds: true
    readonly property int railWidth: DeskLook.strip + Look.e1
    readonly property SceneFolds folding: SceneFolds {
        view: view
    }
    readonly property string drawn: detached.reduce((tree, unit) => layout.bench.without(tree, unit) || tree, layout.root)
    // The sides that fold, one at a time, while the canvas would have less than the room it wants
    // with every side at its floor: first the way it is short (the narrowest side across, the foot
    // down), then any other. Only the size of the window folds a side, never a dragged edge.
    readonly property var rails: railsOf(drawn)
    readonly property var rail: rails.reduce((all, side) => all.concat(side.panels), [])

    function measureWith(folded) {
        return Object.assign({}, layout.measure, {
            "rails": folded.map(side => ({
                        "panels": side.panels,
                        "dir": side.dir
                    })),
            "rail_width": railWidth
        });
    }

    function railsOf(tree) {
        if (!folds || maximised !== "" || layout.width <= 0 || layout.height <= 0)
            return [];
        const all = folding.sides(JSON.parse(tree || "{}"));
        const room = layout.canvasRoom ?? [0, 0];
        const folded = [];
        const left = way => (way === "row" ? layout.width : layout.height) - all.filter(side => side.dir === way).reduce((sum, side) => sum + (folded.includes(side) ? railWidth : folding.floorOf(side)), 0);
        for (let turn = 0; turn < all.length; turn++) {
            const way = left("row") < room[0] ? "row" : left("column") < room[1] ? "column" : "";
            const open = all.filter(side => side.dir === way && !folded.includes(side));
            const next = open.reduce((best, side) => best && folding.floorOf(best) <= folding.floorOf(side) ? best : side, null);
            if (!next)
                break;
            folded.push(next);
        }
        return folded;
    }

    function panelsOf(node) {
        return node.group ? node.group.panels : node.split.nodes.reduce((all, child) => all.concat(panelsOf(child)), []);
    }

    function maximise(unit) {
        maximised = maximised === unit || unit === "" ? "" : unit;
    }

    function detach(unit) {
        return detachAll([unit]);
    }

    function detachAll(units) {
        const leaving = units.filter(unit => !detached.includes(unit));
        if (leaving.length === 0 || leaving.length !== units.length)
            return false;
        maximised = leaving.includes(maximised) ? "" : maximised;
        unfolded = leaving.includes(unfolded) ? "" : unfolded;
        const shown = layout.groupOf(units[0])?.shown;
        windows = windows.concat([
            {
                "units": leaving,
                "shown": leaving.includes(shown) ? shown : leaving[0]
            }
        ]);
        return true;
    }

    function windowOf(unit) {
        return windows.find(window => window.units.includes(unit)) ?? null;
    }

    function showIn(unit) {
        windows = windows.map(window => window.units.includes(unit) ? Object.assign({}, window, {
                "shown": unit
            }) : window);
    }

    // The size of the window of `units`, kept for this session and, outside the checks, the next.
    function remember(units, size) {
        sizes = Object.assign({}, sizes, {
            [units.join("+")]: size
        });
        saved?.setValue("sizes", JSON.stringify(sizes));
        saved?.sync();
    }

    // One panel goes back to the workbench; its window closes once it holds none.
    function attach(unit, size) {
        const home = windowOf(unit);
        if (home && size)
            remember(home.units, size);
        windows = windows.map(window => {
            const units = window.units.filter(other => other !== unit);
            return {
                "units": units,
                "shown": units.includes(window.shown) ? window.shown : units[0] ?? ""
            };
        }).filter(window => window.units.length > 0);
    }

    function attachAll(unit, size) {
        const home = windowOf(unit);
        if (home)
            home.units.forEach(other => attach(other, size));
    }

    function unfold(unit) {
        unfolded = unfolded === unit || unit === "" ? "" : unit;
    }

    // The groups as drawn: only the enlarged panel, or each rail in place of its folded side and
    // the unfolded panel over the workbench, next to its rail.
    function adjust(groups) {
        if (maximised !== "") {
            const home = groups.find(group => group.panels.includes(maximised));
            if (home)
                return [Object.assign({}, home, {
                        "panels": [maximised],
                        "shown": maximised,
                        "x": 0,
                        "y": 0,
                        "width": layout.width,
                        "height": layout.height
                    })];
        }
        if (rails.length === 0)
            return groups;
        const strips = rails.map(side => folding.strip(side, groups.filter(group => group.panels.every(unit => side.panels.includes(unit))))).filter(one => one);
        const kept = groups.filter(group => !group.panels.every(unit => rail.includes(unit)));
        const home = strips.find(one => one.panels.includes(unfolded));
        return strips.concat(kept, home ? [folding.over(home)] : []);
    }

    // The edges beside a rail do not move.
    function edges(found) {
        if (maximised !== "")
            return [];
        const strips = adjust(layout.placed.groups).filter(group => group.rail);
        const beside = edge => strips.some(one => edge.dir === "row" ? [one.x, one.x + one.width].some(x => Math.abs(edge.x - x) < 1.5) && edge.y < one.y + one.height && edge.y + edge.length > one.y : [one.y, one.y + one.height].some(y => Math.abs(edge.y - y) < 1.5) && edge.x < one.x + one.width && edge.x + edge.length > one.x);
        return found.filter(edge => !beside(edge));
    }

    Component.onCompleted: {
        if (App.checking)
            return;
        saved = keeper.createObject(view);
        try {
            const kept = JSON.parse(saved.value("sizes", "{}"));
            const fine = size => Array.isArray(size) && size.length === 2 && size.every(n => Number.isFinite(n) && n > 0);
            const usable = {};
            for (const units in kept)
                if (fine(kept[units]))
                    usable[units] = kept[units];
            sizes = usable;
        } catch (e) {
            sizes = {};
        }
    }
}
