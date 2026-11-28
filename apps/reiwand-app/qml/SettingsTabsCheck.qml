import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var page
    readonly property var places: ["more", "folders", "lists", "lock", "pause", "screen", "layouts", "grants", "sound", "media", "lights", "advanced", "depth", "version", "licenses", "testing"]

    function same(found, wanted, what) {
        if (found !== wanted)
            throw new Error(what + ": " + JSON.stringify(found) + " instead of " + JSON.stringify(wanted));
    }

    function home(at) {
        page.query = "";
        page.place = "";
        page.tab = at;
    }

    function texts(item) {
        return SettingsReach.rows(item).map(row => row.text);
    }

    function has(item, text, what) {
        const row = SettingsReach.rows(item).find(row => row.text === text);
        if (!row)
            throw new Error(what + ": no row “" + text + "” among " + JSON.stringify(texts(item)));
        return row;
    }

    function checkWallpapers() {
        home(0);
        for (const text of [App.tr("Mature content"), App.tr("Flashes"), App.tr("Library folders"), App.tr("Lists per screen"), App.tr("More")])
            has(page.tabs[0], text, "the wallpapers tab");
        same(has(page.tabs[0], App.tr("Flashes"), "flashes").options.length, 2, "the flashes choice");
    }

    function checkLock() {
        home(0);
        same(has(page.tabs[0], App.tr("Lock screen"), "the lock screen row").value, "Harbour · Sea", "what the lock screens show");
        page.open("lock");
        same(page.route.length, 2, "the route of the lock screen");
        const every = has(page.shown.item, App.tr("Every screen"), "the lock screen");
        same(every.options[every.current], "Harbour", "the wallpaper of every lock screen");
        const own = has(page.shown.item, App.screen("HDMI-A-1"), "the lock screen of one screen");
        same(own.options[own.current], "Sea", "the playlist of one lock screen");
        same(own.options[0], App.tr("Like every screen"), "a screen that takes the choice of every screen");
        same(own.options[1], App.tr("Like the desktop"), "a screen that can keep its desktop under every screen's choice");
        same(has(page.shown.item, App.screen("DP-1"), "a screen without its own").current, 0, "a screen without its own choice");
    }

    function checkMore() {
        home(0);
        page.open("more");
        same(page.route.length, 2, "the route of More");
        has(page.shown.item, App.tr("Keyboard shortcuts"), "More");
        same(page.shown.item.unanswered(true, "refused"), true, "keys the desktop refused");
        same(page.shown.item.unanswered(true, "unavailable"), true, "keys the desktop cannot give");
        same(page.shown.item.unanswered(true, "bound"), false, "keys the desktop gave");
        same(page.shown.item.unanswered(false, "refused"), false, "keys nobody asked for");
        same(SettingsFolder.joined(SettingsFolder.keys("LOGO+ALT+N")), "LOGO+ALT+N", "a shortcut read and written back");
    }

    function checkScreens() {
        home(1);
        has(page.tabs[1], App.tr("Screen you adjust"), "two screens");
        has(page.tabs[1], App.tr("Transition on every screen"), "the general transition");
        has(page.tabs[1], App.tr("Less motion"), "less motion");
        const words = SettingsWords.transition(true, "DP-1", {
            "transition": {
                "own": "fade",
                "why": "reduced"
            }
        }, {});
        same(words.values[words.current], "fade", "the own transition of a screen");
        same(words.why, App.tr("Last change: a plain fade, because of less motion."), "the reason of the last change");
        const fixed = SettingsWords.quality(false, "DP-1", {
            "quality_auto": "lower"
        }, {
            "playback.quality": "q2"
        });
        same(fixed.words[fixed.current], App.tr("Always high"), "a level from More shown in the row");
        same(fixed.note, App.tr("You set it: it stays on battery") + App.tr(", even though the automatic one would lower it now"), "a fixed level wins");
        page.open("pause");
        const battery = has(page.shown.item, App.tr("Pause on battery"), "pause");
        same(battery.off, !(App.state.on_battery === true || App.state.battery_percent !== null && App.state.battery_percent !== undefined), "a battery that is not detected");
    }

    function checkLayout() {
        home(1);
        page.open("layouts");
        same(has(page.shown.item, "Desk", "the layouts").value, App.tr("In use"), "the layout in use");
        const offer = Qt.createComponent("LayoutOffer.qml").createObject(check, {
            "host": page
        });
        same(offer.message("DP-1,HDMI-A-1", page.profiles), "", "screens a layout already keeps");
        const told = offer.message("DP-1", page.profiles);
        same(told, App.tr("Screens now: %1. Remember this layout?", "DP-1"), "a screen removed");
        applicationWindow().showPassiveNotification(told, 15000, App.tr("Save"), () => {});
        offer.destroy();
    }

    function checkSearch() {
        home(0);
        page.query = App.tr("Pause on battery").toUpperCase();
        if (page.found.hits.length < 1 || page.found.hits.some(hit => !SettingsReach.matches(hit.text, page.query)))
            throw new Error("searching ignores case: " + JSON.stringify(page.found.hits.map(hit => hit.text)));
        same(SettingsReach.matches("Screens", "SCRÉENS"), true, "searching ignores accents");
        page.query = App.tr("battery");
        if (!page.found.hits.some(hit => hit.tab === 1))
            throw new Error("battery is not found in the screens tab");
        // Esc clears the search from a result too, once the search field is on the page.
        page.found.enter();
        if (page.unsearch()) {
            same(page.query, "", "the search after Esc on a result");
            same(page.unsearch(), false, "Esc with no search");
        }
    }

    function empty() {
        home(0);
        page.query = "zzz";
        same(page.found.hits.length, 0, "a word no setting has");
        same(page.found.tried.visible, true, "the words to try");
        page.found.tried.picked(0);
        same(page.query, page.found.tries[0], "a word tried");
    }

    function error() {
        const kept = page.sources;
        page.sources = kept.concat([{
                "uid": "lost",
                "path": "photos/old",
                "error": "missing",
                "items": 3
            }]);
        home(0);
        same(has(page.tabs[0], App.tr("Library folders"), "folders").note, App.tr("%1 cannot be found", "old"), "a folder that is lost");
        page.open("folders");
        same(has(page.shown.item, "old", "the lost folder").status, App.tr("This folder cannot be found"), "the lost folder row");
        page.sources = kept;
    }

    function labels() {
        const named = page.entries.map(entry => entry.text);
        const lacking = [];
        const look = item => SettingsReach.all(item).filter(row => row.objectName !== "dynamic" && !named.includes(row.text)).forEach(row => lacking.push(row.text));
        for (let at = 0; at < page.tabs.length; at++) {
            home(at);
            look(page.tabs[at]);
        }
        for (const place of places) {
            home(SettingsPlaces.tab(place));
            page.open(place);
            look(page.shown.item);
        }
        same(JSON.stringify(lacking), "[]", "rows the search cannot find");
        same(named.every(text => page.entries.some(entry => SettingsReach.matches(entry.text, text))), true, "every label found by itself");
    }

    function exercise() {
        const tab = page.tab;
        const place = page.place;
        checkWallpapers();
        checkLock();
        checkMore();
        checkScreens();
        checkLayout();
        checkSearch();
        empty();
        error();
        labels();
        Qt.createComponent("SettingsMachineCheck.qml").createObject(check, {
            "page": page
        }).exercise();
        home(tab);
        page.place = place;
    }
}
