import QtQuick
import io.github.reiwand.app

Item {
    id: menu

    required property var page
    property var entry: null
    readonly property alias list: list
    readonly property alias scene: scene
    readonly property bool hurt: !!entry && (entry.missing === true || String(entry.broken ?? "") !== "")
    readonly property var picks: entry === null ? [] : [
        {
            "kind": "set",
            "text": hurt ? App.tr("Show in folder") : App.tr("Set as wallpaper"),
            "icon": hurt ? "folder-open" : "image"
        },
        {
            "kind": "options",
            "text": App.tr("Options"),
            "icon": "caret-right"
        },
        {
            "kind": "copy",
            "text": App.tr("Edit a copy…"),
            "icon": "edit-copy",
            "off": entry.kind !== "scene" || hurt
        }
    ]

    function copy(uid) {
        scene.start(uid);
    }

    function ask(at) {
        if (!page.entry)
            return;
        entry = page.entry;
        list.opener = at;
        list.current = 0;
        list.open();
    }

    SceneCopy {
        id: scene
    }

    ListaMenu {
        id: list

        title: menu.entry ? menu.entry.title ?? "" : ""
        options: menu.picks
        onChosen: index => {
            const kind = menu.picks[index].kind;
            if (kind === "set")
                menu.page.activate();
            else if (kind === "options")
                menu.page.open(menu.entry.uid);
            else if (kind === "copy")
                menu.copy(menu.entry.uid);
        }
    }
}
