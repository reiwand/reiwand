pragma Singleton
import QtQuick

QtObject {
    // The group of the workbench that has the keyboard, or had it last. Only there is the item
    // chosen lit; the other groups keep it marked (the focus rule).
    property Item group: null

    signal canvasWanted

    // Whether a chosen item is lit: true unless it sits in a group other than the one in focus.
    function lit(item) {
        if (!group)
            return true;
        for (let at = item; at; at = at.parent) {
            if (at.focusTab !== undefined && at.rail !== undefined)
                return at === group;
        }
        return true;
    }
}
