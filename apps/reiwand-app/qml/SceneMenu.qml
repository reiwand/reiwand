import QtQuick
import io.github.reiwand.app

OptionMenu {
    id: menu

    property Item back: null
    property Item spot: Item {}
    property var pending: null
    property TextMetrics metrics: TextMetrics {
        font.family: Look.textFont
        font.pixelSize: DeskLook.tMenu
        font.weight: Font.DemiBold
    }

    function show(item, x, y, entries) {
        if (entries.length === 0)
            return;
        back = item.Window.activeFocusItem;
        spot.parent = item;
        spot.x = x;
        spot.y = y - Look.e2;
        const longest = entries.reduce((most, entry) => (entry.text ?? "").length > most.length ? entry.text : most, "");
        metrics.text = longest;
        const planned = entries.some(entry => entry.plan);
        width = Math.max(Look.menuSmall, metrics.advanceWidth + (planned ? DeskLook.planMenuWidth : Look.icon) + 6 * Look.e3 + 2 * Look.listGap);
        options = entries;
        current = -1;
        open();
        first.forceActiveFocus();
    }

    opener: spot
    rowHeight: DeskLook.menuRow
    textSize: DeskLook.tMenu
    onChosen: index => pending = options[index].act
    onClosed: {
        back?.forceActiveFocus();
        const act = pending;
        pending = null;
        if (act)
            act();
    }
}
