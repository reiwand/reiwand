import QtQuick
import QtQuick.Layouts
import QtCore as Core
import io.github.reiwand.app

ColumnLayout {
    id: note

    property string playlist
    property var items: []
    property var lists: ({})
    readonly property var gone: JSON.parse(memory.dismissed)
    readonly property var shown: items.filter(a => (!playlist || (a.playlists ?? []).includes(playlist)) && !gone.includes(a.key))

    function load() {
        App.query("ListAway", [], "Sync1", reply => note && (note.items = reply.items ?? []));
        App.query("List", [], "Playlists1", reply => note && (note.lists = (reply.playlists ?? []).reduce((all, p) => Object.assign(all, {
                        [p.uid]: p.name
                    }), {})));
    }

    function says(away) {
        const name = Names.of(away);
        if (away.repo)
            return App.tr("“%1”, synced from your other computers, comes from the repository “%2”, which is not added on this computer.", name, away.repo);
        const send = App.tr("On the computer that has it, use “Send files to my other computers”.");
        const list = (away.playlists ?? []).map(uid => note.lists[uid]).find(n => n);
        if (list)
            return App.tr("“%1” is in your list “%2”, synced from your other computers, but its file is not on this computer.", name, list) + " " + send;
        const screen = (away.profiles ?? []).map(p => p.output)[0];
        if (screen)
            return App.tr("“%1” is set on %2 by a profile synced from your other computers, but its file is not on this computer.", name, screen) + " " + send;
        return App.tr("“%1” comes from your other computers, but its file is not on this computer.", name) + " " + send;
    }

    function dismiss(key) {
        memory.dismissed = JSON.stringify(gone.concat([key]));
    }

    visible: shown.length > 0
    spacing: Look.e2
    Component.onCompleted: load()

    Core.Settings {
        id: memory

        property string dismissed: "[]"

        category: "away"
        location: Core.StandardPaths.writableLocation(Core.StandardPaths.ConfigLocation) + "/reiwand/away.conf"
    }

    Connections {
        target: App

        function onUpdated(topic) {
            if (["account", "library", "playlists"].includes(topic))
                note.load();
        }
    }

    Repeater {
        model: note.shown

        Aviso {
            required property var modelData

            Layout.fillWidth: true
            text: note.says(modelData)
            action: modelData.repo ? App.tr("Add the repository") : ""
            onActed: applicationWindow().activate("repositories")
            onClosed: note.dismiss(modelData.key)
        }
    }
}
