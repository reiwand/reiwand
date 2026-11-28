import QtQuick
import io.github.reiwand.app

QtObject {
    id: rows

    readonly property var uneven: [{
            "name": "notice-close",
            "why": "the cross that closes a notice is the small icon size of the kit; it stays centred on the row"
        }]
    property var cuts: []

    function shown(item) {
        let top = item;
        for (let at = item; at; at = at.parent) {
            if (!at.visible || at.opacity === 0)
                return false;
            top = at;
        }
        return item.width > 0 && item.height > 0 && meets(box(item), box(top));
    }

    function button(item) {
        return item.checkable !== undefined && item.down !== undefined && item.icon !== undefined && item.display !== undefined && !item.indicator && item.menu === undefined;
    }

    function named(item) {
        return item.text || item.icon?.name || item.objectName || String(item).replace(/\(0x[0-9a-f]+\)/, "");
    }

    function box(item) {
        return item.mapToItem(null, 0, 0, item.width, item.height);
    }

    function inside(a, b) {
        return a.x >= b.x - 0.5 && a.y >= b.y - 0.5 && a.x + a.width <= b.x + b.width + 0.5 && a.y + a.height <= b.y + b.height + 0.5;
    }

    function meets(a, b) {
        return a.x < b.x + b.width - 0.5 && b.x < a.x + a.width - 0.5 && a.y < b.y + b.height - 0.5 && b.y < a.y + a.height - 0.5;
    }

    function level(item) {
        let icon = null;
        let label = null;
        for (const part of item.contentItem?.children ?? []) {
            if (!part.visible || part.width <= 0)
                continue;
            if (part.sourceSize !== undefined && part.source !== undefined)
                icon = part;
            else if (part.text !== undefined && part.font !== undefined && part.text !== "")
                label = part;
        }
        if (!icon || !label)
            return "";
        const off = Math.abs(icon.mapToItem(item, 0, icon.height / 2).y - label.mapToItem(item, 0, label.height / 2).y);
        return off > 1 ? "the icon of “" + named(item) + "” is " + off.toFixed(1) + " px off its text" : "";
    }

    function lines(found) {
        const out = [];
        for (const item of found.sort((a, b) => a.y - b.y)) {
            const line = out.find(group => group.some(other => item.y < other.y + other.height && other.y < item.y + item.height));
            line ? line.push(item) : out.push([item]);
        }
        return out.filter(line => line.length > 1);
    }

    function row(line, parent, where) {
        const texts = line.map(named);
        const out = [];
        const tall = line.filter(item => !uneven.some(entry => entry.name === named(item))).map(item => item.height);
        const centres = line.map(item => item.mapToItem(parent, 0, item.height / 2).y);
        if (Math.max(...tall) - Math.min(...tall) > 0.5)
            out.push("heights " + tall.join(", "));
        if (Math.max(...centres) - Math.min(...centres) > 1)
            out.push("centres " + centres.map(c => c.toFixed(1)).join(", "));
        line.map(level).filter(text => text !== "").forEach(text => out.push(text));
        if (out.length === 0)
            return [];
        return ["row on " + where + " [" + texts.join(" / ") + "]: " + out.join("; ")];
    }

    function scan(item, where, out) {
        const found = [];
        for (const child of item.children) {
            if (button(child) && shown(child))
                found.push(child);
            scan(child, where, out);
        }
        for (const line of lines(found))
            out.push(...row(line, item, where));
        return out;
    }

    function ringOf(item, depth) {
        for (const child of item.children) {
            if (child.corner !== undefined && child.border !== undefined && child.visible)
                return child;
            const deeper = depth < 3 ? ringOf(child, depth + 1) : null;
            if (deeper)
                return deeper;
        }
        return null;
    }

    function paints(item) {
        if (item.corner !== undefined || !shown(item))
            return false;
        if (button(item))
            return true;
        if (item.radius !== undefined && item.border !== undefined && item.color !== undefined)
            return item.color.a > 0 || item.border.width > 0;
        if (item.font !== undefined && item.text !== undefined)
            return item.text !== "";
        return item.fillMode !== undefined && item.status === Image.Ready;
    }

    function covered(item, ring, own) {
        let at = item;
        for (let up = 0; up < 3 && at.parent; up++, at = at.parent) {
            const kids = at.parent.children;
            let past = false;
            for (const other of kids) {
                if (other === at) {
                    past = true;
                    continue;
                }
                const above = other.z > at.z || other.z === at.z && past;
                if (!above || !paints(other))
                    continue;
                const area = box(other);
                if (meets(area, ring) && !inside(area, own))
                    return other;
            }
        }
        return null;
    }

    function ring(item, where) {
        const drawn = item ? ringOf(item, 0) : null;
        if (!drawn || !shown(item))
            return;
        for (let at = item.parent; at; at = at.parent)
            at.ensurePolished();
        const ringBox = box(drawn);
        const own = box(item);
        for (let at = item.parent; at; at = at.parent) {
            if (!at.clip && at.parent)
                continue;
            const edge = box(at);
            if (inside(own, edge) && !inside(ringBox, edge)) {
                const sides = [ringBox.x < edge.x - 0.5 ? "left" : "", ringBox.y < edge.y - 0.5 ? "top" : "", ringBox.x + ringBox.width > edge.x + edge.width + 0.5 ? "right" : "", ringBox.y + ringBox.height > edge.y + edge.height + 0.5 ? "bottom" : ""].filter(side => side !== "");
                cuts.push("ring on " + where + " of “" + named(item) + "” cut on the " + sides.join(" and ") + " by " + named(at) + (at.objectName ? " " + at.objectName : "") + " in " + named(at.parent ?? at));
                return;
            }
        }
        const over = covered(item, ringBox, own);
        if (over)
            cuts.push("ring on " + where + " of “" + named(item) + "” covered by " + named(over));
    }

    function settle(item) {
        for (const child of item.children)
            settle(child);
        item.ensurePolished();
        if (typeof item.forceLayout === "function")
            item.forceLayout();
    }

    function find(root, test) {
        if (test(root) && shown(root))
            return root;
        for (const child of root.children) {
            const found = find(child, test);
            if (found)
                return found;
        }
        return null;
    }

    function focused(root, test, where) {
        const item = find(root, test);
        if (!item)
            return cuts.push("nothing to focus on " + where);
        item.forceActiveFocus(Qt.TabFocusReason);
        ring(item, where);
    }

    function fail(found) {
        throw new Error(found.join(" | "));
    }

    function problems(root, where) {
        settle(root);
        const out = cuts.concat(scan(root, where, []));
        cuts = [];
        return out.filter((text, at) => out.indexOf(text) === at);
    }
}
