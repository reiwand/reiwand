import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var page

    function fail(what) {
        throw new Error("bounds: " + what);
    }

    function box(item, onto) {
        const at = item.mapToItem(onto, 0, 0);
        return Qt.rect(at.x, at.y, item.width, item.height);
    }

    function meets(a, b) {
        return a.width > 0 && a.height > 0 && a.x < b.x + b.width && b.x < a.x + a.width && a.y < b.y + b.height && b.y < a.y + a.height;
    }

    function within(a, b) {
        return a.x >= b.x - 1 && a.y >= b.y - 1 && a.x + a.width <= b.x + b.width + 1 && a.y + a.height <= b.y + b.height + 1;
    }

    function name(item) {
        return String(item).split("(")[0].replace(/_QMLTYPE_\d+$/, "");
    }

    // Visible items of the page, outside the workbench, that draw over it.
    function strays() {
        const area = page.docks;
        const whole = box(area, page);
        return Array.from(page.children).concat(Array.from(page.contentItem.children)).filter(item => item !== area && item !== page.contentItem && !page.born.includes(item) && item.visible && item.opacity > 0 && meets(box(item, page), whole)).map(name);
    }

    // Visible items of a panel laid out past the body of its group, unless an item between them clips.
    function spills(item, body, found) {
        for (const child of Array.from(item.children)) {
            if (!child.visible || child.opacity === 0)
                continue;
            const at = box(child, body);
            if (child.width > 0 && child.height > 0 && !within(at, Qt.rect(0, 0, body.width, body.height)))
                found.push(name(child) + " at " + [at.x, at.y, at.width, at.height].map(Math.round).join(","));
            else if (!child.clip)
                spills(child, body, found);
        }
        return found;
    }

    function groups(area, walk) {
        const found = [];
        // A rail draws icons only; the panel it unfolds is a group of its own. Each group is drawn by
        // the item of the same place, which a lookup by panel could miss while a side is folded.
        for (let at = 0; at < area.layout.groups.length; at++) {
            const group = area.layout.groups[at], item = area.groups.itemAt(at);
            if (group.rail)
                continue;
            if (!item)
                fail("the group of " + group.shown + " has no item on the workbench");
            if (!item.clip || !item.body.parent.clip)
                fail("the group of " + group.shown + " does not clip what it holds");
            const unit = item.body.children.find(child => child.visible);
            if (!unit || !within(box(unit, item), box(item.body, item)))
                fail("the shown panel " + group.shown + " is not inside the body of its group");
            const spilt = walk ? spills(unit, item.body.parent, []) : [];
            if (spilt.length > 0)
                found.push(group.shown + ": " + spilt.slice(0, 3).join("; "));
        }
        return found;
    }

    // Run once the window has drawn a frame, so every layout has placed its items.
    function settled() {
        const spilt = groups(page.docks, true);
        if (spilt.length > 0)
            fail("content goes past its group: " + spilt.join(" | "));
    }

    function exercise() {
        const area = page.docks;
        const left = strays();
        if (left.length > 0)
            fail("items outside the workbench draw over it: " + left.join(", "));
        const planted = Qt.createComponent("SceneDepthTool.qml").createObject(page, {
            "input": {
                "page": page,
                "field": {
                    "pointer": "/scene/canvas"
                },
                "value": ""
            }
        });
        const seen = strays().length;
        planted.destroy();
        if (seen !== 1)
            fail("a panel left on the page over the workbench was not found");
        groups(area, false);
    }
}
