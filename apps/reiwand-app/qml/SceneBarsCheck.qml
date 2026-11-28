import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var page

    function fail(what) {
        throw new Error("bars: " + what);
    }

    function menus() {
        const menus = page.menus;
        if (!menus)
            fail("the top bar has no menus");
        if (menus.folded !== ((menus.Window.window ?? page.Window.window).width < DeskLook.folds))
            fail("the menus did not fold below " + DeskLook.folds + " px or folded above it");
        const words = menus.words;
        const letters = words.names.map(name => menus.letter(name).toLowerCase());
        if (letters.some(letter => letter === "") || new Set(letters).size !== letters.length)
            fail("the menus do not each have their own Alt letter: " + letters.join());
        for (const name of words.names) {
            const entries = words.entries(page, name);
            if (entries.filter(entry => !entry.separator && !entry.heading).length === 0)
                fail("the " + name + " menu is empty");
            if (entries.some(entry => !entry.separator && !entry.heading && (!entry.text || typeof entry.act !== "function")))
                fail("an entry of the " + name + " menu has no name or does nothing");
        }
        menus.open(0);
        if (!menus.popup.visible || menus.shown !== 0)
            fail("the File menu did not open");
        menus.popup.close();
    }

    function window() {
        const layout = page.layout, words = page.menus.words;
        const named = unit => words.window(page).find(entry => entry.text === page.docks.title(unit));
        named("problems").act();
        if (layout.groupOf("problems"))
            fail("Window did not close the Problems panel");
        named("problems").act();
        if (!layout.groupOf("problems"))
            fail("Window did not open the Problems panel again");
        layout.reset();
    }

    function plan() {
        const layout = page.layout;
        const boxes = JSON.parse(layout.bench.blueprint(layout.root, DeskLook.planWidth, DeskLook.planHeight));
        const cells = boxes.reduce((sum, box) => sum + box.width * box.height, 0);
        if (boxes.length !== layout.groups.length || boxes.filter(box => box.canvas).length !== 1 || cells !== DeskLook.planWidth * DeskLook.planHeight)
            fail("the blueprint does not draw the groups of the layout: " + JSON.stringify(boxes));
        const planned = page.menus.words.window(page).filter(entry => entry.plan);
        if (planned.length !== layout.layouts.length || planned.filter(entry => entry.current).length !== 1 || planned.some(entry => !entry.note.startsWith("Ctrl+")))
            fail("Window does not list each layout with its blueprint and key: " + JSON.stringify(planned.map(entry => entry.text)));
    }

    function status() {
        const bar = page.status, area = page.docks;
        area.focusTab("layers");
        if (area.focusUnit !== "layers" || !bar.keys.some(key => key[1] === App.tr("choose a layer")))
            fail("the status bar does not show the keys of the focused Layers panel: " + area.focusUnit);
        if (bar.problems !== (page.view.problems ?? []).length)
            fail("the status bar counts other problems than the scene has");
        if (Math.round(page.docks.height + bar.height) !== Math.round(page.height) || bar.height !== DeskLook.status)
            fail("the status bar is not under the workbench with its height");
    }

    function exercise() {
        menus();
        window();
        plan();
        status();
    }
}
