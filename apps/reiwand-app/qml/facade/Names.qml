pragma Singleton
import QtQuick

QtObject {
    id: names

    property var providers: ({})
    readonly property Connections watch: Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "library")
                names.load();
        }
    }

    function load() {
        App.query("ListSources", [], "Library1", reply => names && (names.providers = (reply.sources ?? []).filter(s => s.provider).reduce((all, s) => Object.assign(all, {
                        [s.uid]: s.provider
                    }), {})));
    }

    function stem(path) {
        const file = String(path ?? "").split("/").pop();
        const dot = file.lastIndexOf(".");
        return dot > 0 ? file.slice(0, dot) : file;
    }

    Component.onCompleted: load()

    function of(item) {
        const title = String(item?.title ?? "");
        const file = stem(item?.path);
        const provider = item?.provider ?? providers[item?.source ?? ""] ?? "";
        if (String(item?.provider ?? "") !== "")
            return App.tr("Photo from %1", provider);
        if (title !== "" && !(provider !== "" && title === file))
            return title;
        if (provider !== "")
            return App.tr("Photo from %1", provider);
        return file !== "" ? file : App.tr("Untitled wallpaper");
    }
}
