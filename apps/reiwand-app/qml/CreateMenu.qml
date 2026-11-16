import QtQuick
import io.github.reiwand.app

ListaMenu {
    id: menu

    required property var page
    property var entry: null
    property var row: null
    readonly property bool inGrid: row !== null && row === page.every.grid
    readonly property var picks: entry === null ? [] : [
        {
            "kind": "open",
            "text": entry.missing ? App.tr("Find it…") : App.tr("Keep editing"),
            "icon": entry.missing ? "magnifying-glass" : "pencil-simple"
        },
        {
            "kind": "hide",
            "text": App.tr("Remove from recents"),
            "icon": "clock-counter-clockwise",
            "off": entry.recent === false
        },
        {
            "separator": true
        },
        {
            "kind": "drop",
            "text": App.tr("Remove from the list"),
            "icon": "trash",
            "tone": "coral",
            "off": !inGrid && entry.missing !== true
        }
    ]

    function ask(from, at) {
        const chosen = from.current;
        if (!chosen || chosen.all === true)
            return;
        entry = chosen;
        row = from;
        opener = at;
        current = 0;
        open();
    }

    title: entry ? entry.name : ""
    options: picks
    onChosen: index => {
        const kind = picks[index].kind;
        if (kind === "open")
            page.act(row);
        else if (kind === "hide")
            page.facts.hide(entry.folder);
        else if (kind === "drop")
            page.facts.drop(entry.folder, entry.name);
    }
}
