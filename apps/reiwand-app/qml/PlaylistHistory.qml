import QtQuick
import io.github.reiwand.app

Column {
    id: history

    property bool active: false
    property int screen: 0
    property var entries: []
    readonly property string output: App.outputs[screen] ?? ""

    function load() {
        if (active && row.open && output !== "")
            App.query("History", [output, 50], "Playlists1", reply => history && reply.output === history.output && (history.entries = reply.entries));
    }

    onActiveChanged: load()
    onOutputChanged: load()
    spacing: Look.e2

    ListRow {
        id: row

        width: history.width
        text: App.tr("History")
        kind: "page"
        note: App.tr("What it showed lately, the newest first")
        onOpened: {
            open = !open;
            history.load();
        }
    }

    ListRow {
        width: history.width
        visible: row.open && App.outputs.length > 1
        text: App.tr("Which screen")
        kind: "choice"
        options: App.outputs.map(name => App.screen(name))
        current: history.screen
        onChosen: index => history.screen = index
    }

    Sublist {
        width: history.width
        visible: row.open
        removeText: ""
        model: history.entries.map(entry => ({
                    "name": entry.item ? entry.item.title : App.tr("an item that was removed"),
                    "where": App.time(entry.at)
                }))
    }
}
