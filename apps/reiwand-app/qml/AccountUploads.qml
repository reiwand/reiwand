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
    readonly property var kept: versions.filter(v => v.state !== "removed" && v.state !== "rejected").filter((v, i, all) => all.findIndex(w => w.package === v.package) === i)
    property var leaving: null
    readonly property alias list: list
    readonly property alias cancels: cancels
    readonly property alias withdrawals: withdrawals
    readonly property alias asking: ask

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

    function withdraw() {
        App.act("WithdrawPackage", [repo, leaving.package], "Publish1", () => uploads && uploads.load(), uploads.failed);
        ask.close();
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

    Section {
        text: App.tr("My uploads")
    }

    StatusLine {
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

        ListRow {
            required property var modelData

            width: uploads.width
            text: App.tr("%1 %2", modelData.title || modelData.package, modelData.version)
            note: modelData.state
            kind: "choice"
            options: [App.tr("Public"), App.tr("Unlisted")]
            current: modelData.visibility === "unlisted" ? 1 : 0
            explain: App.tr("Unlisted versions stay out of the catalog and search; whoever has their link can still install them.")
            onChosen: index => uploads.reveal(modelData, index)
        }
    }

    Repeater {
        id: cancels

        model: uploads.unfinished

        ListRow {
            required property var modelData

            width: uploads.width
            text: App.tr("Upload not finished")
            note: modelData.id
            kind: "page"
            status: App.tr("Cancel it")
            statusTone: "amber"
            onOpened: App.act("CancelUpload", [uploads.repo, modelData.id], "Publish1", () => uploads && uploads.load(), uploads.failed)
        }
    }

    Repeater {
        id: withdrawals

        model: uploads.kept

        ListRow {
            id: going

            required property var modelData

            width: uploads.width
            text: modelData.title || modelData.package
            note: modelData.package
            words: [{
                    "text": App.tr("Withdraw for good…"),
                    "icon": "trash"
                }]
            onWorded: {
                uploads.leaving = modelData;
                ask.opener = going;
                ask.open();
            }
        }
    }

    SmallPanel {
        id: ask

        title: uploads.leaving ? App.tr("Withdraw %1 for good?", uploads.leaving.title || uploads.leaving.package) : ""
        text: App.tr("Every version leaves the repository and its files are deleted from the server; devices that installed it stop showing it. This cannot be undone. The id stays yours.")
        primary: App.tr("Withdraw")
        primaryIcon: "trash"
        quiet: App.tr("Cancel")
        destructive: true
        onAccepted: uploads.withdraw()
        onRejected: close()
    }
}
