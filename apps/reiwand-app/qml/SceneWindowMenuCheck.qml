import QtQuick
import io.github.reiwand.app

// The Window menu reached with the keys, as a person reaches it: from a panel, Alt takes the
// keyboard to the menus, the arrows go to Window and open it, and Enter chooses. "Detach into a
// window", "Maximize" and "Next panel" then act on the panel that had the keyboard before.
QtObject {
    required property var page

    function fail(what) {
        throw new Error("window menu: " + what);
    }

    function press(key) {
        App.backend.press(key, 0);
    }

    // Alt from the panel of `unit`, then Right to Window, Down to open it.
    function openWindow(unit) {
        page.layout.reveal(unit);
        page.docks.focusTab(unit);
        if (page.docks.focusUnit !== unit)
            fail("the keyboard did not reach " + unit);
        press(Qt.Key_Alt);
        const menus = page.menus;
        for (let step = 0; step < menus.names.indexOf("window"); step++)
            press(Qt.Key_Right);
        press(Qt.Key_Down);
        if (!menus.popup.opened || menus.names[menus.shown] !== "window")
            fail("Alt and the arrows did not open the Window menu");
        return menus.popup;
    }

    // Down until the entry named `text` is current, then Enter.
    function choose(popup, text) {
        const at = popup.options.findIndex(option => option.text === text);
        if (at < 0)
            fail("the Window menu has no " + text);
        if (popup.options[at].off)
            fail(text + " is greyed out though a panel had the keyboard before the menu");
        for (let tries = 0; tries < popup.options.length && popup.current !== at; tries++)
            press(Qt.Key_Down);
        if (popup.current !== at)
            fail("the arrows did not reach " + text);
        press(Qt.Key_Return);
    }

    function exercise() {
        const view = page.layout.view;
        page.layout.chooseKey(1);
        choose(openWindow("layers"), App.tr("Detach into a window"));
        if (!view.detached.includes("layers"))
            fail("Detach into a window did not detach Layers: " + JSON.stringify(view.detached));
        view.attach("layers");
        const popup = openWindow("layers");
        // The key shows once, beside the name, not in it as well.
        const big = popup.options.find(option => option.text === App.tr("Maximize"));
        if (!big || big.note !== "F11" || popup.options.some(option => !option.separator && option.text?.includes("F11")))
            fail("Maximize shows F11 other than once beside it: " + JSON.stringify(big));
        choose(popup, App.tr("Maximize"));
        if (view.maximised !== "layers")
            fail("Maximize did not enlarge Layers: " + view.maximised);
        view.maximise("");
        const before = page.layout.groupOf("layers");
        choose(openWindow("layers"), App.tr("Next panel"));
        const after = page.layout.groupOf(page.docks.focusUnit);
        if (!after || after === before || page.docks.focusUnit === "")
            fail("Next panel did not go on from Layers (" + page.docks.focusUnit + ")");
    }
}
