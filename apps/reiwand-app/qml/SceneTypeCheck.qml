import QtQuick
import io.github.reiwand.app

// The type of the workbench: in every factory layout each panel is brought into view, and no words
// in sight inside it are larger than its tab's (13 px), so the window's title stays the largest
// text and a panel's title is not outdone by what it holds. The canvas draws the scene's own type.
QtObject {
    required property var page

    function fail(what) {
        throw new Error("panel type: " + what);
    }

    function size(item) {
        return item.font.pixelSize > 0 ? item.font.pixelSize : item.font.pointSize * 96 / 72;
    }

    // The first item in sight whose words are too large, or null.
    function large(item) {
        if (!item.visible || item.opacity === 0)
            return null;
        if (item.font !== undefined && typeof item.text === "string" && item.text !== "" && item.width > 0 && size(item) > DeskLook.tTab + 0.01)
            return item;
        for (const child of item.children) {
            const found = large(child);
            if (found)
                return found;
        }
        return null;
    }

    function exercise() {
        if (!Look.desk)
            fail("the workbench is not drawn at its density");
        const layout = page.layout;
        const was = layout.desk;
        try {
            for (let kit = 1; kit <= 6; kit++) {
                layout.chooseKey(kit);
                for (const unit of layout.units) {
                    if (unit === "canvas" || (layout.absent ?? []).includes(unit))
                        continue;
                    layout.reveal(unit);
                    const group = page.docks.groupItem(unit);
                    if (!group || group.rail || group.current !== unit)
                        continue;
                    const found = large(group.body);
                    if (found)
                        fail("layout " + kit + ", " + unit + ": «" + found.text.slice(0, 60) + "» is " + size(found).toFixed(1) + " px (" + String(found).replace(/\(0x[0-9a-f]+.*$/, "") + ")");
                }
            }
        } finally {
            layout.store(was);
        }
    }
}
