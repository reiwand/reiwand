import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var page

    function must(ok, text) {
        if (!ok)
            throw new Error(text);
    }

    function keys() {
        must(page.shown.length > 1, "the playlists page shows fewer than two playlists to move between");
        page.shelf.covers.view.forceActiveFocus(Qt.TabFocusReason);
        App.backend.press(Qt.Key_Right, 0);
        must(page.shelf.covers.currentIndex === 1, "the Right key does not move to the next playlist");
        App.backend.press(Qt.Key_Left, 0);
        must(page.shelf.covers.currentIndex === 0, "the Left key does not move back to the first playlist");
    }

    function rotating() {
        const line = Qt.createComponent("NowPlaylist.qml").createObject(page, {
            "screen": "DP-1",
            "playlist": {
                "uid": "x",
                "name": "Sea",
                "next_at": Date.now() / 1000 + 400
            }
        });
        const text = line.when();
        line.destroy();
        must(text === App.tr("List “%1” · changes in %2", "Sea", App.tr("%1 min", 7)), "the rotating line does not say when the list changes: " + text);
    }

    function ordering() {
        const list = page.editor.list;
        page.editor.items = [{
                "uid": "a",
                "title": "A"
            }, {
                "uid": "b",
                "title": "B"
            }];
        must(list.movable && list.otherText !== "" && list.addText !== "", "a manual playlist cannot be ordered or filled at level one");
        let moved = [];
        const seen = (from, to) => moved = [from, to];
        list.reordered.connect(seen);
        list.move(0, 1);
        list.move(0, -1);
        list.reordered.disconnect(seen);
        must(moved.length === 2 && moved[0] === 0 && moved[1] === 1, "moving a wallpaper of a playlist does not ask for its new place");
    }

    function grab(list, index, rows) {
        const handle = list.rows.itemAt(index).handle;
        const at = handle.mapToItem(null, handle.width / 2, handle.height / 2);
        App.backend.pointer(0, at.x, at.y);
        for (let step = 1; step <= 8; ++step)
            App.backend.pointer(1, at.x, at.y + step * rows * Look.sublistHeight / 8);
        return at;
    }

    function dragging() {
        const editor = page.editor;
        const list = editor.list;
        editor.wallpapers.open = true;
        editor.items = Array.from({
            "length": 40
        }, (_, i) => ({
                    "uid": "w" + i,
                    "title": "W" + i
                }));
        list.forceLayout();
        editor.rows.forceLayout();
        const view = list.drag.scroller();
        must(view !== null && view.contentHeight > view.height, "the wallpapers of a playlist do not scroll in their view");
        view.contentY = 0;
        const moved = [];
        const seen = (from, to) => moved.push([from, to]);
        list.reordered.connect(seen);
        list.rows.itemAt(5).forceActiveFocus(Qt.TabFocusReason);
        App.backend.press(Qt.Key_Down, Qt.ControlModifier);
        must(JSON.stringify(moved.splice(0)) === "[[5,6]]", "Ctrl+Down does not move the wallpaper with the focus one place lower");
        const at = grab(list, 1, 2);
        must(list.drag.held === 1 && Math.abs(list.drag.lift - 2 * Look.sublistHeight) < 1, "dragging the handle of a wallpaper does not lift it: " + list.drag.lift);
        App.backend.pointer(2, at.x, at.y + 2 * Look.sublistHeight);
        const dropped = JSON.stringify(moved);
        const bottom = view.mapToItem(null, view.width / 2, view.height - 2).y;
        const last = Math.floor(list.mapFromItem(null, at.x, bottom).y / Look.sublistHeight);
        const far = grab(list, 2, 1);
        App.backend.pointer(1, far.x, bottom);
        for (let tick = 0; tick < 40; ++tick)
            list.drag.scroll();
        const scrolled = view.contentY;
        App.backend.pointer(2, far.x, bottom);
        list.reordered.disconnect(seen);
        view.contentY = 0;
        editor.wallpapers.open = false;
        must(dropped === "[[1,3]]", "dropping a dragged wallpaper two rows lower does not move it there: " + dropped);
        must(scrolled > 0 && moved.length === 2 && moved[1][0] === 2 && moved[1][1] > last, "holding a dragged wallpaper at the edge does not scroll the list to carry it further: " + [scrolled, last, JSON.stringify(moved)].join(" "));
    }

    function exercise() {
        must(page.loaded && page.playlists.length === 2, "the playlists page did not load the playlists");
        must(page.shelf.filter.words[1].count === page.running.length && page.shelf.filter.words[0].count === 2, "the filter of the playlists does not count every word");
        must(page.bandLow > 0, "the playlists shelf does not use the low band");
        must(page.shelf.entries[0].caption.indexOf(page.shelf.every("30m")) >= 0, "the card of a playlist does not say how often it changes");
        must(page.shelf.every("1h30m") === App.tr("every %1", App.tr("%1 h", 1) + " " + App.tr("%1 min", 30)), "an interval of hours and minutes is shown as written");
        must(page.shelf.every("2d") === App.tr("every %1 days", 2) && page.shelf.every("90m") === App.tr("every %1", App.tr("%1 h", 1) + " " + App.tr("%1 min", 30)), "whole days or minutes past an hour are not said in words");
        page.kind = 1;
        must(page.shown.length === page.running.length, "the filter Running still shows stopped playlists");
        page.kind = 0;
        rotating();
        const first = page.playlists[0];
        page.opened = first.uid;
        must(page.skeleton === "options" && page.route.length === 2, "editing a playlist does not open its options");
        must(page.shelf.covers.visible === false && page.bandLow === 0, "the shelf stays under the playlist editor");
        ordering();
        dragging();
        page.level = "more";
        must(page.route.length === 3, "the More level of a playlist has no route");
        const rules = (App.schema.rules ?? []).length - 1;
        must(page.editor.more.rows.children.filter(row => row.kind !== undefined).length >= 4, "the More level does not show search, folder and removal as rows");
        must(rules > 0 && page.editor.more.rules.count === rules, "the More level does not show every rule of the playlist as a row");
        page.routeBack();
        must(page.level === "" && page.opened === first.uid, "going back from More closes the playlist");
        page.routeBack();
        must(page.opened === "", "going back does not return to the shelf");
        const all = page.playlists;
        page.playlists = [];
        must(page.shelf.empty && page.bandLow > 0, "the playlists page shows no empty state in the band without playlists");
        page.playlists = all;
    }
}
