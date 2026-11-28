import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The recent projects above, the ways to start below. When the room under the recent projects is
// shorter than one row of tiles (a small or scaled screen), both share one scroll that ends over the
// footer and follows the row in use, so starting a new one stays in reach.
Item {
    id: scroll

    required property var page
    readonly property alias recent: recent
    readonly property alias rows: rows
    readonly property real room: Math.max(0, page.low.height - page.foot.height - Look.e4)
    readonly property bool shared: !page.all && page.low.x === page.high.x && page.low.width === page.high.width && room < rows.oneLine

    function follow() {
        if (!shared)
            return;
        const row = page.focused;
        const low = Math.max(-flick.topMargin, flick.contentHeight + flick.bottomMargin - flick.height);
        // The first row of the band shows with its tabs; a later one from its own heading.
        const first = row === rows.scene || row === rows.learn;
        const top = !row || row === recent.row ? 0 : first ? rows.y : row.parent.mapToItem(flick.contentItem, 0, 0).y;
        flick.contentY = Math.min(low, top - flick.topMargin);
    }

    // Any item that takes the keyboard inside the scroll (a tab, a button) is brought into view.
    function reveal(item) {
        let inside = item;
        while (inside && inside !== flick.contentItem)
            inside = inside.parent;
        if (!shared || !inside || item === flick.contentItem)
            return;
        const top = item.mapToItem(flick.contentItem, 0, 0).y, view = flick.height - flick.topMargin - flick.bottomMargin;
        if (top < flick.contentY + flick.topMargin)
            flick.contentY = top - flick.topMargin;
        else if (top + item.height > flick.contentY + flick.topMargin + view)
            flick.contentY = top + item.height - view - flick.topMargin;
    }

    width: page.width
    height: page.height
    onSharedChanged: Qt.callLater(follow)

    Connections {
        target: scroll.page

        function onFocusedChanged() {
            scroll.follow();
        }
    }

    Connections {
        target: scroll.Window.window

        function onActiveFocusItemChanged() {
            scroll.reveal(scroll.Window.activeFocusItem);
        }
    }

    CreateRecent {
        id: recent

        page: scroll.page
        parent: scroll.shared ? flick.contentItem : scroll
        visible: !scroll.page.all
        x: scroll.shared ? 0 : scroll.page.high.x
        y: scroll.shared ? 0 : scroll.page.high.y
        width: scroll.page.high.width
    }

    Flickable {
        id: flick

        visible: !scroll.page.all
        x: scroll.page.low.x - Look.ringRoom
        y: (scroll.shared ? scroll.page.high.y : scroll.page.low.y) - Look.ringRoom
        width: scroll.page.low.width + 2 * Look.ringRoom
        height: (scroll.shared ? Math.max(0, scroll.page.foot.y - Look.e4 - scroll.page.high.y) : scroll.room) + 2 * Look.ringRoom
        leftMargin: Look.ringRoom
        rightMargin: Look.ringRoom
        topMargin: Look.ringRoom
        bottomMargin: Look.ringRoom
        contentWidth: width - 2 * Look.ringRoom
        contentHeight: Math.max(height - 2 * Look.ringRoom, rows.y + rows.height)
        clip: true
        boundsBehavior: Flickable.StopAtBounds
        QQC2.ScrollBar.vertical: QQC2.ScrollBar {}

        CreateBand {
            id: rows

            page: scroll.page
            y: scroll.shared ? scroll.page.low.y - scroll.page.high.y : Math.max(0, flick.height - 2 * Look.ringRoom - height)
            width: parent.width
        }
    }
}
