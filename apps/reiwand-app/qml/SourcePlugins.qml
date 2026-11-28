import QtQuick
import io.github.reiwand.app

FocusScope {
    id: plugged

    required property var plugins
    property var offer: null
    property string path
    property var leaving: null
    readonly property alias asking: ask
    readonly property alias removing: drop
    readonly property alias zip: zip
    readonly property alias folder: folder

    signal closed
    signal changed

    function enter() {
        fromZip.forceActiveFocus(Qt.TabFocusReason);
    }

    function inspect(chosen) {
        path = chosen;
        App.query("InspectPlugin", [chosen], "Providers1", reply => plugged && plugged.offered(reply), e => applicationWindow().showPassiveNotification(e.message));
    }

    function offered(reply) {
        offer = reply;
        later.restart();
    }

    function install() {
        const title = offer.plugin.title;
        App.act("InstallPlugin", [path, offer.token], "Providers1", () => plugged && plugged.installed(title), e => applicationWindow().showPassiveNotification(e.message));
        ask.close();
    }

    function installed(title) {
        applicationWindow().showPassiveNotification(App.tr("%1 is installed. Add it as a source from Add a source.", title));
        changed();
    }

    function asked(p) {
        const lines = [App.tr("Version %1, license %2.", p.version, p.license), App.tr("It contacts %1.", p.hosts.join(", "))];
        if (offer.installed)
            lines.push(offer.added_hosts.length > 0 ? App.tr("It replaces version %1 and now also contacts %2.", offer.installed, offer.added_hosts.join(", ")) : App.tr("It replaces version %1 and contacts no new place.", offer.installed));
        if (offer.step === "same")
            lines.push(App.tr("It is the version already installed."));
        else if (offer.step === "older")
            lines.push(App.tr("It is older than the installed version %1: installing it goes back.", offer.installed));
        if (p.secret)
            lines.push(p.secret.required ? App.tr("It asks for a key, %1, sent only to %2.", p.secret.label, p.secret.hosts.join(", ")) : App.tr("It may use a key, %1, sent only to %2.", p.secret.label, p.secret.hosts.join(", ")));
        else
            lines.push(App.tr("It asks for no key."));
        if (offer.secret_changed)
            lines.push(App.tr("The key it asks for changed since the installed version."));
        return lines.join("\n");
    }

    implicitHeight: rows.implicitHeight + Look.e3 + tail.implicitHeight
    Keys.onEscapePressed: closed()

    ListRows {
        id: rows

        Section {
            text: plugged.plugins.length > 0 ? App.tr("Installed plugins") : App.tr("No plugins installed")
        }

        Repeater {
            model: plugged.plugins

            delegate: ListRow {
                id: row

                required property var modelData

                text: modelData.title
                value: App.tr("Version %1", modelData.version)
                status: modelData.error ? App.tr("Does not work") : ""
                statusTone: "amber"
                note: modelData.sources.length > 0 ? App.tr("Used by %1", modelData.sources.join(", ")) : App.tr("No source uses it")
                explain: [App.tr("It contacts %1.", modelData.hosts.join(", ")), modelData.secret ? App.tr("It asks for a key: %1.", modelData.secret.label) : App.tr("It asks for no key."), modelData.error || ""].filter(l => l).join("\n")
                words: [{
                        "text": App.tr("Remove…"),
                        "icon": "trash"
                    }]
                onWorded: {
                    plugged.leaving = modelData;
                    drop.opener = row;
                    drop.open();
                }
            }
        }

        ListRow {
            id: fromZip

            kind: "expand"
            text: App.tr("Install a plugin from a zip file")
            onOpened: zip.open()
        }

        ListRow {
            id: fromFolder

            kind: "expand"
            text: App.tr("Install a plugin from a folder")
            onOpened: folder.open()
        }
    }

    Column {
        id: tail

        anchors.top: rows.bottom
        anchors.topMargin: Look.e3
        width: rows.width
        spacing: Look.e2

        Note {
            width: Math.min(Look.rowsWidth, plugged.width)
            wraps: true
            text: App.tr("A provider plugin adds a new kind of source. It is readable code that runs closed off: it reaches only the places it names, which you see before installing, and never your files.")
        }
    }

    // the file dialog gives the focus back to its row when its closing ends, which can come after
    // the daemon's answer: the panel opens once that is over, so it keeps the keys
    Timer {
        id: later

        interval: 300
        onTriggered: ask.open()
    }

    Chooser {
        id: zip

        title: App.tr("Install a provider plugin")
        nameFilters: [App.tr("Provider plugins") + " (*.zip)"]
        onAccepted: {
            ask.opener = fromZip;
            plugged.inspect(App.path(selectedFile));
        }
    }

    Chooser {
        id: folder

        mode: Chooser.Folder
        title: App.tr("Install a provider plugin")
        onAccepted: {
            ask.opener = fromFolder;
            plugged.inspect(App.path(selectedFolder));
        }
    }

    SmallPanel {
        id: ask

        opener: fromZip
        title: !plugged.offer ? "" : plugged.offer.installed ? App.tr("Update %1?", plugged.offer.plugin.title) : App.tr("Install %1?", plugged.offer.plugin.title)
        text: plugged.offer ? plugged.asked(plugged.offer.plugin) : ""
        primary: plugged.offer && plugged.offer.installed ? App.tr("Update") : App.tr("Install")
        quiet: App.tr("Cancel")
        onAccepted: plugged.install()
        onRejected: close()
    }

    SmallPanel {
        id: drop

        title: App.tr("Remove this plugin?")
        text: !plugged.leaving ? "" : plugged.leaving.sources.length > 0 ? App.tr("%1 is the plugin of %2. Remove those sources first.", plugged.leaving.title, plugged.leaving.sources.join(", ")) : App.tr("%1 is deleted from this computer. Install it again to use it.", plugged.leaving.title)
        primary: plugged.leaving && plugged.leaving.sources.length === 0 ? App.tr("Remove") : ""
        primaryIcon: "trash"
        quiet: plugged.leaving && plugged.leaving.sources.length === 0 ? App.tr("Cancel") : App.tr("Close")
        destructive: true
        onAccepted: {
            App.act("RemovePlugin", [plugged.leaving.id], "Providers1", () => plugged && plugged.changed(), e => applicationWindow().showPassiveNotification(e.message));
            close();
        }
        onRejected: close()
    }
}
