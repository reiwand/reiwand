import QtQuick
import io.github.reiwand.app

Item {
    id: thumbs

    required property var installed
    property var found: ({})
    readonly property int size: PreviewLook.edge(Look.coverFocus * Screen.devicePixelRatio)
    readonly property var items: installed.map(p => p.item ?? "").filter(uid => uid !== "")

    function uid(pkg) {
        const own = installed.find(p => p.repo_id === pkg.repo_id && p.id === pkg.id && (p.item ?? "") !== "");
        return own ? own.item : "";
    }

    function live(pkg) {
        return pkg.kind === "scene" && uid(pkg) !== "";
    }

    function picture(uid) {
        return found[uid] ? App.url(found[uid]) : "";
    }

    function load() {
        if (items.length > 0)
            App.query("RequestThumbnails", [items, size], "Library1", reply => thumbs && (thumbs.found = Object.assign({}, thumbs.found, reply[0])));
    }

    onItemsChanged: load()
    onSizeChanged: load()

    Connections {
        target: App.backend

        function onThumbnailReady(uid, size, path) {
            if (size === thumbs.size && thumbs.items.includes(uid))
                thumbs.found = Object.assign({}, thumbs.found, {
                    [uid]: path
                });
        }
    }
}
