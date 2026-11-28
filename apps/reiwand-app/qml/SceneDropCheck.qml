import QtQuick
import io.github.reiwand.app

QtObject {
    required property var page
    required property var checks

    function fail(what) {
        throw new Error("drop targets: " + what);
    }

    function near(one, other) {
        return ["x", "y", "width", "height"].every(key => Math.abs(one[key] - other[key]) <= 1);
    }

    function middle(box) {
        return Qt.point(box.x + box.width / 2, box.y + box.height / 2);
    }

    function aimed(drop, unit, group) {
        drop.carried = "";
        drop.carried = unit;
        drop.spot = middle(group);
        if (JSON.stringify(drop.under?.panels) !== JSON.stringify(group.panels))
            fail(unit + " over " + group.panels + " found no group under it");
        const own = group.panels.includes(unit);
        const stacks = drop.cross.some(target => target.kind === "stack");
        if (stacks === (own || group.panels.includes("canvas")))
            fail(unit + " over " + group.panels + " offered to stack wrongly");
        const sides = drop.cross.filter(target => target.kind === "side").length;
        if (sides !== (own && group.panels.length === 1 ? 0 : 4))
            fail(unit + " over " + group.panels + " offered " + sides + " sides");
        if (drop.edges.some(target => target.kind !== "edge") || drop.edges.length > 4)
            fail("the edges of the window offered " + JSON.stringify(drop.edges));
    }

    function landed(layout, drop, unit, group) {
        aimed(drop, unit, group);
        const before = layout.root;
        for (const target of drop.targets) {
            drop.spot = middle(target);
            if (drop.choice?.key !== target.key || !drop.zone)
                fail(unit + " on the " + target.key + " target of " + group.panels + " painted no zone");
            const [zone, next] = [drop.zone, drop.next];
            drop.settle(unit, next);
            checks.whole(layout, unit + " dropped on " + target.key + " of " + group.panels);
            const now = layout.groupOf(unit);
            if (!now || !layout.isShown(unit) || !near(now, zone))
                fail(unit + " on " + target.key + " did not take the painted zone " + JSON.stringify(zone) + ": " + JSON.stringify(now));
            if (target.kind === "edge" && Math.abs((target.side === "left" || target.side === "right" ? now.height : now.width) - (target.side === "left" || target.side === "right" ? layout.height : layout.width)) > 1)
                fail(unit + " on the " + target.side + " edge does not run along the whole window");
            layout.change(before);
            aimed(drop, unit, group);
        }
        drop.spot = Qt.point(group.x + Look.e2, group.y + group.height - Look.e2);
        const should = !group.panels.includes(unit) && !group.panels.includes("canvas");
        if ((drop.choice?.kind === "stack") !== should)
            fail(unit + " let go over " + group.panels + " away from the targets chose " + JSON.stringify(drop.choice));
        if (should) {
            drop.settle(unit, drop.next);
            if (layout.groupOf(unit).panels.length !== group.panels.length + 1)
                fail(unit + " let go over " + group.panels + " did not stack: " + layout.root);
            layout.change(before);
        }
        drop.carried = "";
    }

    function closing(layout) {
        const count = layout.groups.length;
        // A layout keeps closed the panels its work does not use; the others close and come back.
        const shut = layout.closed.slice();
        const open = layout.units.filter(unit => !shut.includes(unit));
        for (const unit of open) {
            const usual = layout.groupOf(unit).panels.slice().sort();
            if (!layout.canClose(unit) || !layout.close(unit) || layout.groupOf(unit) || !layout.closed.includes(unit))
                fail(unit + " did not close");
            checks.whole(layout, "closing " + unit);
            if (!layout.reopen(unit) || layout.closed.length !== shut.length || JSON.stringify(layout.groupOf(unit).panels.slice().sort()) !== JSON.stringify(usual))
                fail(unit + " did not come back to its usual group: " + layout.root);
            checks.whole(layout, "opening " + unit + " again");
        }
        if (layout.canClose("canvas") || layout.close("canvas"))
            fail("the canvas closed");
        for (const unit of layout.units)
            layout.close(unit);
        if (layout.groups.length !== 1 || layout.closed.length !== layout.units.length)
            fail("closing every panel left " + layout.root);
        for (const unit of open)
            layout.reopen(unit);
        checks.whole(layout, "opening every panel again");
        if (layout.groups.length !== count)
            fail("the panels did not come back to their groups: " + layout.root);
    }

    function menu(layout) {
        const tabs = page.docks.groupItem("assets").menuOf("assets");
        tabs.popup();
        const others = tabs.others.map(group => group.panels[0]).sort();
        if (JSON.stringify(others) !== JSON.stringify(["layers", "properties", "timeline"]))
            fail("Stack with… offered " + others);
        tabs.stack("properties");
        tabs.dismiss();
        if (JSON.stringify(layout.groupOf("assets").panels) !== JSON.stringify(["properties", "effects", "assets"]))
            fail("Stack with… did not stack: " + layout.root);
        const again = page.docks.groupItem("assets").menuOf("assets");
        again.shut();
        if (!layout.closed.includes("assets") || layout.groupOf("properties").panels.length !== 2)
            fail("Close the panel did not close it: " + layout.root);
    }

    function exercise() {
        const layout = page.layout;
        const drop = page.docks.drop;
        layout.reset();
        for (const unit of ["assets", "properties", "problems"])
            for (const group of layout.groups.slice())
                landed(layout, drop, unit, group);
        layout.reset();
        closing(layout);
        layout.reset();
        menu(layout);
        layout.reset();
    }
}
