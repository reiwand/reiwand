import QtQuick
import io.github.reiwand.app

// Each of the six layouts: it builds with every panel once (in a group or closed) and the canvas
// alone above its floor; each panel it holds moves to each side its menu offers and back; one panel
// goes to a window of its own and returns; and what is changed is saved and read back the same.
QtObject {
    id: check

    required property var page
    required property var checks

    function fail(what) {
        throw new Error("layouts: " + what);
    }

    function walk(layout, kit) {
        const usual = layout.root;
        const open = layout.units.filter(unit => !layout.closed.includes(unit));
        if (open.length === 0 || open.length > 8)
            fail(kit + " holds " + open.length + " panels: " + open);
        for (const unit of open)
            for (const side of ["left", "right", "up", "down"]) {
                const could = layout.canMove(unit, side);
                if (layout.move(unit, side) !== could)
                    fail(kit + ": " + unit + " moved " + side + " against what its menu offered");
                checks.whole(layout, kit + ": " + unit + " moved " + side);
                layout.change(usual);
            }
        const away = open[0];
        if (!layout.view.detach(away) || !layout.view.detached.includes(away))
            fail(kit + ": " + away + " did not go to a window of its own");
        layout.view.attach(away);
        if (layout.view.detached.length > 0 || !layout.groupOf(away))
            fail(kit + ": " + away + " did not come back from its window");
        const moved = open.find(unit => layout.canMove(unit, "right") && layout.move(unit, "right"));
        layout.flush();
        const read = Qt.createComponent("SceneLayout.qml").createObject(page, {
            "width": layout.width,
            "height": layout.height
        });
        if (read.held || read.root !== layout.root || JSON.stringify(read.parsed.chosen) !== JSON.stringify(layout.parsed.chosen))
            fail(kit + ": " + (moved ?? "nothing") + " moved was not read back: " + read.root);
        read.destroy();
    }

    function exercise() {
        const layout = page.layout;
        for (let at = 0; at < layout.kits.length; at++) {
            layout.chooseKey(at + 1);
            layout.reset();
            const kit = layout.kits[at];
            checks.whole(layout, "the layout " + kit);
            walk(layout, kit);
            layout.reset();
        }
        layout.chooseKey(1);
    }
}
