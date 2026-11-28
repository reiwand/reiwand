import QtQuick
import io.github.reiwand.app

// The keys of the workbench, each taken from the shortcut that holds it in the page: F6 and
// Shift+F6 move the keyboard between groups, Ctrl+Tab and Ctrl+Shift+Tab between the tabs of a group,
// F11 enlarges the focused panel and gives it back, Ctrl+1 … Ctrl+6 choose the factory layouts and
// the arrows move a focused edge, ten times as far with Shift; the menu key opens a tab's menu.
// After F6 in every layout, the keys of the panel work at once.
QtObject {
    required property var page

    function fail(what) {
        throw new Error("workbench keys: " + what);
    }

    property var keys: null

    // Every shortcut that is on in what the window shows, by its sequence (the top bar is drawn
    // outside the page); an instantiator gives the ones it made.
    function gather(item, out) {
        const list = item.data ?? [];
        for (let at = 0; at < list.length; at++) {
            const one = list[at];
            if (!one)
                continue;
            if (typeof one.sequence === "string" && typeof one.activated === "function") {
                if (one.enabled)
                    out[one.sequence] = (out[one.sequence] ?? []).concat([one]);
            } else if (String(one).startsWith("QQmlInstantiator")) {
                for (let made = 0; made < one.count; made++)
                    gather({
                        "data": [one.objectAt(made)]
                    }, out);
            } else if (one.data !== undefined && one.visible === true) {
                gather(one, out);
            }
        }
        return out;
    }

    function chord(sequence) {
        const found = keys[sequence] ?? [];
        if (found.length !== 1)
            fail(sequence + " is held by " + found.length + " shortcuts that are on");
        found[0].activated();
    }

    function groups(area) {
        chord("F6");
        const first = area.focusUnit;
        if (first === "")
            fail("F6 did not take the keyboard to a group");
        // F6 lands in the panel, where its keys work, not on its tab.
        const landed = String(page.Window.activeFocusItem);
        if (landed.startsWith("SceneGroupTab") || landed.startsWith("QQuickToolButton"))
            fail("F6 left the keyboard on the tab of " + first + ": " + landed);
        chord("F6");
        const second = area.focusUnit;
        if (second === "" || page.layout.groupOf(second) === page.layout.groupOf(first))
            fail("F6 did not go on to the next group from " + first + " (" + second + ")");
        chord("Shift+F6");
        if (area.focusUnit !== first)
            fail("Shift+F6 did not come back to " + first + " (" + area.focusUnit + ")");
    }

    function tabs(area) {
        const layout = page.layout;
        const home = layout.groups.find(group => group.panels.length > 1 && !group.panels.includes("code"));
        if (!home)
            fail("no group of the Scene layout holds two panels: " + layout.root);
        area.focusTab(home.shown);
        const was = home.shown;
        chord("Ctrl+Tab");
        const now = layout.groupOf(was).shown;
        if (now === was || area.focusUnit !== now)
            fail("Ctrl+Tab did not open the next tab after " + was + " (" + now + ", focus on " + area.focusUnit + ")");
        chord("Ctrl+Shift+Tab");
        if (layout.groupOf(was).shown !== was)
            fail("Ctrl+Shift+Tab did not come back to " + was);
    }

    function enlarge(area) {
        const view = page.layout.view;
        const unit = area.focusUnit;
        chord("F11");
        if (!view.maximised)
            fail("F11 did not enlarge " + unit);
        chord("F11");
        if (view.maximised)
            fail("F11 did not give the workbench back");
    }

    function layouts() {
        const layout = page.layout;
        for (let at = layout.kits.length - 1; at >= 0; at--) {
            chord("Ctrl+" + (at + 1));
            if (layout.parsed.chosen.factory !== layout.kits[at])
                fail("Ctrl+" + (at + 1) + " chose " + JSON.stringify(layout.parsed.chosen));
        }
    }

    function inList(item) {
        return !!item && (item.focusList === true || inList(item.parent));
    }

    function listed(node) {
        return Array.from(node?.children ?? []).some(child => child.visible && (child.focusList === true || listed(child)));
    }

    // In each of the six layouts, F6 lands in every group where the keys of the status bar work:
    // the canvas, Layers and Assets take the arrows, and Space plays the timeline.
    function panels(area) {
        const layout = page.layout;
        for (let at = 0; at < layout.kits.length; at++) {
            chord("Ctrl+" + (at + 1));
            const seen = [];
            for (let turn = 0; turn <= layout.groups.length; turn++) {
                chord("F6");
                const unit = area.focusUnit, item = page.Window.activeFocusItem;
                if (seen.includes(unit))
                    break;
                seen.push(unit);
                const where = " after F6 in the " + layout.kits[at] + " layout";
                if (String(item).startsWith("SceneGroupTab"))
                    fail("the keyboard stayed on the tab of " + unit + where);
                if (["layers", "assets"].includes(unit) && listed(area.groupItem(unit)?.body) && !inList(item))
                    fail("the list of " + unit + " does not take the arrows" + where);
                if (unit === "canvas" && item?.keyTarget !== true)
                    fail("the canvas does not take the arrows" + where);
                if (unit === "timeline") {
                    const playing = page.editor.playing;
                    App.backend.press(Qt.Key_Space, 0);
                    const played = page.editor.playing !== playing;
                    App.backend.press(Qt.Key_Space, 0);
                    if (!played)
                        fail("Space did not play the timeline" + where);
                }
            }
            if (seen.length < 2)
                fail("F6 reached " + seen.length + " group in the " + layout.kits[at] + " layout");
        }
    }

    // The menu key and Shift+F10 on a tab open the tab's menu, not the menu of a panel's own shortcut.
    function tabMenu(area) {
        for (const unit of ["canvas", "layers"]) {
            for (const [key, mods] of [[Qt.Key_Menu, 0], [Qt.Key_F10, Qt.ShiftModifier]]) {
                area.focusTab(unit);
                const tab = page.Window.activeFocusItem;
                if (!String(tab).startsWith("SceneGroupTab"))
                    fail("the tab of " + unit + " does not take the keyboard: " + tab);
                App.backend.press(key, mods);
                const opened = tab.menu.opened;
                tab.menu.close();
                if (!opened)
                    fail((mods ? "Shift+F10" : "the menu key") + " on the tab of " + unit + " did not open its menu");
            }
        }
    }

    function edge(area) {
        const grip = Array.from(area.children).find(item => typeof item.nudge === "function" && item.edge && !item.across);
        if (!grip)
            fail("the Scene layout has no edge between columns");
        grip.forceActiveFocus(Qt.TabFocusReason);
        if (!grip.activeFocus)
            fail("an edge does not take the keyboard");
        const small = grip.step({
            "modifiers": 0
        }), large = grip.step({
            "modifiers": Qt.ShiftModifier
        });
        if (large !== small * 10)
            fail("an arrow moves an edge " + small + " px and Shift with it " + large + " px");
        const root = page.layout.root;
        App.backend.press(Qt.Key_Left, Qt.ShiftModifier);
        App.backend.press(Qt.Key_Right, 0);
        if (page.layout.root === root)
            fail("the arrows did not move the edge: " + root);
        App.backend.press(Qt.Key_Return, 0);
    }

    function exercise() {
        const area = page.docks, layout = page.layout;
        const was = layout.desk;
        try {
            layout.chooseKey(1);
            keys = gather(page.Window.window?.contentItem ?? page, {});
            groups(area);
            tabs(area);
            tabMenu(area);
            enlarge(area);
            edge(area);
            layouts();
            panels(area);
        } finally {
            layout.store(was);
        }
    }
}
