import QtQuick
import io.github.reiwand.app

// How the sides of the workbench fold into rails: which nodes are sides, what each takes at least,
// the strip a folded side becomes and where its panel unfolds.
QtObject {
    required property var view

    // What a side takes at least the way it folds: its widest panel across a row, a foot bar down a
    // column. Only fixed floors count, so folding a side never makes it unfold again.
    function floorOf(side) {
        if (side.dir !== "row")
            return view.layout.footFloor;
        return Math.max(0, ...side.panels.map(unit => view.layout.measure.floors[unit]?.[0] ?? 0));
    }

    // The nodes beside the canvas at every level of the tree, with the way they fold and their side.
    function sides(tree) {
        const out = [];
        for (let node = tree; node.split; ) {
            const nodes = node.split.nodes;
            const at = nodes.findIndex(child => view.panelsOf(child).includes("canvas"));
            if (at < 0)
                break;
            nodes.forEach((child, index) => {
                if (index !== at)
                    out.push({
                        "panels": view.panelsOf(child),
                        "dir": node.split.dir,
                        "side": node.split.dir === "row" ? (index < at ? "left" : "right") : (index < at ? "top" : "bottom")
                    });
            });
            node = nodes[at];
        }
        return out;
    }

    function strip(side, held) {
        if (held.length === 0)
            return null;
        const x = Math.min(...held.map(group => group.x)), y = Math.min(...held.map(group => group.y));
        return {
            "rail": true,
            "side": side.side,
            "panels": side.panels,
            "shown": side.panels.includes(view.unfolded) ? view.unfolded : "",
            "x": x,
            "y": y,
            "width": Math.max(...held.map(group => group.x + group.width)) - x,
            "height": Math.max(...held.map(group => group.y + group.height)) - y
        };
    }

    function over(home) {
        const across = home.side === "left" || home.side === "right";
        const width = across ? Math.min(Look.sideLeft * 1.3, view.layout.width * 0.4) : home.width;
        const height = across ? home.height : Math.min(Look.sideLeft * 1.3, view.layout.height * 0.5);
        return {
            "overlay": true,
            "panels": [view.unfolded],
            "shown": view.unfolded,
            "x": home.side === "left" ? home.x + home.width : home.side === "right" ? home.x - width : home.x,
            "y": home.side === "top" ? home.y + home.height : home.side === "bottom" ? home.y - height : home.y,
            "width": width,
            "height": height
        };
    }
}
