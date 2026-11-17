import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var page
    readonly property var library: page.library
    readonly property var covers: page.covers
    property SceneDailyCheck daily: SceneDailyCheck {
        page: check.page
    }
    property Timer later: Timer {
        interval: 150
        onTriggered: check.close()
    }

    property FirstRunCheck first: FirstRunCheck {
        page: check.page
    }
    property LibraryMenuCheck menus: LibraryMenuCheck {
        page: check.page
    }
    property LibraryBarCheck bars: LibraryBarCheck {
        page: check.page
    }

    function must(ok, text) {
        if (!ok)
            throw new Error(text);
    }

    function keys() {
        must(library.total > 0, "the library shows no wallpaper to move to with the keyboard");
        covers.shelf.view.forceActiveFocus(Qt.TabFocusReason);
        App.backend.press(Qt.Key_A, Qt.ControlModifier);
        must(library.selected === library.total && page.choosing, "Ctrl+A does not choose every wallpaper of the library");
        App.backend.press(Qt.Key_Escape, 0);
        must(library.selected === 0 && !page.choosing, "Escape does not clear the choice of the library");
        must(library.total > 1, "the check library has fewer than two wallpapers");
        covers.shelf.currentIndex = 0;
        App.backend.press(Qt.Key_Right, Qt.ShiftModifier);
        must(library.selected === 2 && page.choosing, "Shift with an arrow key does not extend the choice: " + library.selected);
        App.backend.press(Qt.Key_Escape, 0);
        App.backend.press(Qt.Key_Down, 0);
        must(page.all && page.full, "the Down key does not show every wallpaper");
        App.backend.press(Qt.Key_Escape, 0);
        must(!page.all, "Escape does not bring the shelf back");
        must(page.keyHints.length === 5 && page.keyHints[1].keys[0] === "down", "the shelf does not show its key hints");
        tabs();
        bars.exercise();
    }

    function tabs() {
        const at = () => covers.Window.activeFocusItem;
        covers.shelf.currentIndex = 0;
        covers.Window.contentItem.forceActiveFocus();
        covers.keep();
        must(covers.keys.viewHas(at()), "the covers do not take the keys when the library shows");
        App.backend.press(Qt.Key_Tab, 0);
        must(covers.keys.cardHas(at()) && covers.keys.hintsOf(at())[0].keys[0] === "Tab", "Tab does not reach the buttons of the lit cover, or the hints still show the arrows");
        App.backend.press(Qt.Key_Backtab, Qt.ShiftModifier);
        must(covers.keys.viewHas(at()) && covers.keys.hintsOf(at())[0].keys[0] === "left", "Shift+Tab does not go back to the covers");
        App.backend.press(Qt.Key_Space, 0);
        must(page.choosing && library.selected === 1, "Space does not start choosing");
        App.backend.press(Qt.Key_Tab, 0);
        App.backend.press(Qt.Key_Return, 0);
        const shell = applicationWindow().shell;
        must(shell.inPopup(at()) && covers.keys.hintsOf(at()) === undefined, "Intro on the chosen band does not open its menu, or the library still claims the bar: " + at());
        App.backend.press(Qt.Key_Escape, 0);
        must(!shell.inPopup(at()) && page.choosing, "Esc does not close the menu of the chosen band alone");
        App.backend.press(Qt.Key_Escape, 0);
    }

    function exercise() {
        screens();
        several();
        options();
        empty();
        first.exercise();
        damaged();
        menus.exercise();
        daily.exercise();
        page.tools.shown();
    }

    function screens() {
        covers.shelf.currentIndex = 0;
        const uid = library.uid(0);
        library.showing(JSON.stringify({
            "DP-1": uid,
            "HDMI-A-1": "other"
        }));
        const entry = page.entry;
        const found = [entry.inUse, entry.usedOn];
        page.shown();
        must(found.join() === "true,DP-1", "a wallpaper in use does not name its screen when there are two: " + found.join());
    }

    function several() {
        page.toggle(0);
        page.toggle(1);
        must(page.chosen.length === 2 && page.choosing, "choosing two wallpapers does not show them chosen");
        page.stop();
        must(page.chosen.length === 0 && !page.choosing, "Stop choosing does not clear the choice");
    }

    function options() {
        page.open(library.uid(0));
        must(page.skeleton === "options" && page.route.length === 2 && page.actions.length === 1 && page.head !== null, "the options of a wallpaper do not open in their own panel");
        page.level = "presets";
        must(page.route.length === 3 && page.route[2] === App.tr("My presets"), "the level My presets of the options is not in the route");
        page.level = "more";
        must(page.route.length === 3, "the More level of the options is not in the route");
        later.start();
    }

    function close() {
        page.routeBack();
        must(page.level === "" && page.opened !== "", "Escape does not go back from the More level");
        page.routeBack();
        must(page.opened === "" && page.skeleton === "", "Escape does not close the options of a wallpaper");
    }

    function empty() {
        const make = (error, filtered) => Qt.createComponent("LibraryState.qml").createObject(page, {
                "page": {
                    "library": {
                        "total": 0,
                        "loading": false,
                        "error": error
                    },
                    "band": Qt.rect(300, 200, 900, 600),
                    "upper": Qt.rect(300, 0, 900, 200)
                },
                "bar": {
                    "filtered": filtered,
                    "text": "zz"
                }
            });
        const none = make("", true);
        const broken = make("no answer", false);
        broken.width = 900;
        broken.height = 600;
        const fault = broken.fault;
        must(fault.visible && fault.x === 0 && fault.y >= 0 && fault.y + fault.height <= broken.height + 1, "the service error block is drawn outside its band: " + [fault.x, fault.y, fault.height]);
        const found = [none.visible, none.unmatched, none.failed, broken.failed];
        none.destroy();
        broken.destroy();
        must(found.join() === "true,true,false,true", "a search with no results or a failed library does not show its state: " + found.join());
    }

    function damaged() {
        const card = Qt.createComponent("LibraryCard.qml").createObject(page, {
            "page": {
                "target": "",
                "entry": {
                    "uid": "01J9Z0X9Z6Q7X4F3K5E1V0R1MS",
                    "title": "Lost",
                    "kind": "scene",
                    "missing": false,
                    "broken": "the picture header is cut short"
                }
            }
        });
        const found = [card.hurt, card.main, card.said(card.entry)];
        card.destroy();
        must(found.join("|") === ["true", App.tr("Show in folder"), App.tr("Could not open this scene: the file is damaged")].join("|"), "a damaged file does not offer to show its folder or names another kind: " + found.join("|"));
    }
}
