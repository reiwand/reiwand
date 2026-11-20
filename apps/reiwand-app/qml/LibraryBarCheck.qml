import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var page
    readonly property var library: page.library
    readonly property var covers: page.covers

    function must(ok, text) {
        if (!ok)
            throw new Error(text);
    }

    function exercise() {
        wheel();
        typing();
        filters();
        order();
        refill();
        emptied();
    }

    function emptied() {
        const bar = page.bar;
        const found = Qt.createQmlObject("import QtQuick; QtObject { property int total: 3; property bool loading: false; property string error: \"\" }", page);
        const state = Qt.createComponent("LibraryState.qml").createObject(page, {
            "page": {
                "library": found,
                "band": Qt.rect(0, 0, 0, 0),
                "upper": Qt.rect(0, 0, 0, 0)
            },
            "bar": bar
        });
        bar.find.forceActiveFocus(Qt.TabFocusReason);
        bar.find.text = "zzq";
        found.total = 0;
        App.backend.press(Qt.Key_Backspace, 0);
        const kept = [bar.find.text, bar.find.activeFocus];
        state.destroy();
        found.destroy();
        bar.clear();
        covers.view.view.forceActiveFocus(Qt.TabFocusReason);
        must(kept.join() === "zz,true", "a search with no match takes the focus away from the search box: " + kept.join());
    }

    function filters() {
        const bar = page.bar;
        const kept = bar.word;
        bar.word = 5;
        const found = [bar.term, bar.shown.includes(5), App.libraryFilter.word, bar.find.placeholder === App.tr("Search in %1", App.tr("From sources"))];
        bar.word = kept;
        must(found.join() === "provider:yes,true,5,true", "the source filter is not a word of the bar, or the bar forgets it: " + found.join());
    }

    function order() {
        const order = page.order;
        const found = [order.unique(["a", "", "a", "b"]).join("+"), order.mark("x")];
        must(found.join() === "a+b,", "what is in use does not lead the grid once: " + found.join());
    }

    function wheel() {
        const shelf = covers.shelf;
        shelf.currentIndex = 0;
        shelf.turn(0, -120);
        shelf.turn(0, -60);
        const one = shelf.currentIndex;
        shelf.turn(0, -60);
        page.bar.turned(0, 120);
        shelf.turn(0, 120 * library.total);
        must([one, shelf.currentIndex, shelf.turning].join() === "1,0,true", "the wheel does not move the shelf one wallpaper per notch: " + [one, shelf.currentIndex].join());
        const cover = shelf.view.currentItem;
        const at = cover.mapToItem(null, cover.width / 2, cover.height / 2);
        App.backend.wheel(at.x, at.y, -120);
        must(shelf.currentIndex === 1, "a real turn of the wheel over the shelf does not reach it: " + shelf.currentIndex + " under " + under(cover.Window.contentItem, at.x, at.y));
        shelf.currentIndex = 0;
    }

    function under(item, x, y) {
        const chain = [];
        for (let here = item, p = Qt.point(x, y); here; ) {
            chain.push(String(here).split("(")[0]);
            const next = here.childAt(p.x, p.y);
            if (next)
                p = here.mapToItem(next, p.x, p.y);
            here = next;
        }
        return chain.join(" > ");
    }

    function refill() {
        const shelf = covers.shelf;
        shelf.currentIndex = 2;
        shelf.view.forceActiveFocus(Qt.TabFocusReason);
        covers.visible = false;
        shelf.currentIndex = -1;
        covers.visible = Qt.binding(() => library.total > 0);
        const found = [shelf.currentIndex, shelf.view.activeFocus];
        App.backend.press(Qt.Key_Right, 0);
        found.push(shelf.currentIndex);
        shelf.currentIndex = 0;
        must(found.join() === "0,true,1", "the shelf has no lit wallpaper or does not move after its result goes empty and comes back: " + found.join());
    }

    function typing() {
        const bar = page.bar;
        bar.find.forceActiveFocus(Qt.TabFocusReason);
        bar.find.text = "a";
        library.reloaded();
        covers.keep();
        const kept = bar.find.activeFocus;
        bar.clear();
        covers.view.view.forceActiveFocus(Qt.TabFocusReason);
        must(kept, "a reply of the search takes the focus away from the search box");
    }
}
