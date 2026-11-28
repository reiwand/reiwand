import QtQuick
import io.github.reiwand.app

FocusScope {
    id: covers

    required property var page
    readonly property bool all: page.all
    readonly property Collection view: all ? grid : shelf
    readonly property alias shelf: shelf
    readonly property alias grid: grid
    readonly property alias menu: menu
    readonly property alias fixed: fixed
    readonly property alias keys: keys
    readonly property var hints: keys.hints

    function look() {
        const count = covers.page.library.total;
        if (count === 0)
            return;
        if (all) {
            const columns = grid.columns;
            const cell = Math.max(1, grid.view.cellHeight);
            const top = Math.max(0, Math.floor(grid.view.contentY / cell));
            const bottom = Math.ceil((grid.view.contentY + grid.view.height) / cell);
            covers.page.library.visible(top * columns, Math.min(count, bottom * columns) - 1);
        } else {
            const at = shelf.view.indexAt(shelf.view.contentX + Look.cover / 2, shelf.view.height - Look.e5);
            const first = Math.max(0, Math.min(count - 1, at >= 0 ? at : shelf.currentIndex) - 2);
            covers.page.library.visible(first, Math.min(count - 1, first + Math.ceil(shelf.width / Look.cover) + 4));
        }
    }

    function keep() {
        const at = view.currentIndex;
        if (at < 0)
            return;
        view.currentIndex = -1;
        view.currentIndex = at;
        if (keys.free)
            view.view.forceActiveFocus();
    }

    function newest() {
        covers.page.routeBack();
        covers.page.all = false;
        covers.page.bar.clear();
        shelf.currentIndex = 0;
        shelf.view.positionViewAtBeginning();
        shelf.view.forceActiveFocus(Qt.TabFocusReason);
    }

    function extend(from, to) {
        if (covers.page.chosen.length === 0)
            covers.page.library.toggle(from, true);
        covers.page.choosing = true;
        covers.page.library.range(to, false);
        covers.page.sync();
    }

    implicitHeight: all ? grid.height + fixed.implicitHeight + Look.e5 : shelf.implicitHeight
    onAllChanged: {
        covers.page.order.land(String(covers.page.library.uid((all ? shelf : grid).currentIndex)));
        Qt.callLater(look);
        Qt.callLater(() => covers && covers.view.forceActiveFocus());
    }
    onVisibleChanged: visible && page.order.refill()
    Keys.onDownPressed: event => {
        event.accepted = !covers.page.all;
        covers.page.all = true;
    }
    Keys.onUpPressed: covers.page.bar.words.forceActiveFocus(Qt.TabFocusReason)
    Keys.onTabPressed: event => keys.tab(event)
    Keys.onBacktabPressed: event => keys.backtab(event)
    Keys.onEscapePressed: event => {
        event.accepted = covers.page.choosing || covers.page.all;
        if (covers.page.choosing)
            covers.page.stop();
        else if (covers.page.all)
            covers.page.all = false;
    }
    Keys.onPressed: event => {
        if (event.key === Qt.Key_A && event.modifiers & Qt.ControlModifier) {
            covers.page.choosing = true;
            covers.page.library.everything(true);
            covers.page.sync();
            event.accepted = true;
        }
    }

    LibraryKeys {
        id: keys

        covers: covers
    }

    CoverShelf {
        id: shelf

        visible: !covers.all
        width: parent.width
        height: implicitHeight
        focus: !covers.all
        label: App.tr("Your library")
        model: covers.page.library
        pictureRole: "thumb"
        nameRole: "title"
        choosing: covers.page.choosing
        chosen: covers.page.chosen
        choosable: true
        actions: LibraryCard {
            page: covers.page
        }
        selection: LibraryChosen {
            page: covers.page
        }
        onActivated: covers.page.activate()
        onAsked: (index, at) => menu.ask(at)
        onToggled: index => covers.page.toggle(index)
        onExtended: (from, to) => covers.extend(from, to)
        onCurrentIndexChanged: Qt.callLater(covers.look)
    }

    Connections {
        target: shelf.view

        function onContentXChanged() {
            covers.look();
        }
    }

    CoverGrid {
        id: grid

        visible: covers.all
        width: parent.width
        height: Math.max(Look.coverGrid, covers.page.height * (Look.tier === "short" ? 0.5 : 0.62))
        focus: covers.all
        label: App.tr("All your wallpapers")
        listedOf: uid => covers.page.order.mark(uid)
        model: covers.page.library
        pictureRole: "thumb"
        nameRole: "title"
        choosing: covers.page.choosing
        chosen: covers.page.chosen
        choosable: true
        onActivated: covers.page.activate()
        onAsked: (index, at) => menu.ask(at)
        onToggled: index => covers.page.toggle(index)
        onExtended: (from, to) => covers.extend(from, to)
    }

    LibraryMenu {
        id: menu

        page: covers.page
    }

    Connections {
        target: grid.view

        function onContentYChanged() {
            covers.look();
        }

        function onHeightChanged() {
            covers.look();
        }
    }

    LibraryFixed {
        id: fixed

        covers: covers
    }
}
