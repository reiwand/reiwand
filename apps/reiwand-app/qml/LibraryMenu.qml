import QtQuick
import io.github.reiwand.app

Item {
    id: menu

    required property var page
    property var entry: null
    readonly property alias list: list
    readonly property alias scene: scene
    readonly property bool hurt: !!entry && (entry.missing === true || String(entry.broken ?? "") !== "")
    property var layering: entry === null ? null : Capas.of(String(entry.uid ?? ""))
    readonly property bool retry: !!layering && (layering.state === "failed" || layering.state === "cancelled")
    readonly property var picks: entry === null ? [] : offered(hurt, retry)

    function offered(hurt, retry) {
        const found = [
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
                "off": menu.entry.kind !== "scene" || hurt
            }
        ];
        if (retry)
            found.push({
                "kind": "retry",
                "text": App.trc("layers", "Try again"),
                "icon": "arrow-clockwise"
            });
        return found;
    }

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
            else if (kind === "retry")
                App.act("RetryLayers", [menu.layering.file]);
        }
    }
}
