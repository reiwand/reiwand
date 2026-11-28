import QtQuick
import io.github.reiwand.app

// What the workbench shows without saving it: F11 enlarges a panel and gives it back, a panel or a
// whole group goes to a window and returns, and a small window folds the sides into rails while the
// canvas has less than the room it wants: on 1366 × 768 the left side and the foot, never 1600 × 900.
QtObject {
    required property var page

    function fail(what) {
        throw new Error("workbench view: " + what);
    }

    function exercise() {
        const layout = page.layout, view = layout.view, area = page.docks;
        layout.reset();
        const usual = layout.groups.length;
        view.maximise("layers");
        if (layout.groups.length !== 1 || layout.groups[0].shown !== "layers" || layout.groups[0].width !== layout.width || layout.edges.length !== 0)
            fail("F11 did not enlarge the layers: " + JSON.stringify(layout.groups));
        view.maximise("layers");
        if (layout.groups.length !== usual)
            fail("F11 again did not give the workbench back");
        view.detach("layers");
        if (layout.groupOf("layers") || !layout.isShown("layers") || !area.bodyOf("layers", area.serial))
            fail("a detached panel stayed on the workbench or has no window");
        if (layout.desk.includes("detached"))
            fail("a detached panel was saved");
        area.bodyOf("layers", area.serial).Window.window.back();
        if (!layout.groupOf("layers") || view.detached.length !== 0)
            fail("Return to the workbench did not bring the layers back");
        view.detach("canvas");
        if (layout.groupOf("canvas") || !area.bodyOf("canvas", area.serial) || layout.view.drawn.includes("canvas") || layout.groups.length === 0)
            fail("the canvas did not go to a window of its own: " + layout.view.drawn);
        area.bodyOf("canvas", area.serial).Window.window.back();
        if (!layout.groupOf("canvas") || view.windows.length !== 0)
            fail("the canvas did not come back to the workbench");
        const whole = layout.groupOf("assets").panels;
        if (!view.detachAll(whole) || view.windows.length !== 1 || whole.some(unit => layout.groupOf(unit)))
            fail("the whole group of Assets did not go to one window: " + JSON.stringify(view.windows));
        view.showIn(whole[1]);
        if (!layout.isShown(whole[1]) || layout.isShown(whole[0]) || !area.bodyOf(whole[1], area.serial) || area.bodyOf(whole[0], area.serial))
            fail("the window of the group did not show the tab chosen in it");
        area.bodyOf(whole[1], area.serial).Window.window.back();
        if (view.windows.length !== 0 || JSON.stringify(layout.groupOf("assets").panels) !== JSON.stringify(whole))
            fail("Return to the workbench did not bring the whole group back: " + layout.root);
        const made = (width, height) => Qt.createComponent("SceneLayout.qml").createObject(page, {
                "width": width,
                "height": height
            });
        const wide = made(1600, 844);
        if (wide.view.rails.length > 0)
            fail("a 1600 × 900 window folded " + JSON.stringify(wide.view.rails));
        wide.destroy();
        const narrow = made(1366, 712);
        const rails = narrow.view.rails, strips = narrow.groups.filter(group => group.rail);
        const left = strips.find(group => group.side === "left"), foot = strips.find(group => group.side === "bottom");
        if (!left || left.x !== 0 || left.width !== narrow.view.railWidth || !foot || foot.height !== narrow.view.railWidth || !foot.panels.includes("timeline"))
            fail("a 1366 × 768 window did not fold the left side and the foot into strips: " + JSON.stringify(strips) + " " + JSON.stringify(narrow.canvasBox));
        if (narrow.canvasBox.width < narrow.canvasFloor || strips.some(one => one.side === "right"))
            fail("the canvas lost its floor or the inspector folded: " + JSON.stringify(narrow.groups));
        for (const strip of [left, foot]) {
            narrow.reveal(strip.panels[0]);
            const over = narrow.groups.find(group => group.overlay);
            const next = strip.side === "left" ? over?.x === strip.x + strip.width : over && Math.abs(over.y + over.height - strip.y) < 1;
            if (!over || over.shown !== strip.panels[0] || narrow.groupOf(strip.panels[0]) !== over || !next)
                fail("the rail did not unfold " + strip.panels[0] + " beside it: " + JSON.stringify(over));
            narrow.view.unfold("");
        }
        const tiny = made(800, 548);
        if (tiny.canvasBox.width < tiny.canvasFloor - 1 || tiny.view.rails.length < 2)
            fail("a small window did not fold enough sides: " + JSON.stringify(tiny.view.rails));
        tiny.destroy();
        narrow.destroy();
        layout.reset();
    }
}
