import QtQuick
import io.github.reiwand.app

QtObject {
    id: walk

    required property var host
    readonly property RowsCheck rows: RowsCheck {}
    property string where: "a page"

    function press(key, modifiers) {
        App.backend.press(key, modifiers ?? 0);
    }

    function shown(item) {
        for (let at = item; at; at = at.parent) {
            if (!at.visible || at.opacity === 0)
                return false;
        }
        return item.width > 0 && item.height > 0;
    }

    function gather(item, out) {
        out.push(item);
        if (item.focusList !== true)
            for (const child of item.children)
                gather(child, out);
        return out;
    }

    function interactive(item) {
        if (item.keyTarget === true || item.focusList === true)
            return item.enabled;
        if (item.hovered === undefined || !item.enabled || item.showClickFeedback !== undefined)
            return false;
        if (item.position !== undefined && item.size !== undefined || item.readOnly === true || item.editable === true && item.stepSize !== undefined)
            return false;
        return item.down !== undefined && item.checkable !== undefined || item.popup !== undefined || item.stepSize !== undefined || item.selectedText !== undefined;
    }

    function tabs(keep) {
        const seen = [];
        if (!keep)
            host.contentItem.forceActiveFocus();
        for (let n = 0; n < 500; n++) {
            press(Qt.Key_Tab, 0);
            const now = host.activeFocusItem;
            if (!now || seen.includes(now))
                break;
            seen.push(now);
            rows.ring(now, where);
        }
        return seen;
    }

    function reached(seen, item) {
        return seen.some(now => {
            for (let at = now; at; at = at.parent) {
                if (at === item)
                    return true;
            }
            return false;
        });
    }

    function describe(item) {
        return String(item).replace(/\(0x[0-9a-f]+/, "(") + (item.text ? " “" + item.text + "”" : "");
    }

    function check(name) {
        where = name;
        const seen = tabs();
        if (seen.length === 0)
            throw new Error("the " + name + " page cannot be reached with the Tab key");
        const lost = gather(host.contentItem, []).filter(item => interactive(item) && shown(item) && !(item.focusList === true ? reached(seen, item) : seen.includes(item)) && !String(item).startsWith("Tile_"));
        const uneven = rows.problems(host.contentItem.parent ?? host.contentItem, name);
        if (uneven.length > 0)
            Qt.callLater(rows.fail, uneven);
        if (lost.length > 0)
            throw new Error("the " + name + " page has controls the Tab key does not reach: " + lost.map(describe).join(", "));
        return seen;
    }
}
