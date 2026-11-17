import QtQuick
import io.github.reiwand.app

Column {
    id: uploads

    required property string repo
    property var versions: []
    property var sessions: []
    property string problem
    property bool lost: false
    readonly property var unfinished: sessions.filter(s => s.state === "open")
    readonly property alias list: list
    readonly property alias cancels: cancels

    signal signedOut

    function failed(error) {
        if (!uploads)
            return;
        uploads.problem = error.message;
        if (String(error.name).endsWith(".SignedOut")) {
            uploads.lost = true;
            uploads.signedOut();
        }
    }

    function load() {
        App.query("ListPackages", [repo], "Publish1", reply => {
            if (!uploads)
                return;
            uploads.versions = reply.items ?? [];
            uploads.problem = "";
        }, uploads.failed);
        App.query("ListUploads", [repo], "Publish1", reply => uploads && (uploads.sessions = reply.uploads ?? []), uploads.failed);
    }

    function reveal(item, index) {
        App.act("SetVisibility", [repo, item.package, item.version, index === 1 ? "unlisted" : "public"], "Publish1", () => uploads && uploads.load(), uploads.failed);
    }

    spacing: Look.e2
    Component.onCompleted: load()

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "jobs" || topic === "accounts")
                uploads.load();
        }
    }

    Seccion {
        text: App.tr("My uploads")
    }

    Estado {
        visible: uploads.problem !== ""
        text: uploads.problem
        tone: "coral"
        icon: "warning"
    }

    BrowseTerm {
        visible: uploads.versions.length === 0 && uploads.unfinished.length === 0 && uploads.problem === ""
        text: App.tr("Nothing uploaded to this repository yet")
        small: true
    }

    Repeater {
        id: list

        model: uploads.versions

        Fila {
            required property var modelData

            width: uploads.width
            text: App.tr("%1 %2", modelData.title || modelData.package, modelData.version)
            note: modelData.state
            kind: "elige"
            options: [App.tr("Public"), App.tr("Unlisted")]
            current: modelData.visibility === "unlisted" ? 1 : 0
            explain: App.tr("Unlisted versions stay out of the catalog and search; whoever has their link can still install them.")
            onChosen: index => uploads.reveal(modelData, index)
        }
    }

    Repeater {
        id: cancels

        model: uploads.unfinished

        Fila {
            required property var modelData

            width: uploads.width
            text: App.tr("Upload not finished")
            note: modelData.id
            kind: "pliega"
            status: App.tr("Cancel it")
            statusTone: "ambar"
            onOpened: App.act("CancelUpload", [uploads.repo, modelData.id], "Publish1", () => uploads && uploads.load(), uploads.failed)
        }
    }
}
