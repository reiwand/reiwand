import QtQuick
import io.github.reiwand.app

ListaMenu {
    id: menu

    property Item back: null
    property Item spot: Item {}
    property var pending: null
    property TextMetrics metrics: TextMetrics {
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
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
        width = Math.max(Look.menuChico, metrics.advanceWidth + Look.icono + 4 * Look.e3 + 2 * Look.listaHueco);
        options = entries;
        current = -1;
        open();
        first.forceActiveFocus();
    }

    opener: spot
    onChosen: index => pending = options[index].act
    onClosed: {
        back?.forceActiveFocus();
        const act = pending;
        pending = null;
        if (act)
            act();
    }
}
