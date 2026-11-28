import QtQuick
import io.github.reiwand.app

QtObject {
    required property var covers
    readonly property var page: covers.page
    readonly property Item card: covers.all ? covers.fixed.item : covers.shelf.cardItem
    readonly property Item focused: covers.Window.activeFocusItem
    readonly property bool onView: viewHas(focused)
    readonly property bool onCard: cardHas(focused)
    readonly property bool free: !focused || focused === covers.Window.contentItem || inside(page, focused) && !inside(page.bar, focused) && !inside(page.notes, focused)
    readonly property var choose: ({
            "keys": [App.trc("key", "Space")],
            "text": App.tr("Choose")
        })
    readonly property var hints: hintsOf(focused)

    function hintsOf(at) {
        return inside(page.bar.words, at) ? [{
            "keys": ["left", "right"],
            "text": App.tr("Move around")
        }, {
            "keys": [App.trc("key", "Enter")],
            "text": App.tr("Show")
        }, {
            "keys": ["down"],
            "text": App.tr("Back to the wallpapers")
        }] : cardHas(at) ? [{
            "keys": ["Tab"],
            "text": App.tr("Next part")
        }, {
            "keys": ["Esc"],
            "text": page.choosing ? App.tr("Stop choosing") : page.all ? App.tr("Show shelf") : App.tr("Menu")
        }] : !viewHas(at) ? undefined : page.choosing ? [choose, {
            "keys": ["Esc"],
            "text": App.tr("Stop choosing")
        }] : page.all ? [{
            "keys": ["left", "right", "up", "down"],
            "text": App.tr("Move around")
        }, choose, {
            "keys": ["Esc"],
            "text": App.tr("Show shelf")
        }] : [{
            "keys": ["left", "right"],
            "text": App.tr("Move around")
        }, {
            "keys": ["down"],
            "text": App.tr("See all")
        }, {
            "keys": ["up"],
            "text": App.tr("Filters")
        }, choose, {
            "keys": ["Esc"],
            "text": App.tr("Menu")
        }];
    }

    function viewHas(at) {
        return !!at && (at === covers.view.view || at === covers.view.view.currentItem);
    }

    function cardHas(at) {
        return !!at && !viewHas(at) && inside(covers, at);
    }

    function inside(owner, item) {
        for (let at = item; at; at = at.parent)
            if (at === owner)
                return true;
        return false;
    }

    function first(item) {
        for (let i = 0; item && i < item.children.length; i++) {
            const child = item.children[i];
            const found = !child.visible || !child.enabled ? null : child.activeFocusOnTab ? child : first(child);
            if (found)
                return found;
        }
        return null;
    }

    function tab(event) {
        const next = viewHas(covers.Window.activeFocusItem) ? first(card) : null;
        event.accepted = !!next;
        if (next)
            next.forceActiveFocus(Qt.TabFocusReason);
    }

    function backtab(event) {
        const at = covers.Window.activeFocusItem;
        event.accepted = cardHas(at) && !!card && at === first(card);
        if (event.accepted)
            covers.view.view.forceActiveFocus(Qt.BacktabFocusReason);
    }
}
