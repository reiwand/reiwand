import QtQuick
import io.github.reiwand.app

// The keys of the workbench: F6 and Shift+F6 take the keyboard to the next or previous group (in
// the order of the tree, left to right and top to bottom, folded rails included), Ctrl+Tab and Ctrl+Shift+Tab to the next
// or previous tab of the group that has it, F11 enlarges its panel, and Esc folds a panel
// unfolded from the rail.
Item {
    id: keys

    required property var area
    readonly property var layout: area.layout
    readonly property var view: layout.view

    // The groups in the order of the tree; a side folded into a rail is one stop, in the place of
    // its groups, which unfolds its first panel, and leaving it folds it again.
    function groupAt(step) {
        const list = [];
        let over = null;
        for (let at = 0; at < area.groups.count; at++) {
            const item = area.groups.itemAt(at);
            if (item?.held?.overlay)
                over = item;
            else if (item)
                list.push(item);
        }
        if (list.length === 0)
            return;
        const placed = layout.placed.groups;
        const rank = item => placed.findIndex(group => group.panels.some(unit => item.units.includes(unit)));
        list.sort((a, b) => rank(a) - rank(b));
        let here = list.findIndex(item => item.focused);
        // From the menus the keyboard goes on from the panel that had it before.
        if (here < 0 && area.lastUnit)
            here = list.findIndex(item => item.units.includes(area.lastUnit));
        if (here < 0 && over?.focused)
            here = list.findIndex(item => item.rail && item.units.includes(over.current));
        const next = list[(here < 0 ? (step > 0 ? 0 : list.length - 1) : (here + step + list.length) % list.length)];
        // The keyboard leaves an unfolded panel before it folds back, or it would follow its items.
        if (!next.rail) {
            enter(next, next.current);
            return view.unfold("");
        }
        view.unfold("");
        const unit = next.units[0];
        view.unfold(unit);
        Qt.callLater(() => area.groupItem(unit) && enter(area.groupItem(unit), unit));
    }

    // The keyboard goes into the panel shown, where the keys of the status bar work: its list, else
    // the area that takes the keys, else its first control; the tab only when the panel has none.
    function enter(group, unit) {
        let first = null;
        const walk = (node, wanted) => {
            for (const child of node?.children ?? []) {
                if (!child.visible || child.enabled === false)
                    continue;
                if (wanted(child))
                    return child;
                first = first ?? (child.activeFocusOnTab ? child : null);
                const found = walk(child, wanted);
                if (found)
                    return found;
            }
            return null;
        };
        const target = walk(group.body, item => item.focusList === true) ?? walk(group.body, item => item.keyTarget === true) ?? first;
        target ? target.forceActiveFocus(Qt.TabFocusReason) : group.focusTab(unit);
    }

    function tabAt(step) {
        const unit = area.focusUnit;
        if (unit === "code" && area.code.files.length > 1)
            return area.code.cycle(step);
        const home = layout.groupOf(unit);
        if (!home || home.panels.length < 2)
            return;
        const next = home.panels[(home.panels.indexOf(unit) + step + home.panels.length) % home.panels.length];
        layout.reveal(next);
        Qt.callLater(area.focusTab, next);
    }

    Shortcut {
        sequence: "F6"
        enabled: keys.area.visible
        onActivated: keys.groupAt(1)
    }

    Shortcut {
        sequence: "Shift+F6"
        enabled: keys.area.visible
        onActivated: keys.groupAt(-1)
    }

    Shortcut {
        sequence: "Ctrl+Tab"
        enabled: keys.area.visible
        onActivated: keys.tabAt(1)
    }

    Shortcut {
        sequence: "Ctrl+Shift+Tab"
        enabled: keys.area.visible
        onActivated: keys.tabAt(-1)
    }

    Shortcut {
        sequence: "F11"
        enabled: keys.area.visible
        // The panel that had the keyboard last when none has it now (a tab just changed, the menus).
        onActivated: keys.view.maximise(keys.view.maximised || keys.area.focusUnit || (keys.layout.groupOf(keys.area.lastUnit) ? keys.area.lastUnit : ""))
    }

    Shortcut {
        sequence: "Esc"
        enabled: keys.area.visible && keys.view.unfolded !== ""
        onActivated: keys.view.unfold("")
    }
}
