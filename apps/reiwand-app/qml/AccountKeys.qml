import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Column {
    id: keys

    required property string repo
    property var items: []
    property string problem
    property string pending
    readonly property alias list: list
    readonly property alias add: add
    readonly property alias handing: handing

    signal signedOut

    function failed(error) {
        if (!keys)
            return;
        keys.problem = error.message;
        if (String(error.name).endsWith(".SignedOut"))
            keys.signedOut();
    }

    function load() {
        App.query("ListAuthorKeys", [repo], "Publish1", reply => {
            if (!keys)
                return;
            keys.items = reply.items ?? [];
            keys.problem = "";
        }, keys.failed);
    }

    function call(name, args) {
        keys.problem = "";
        App.act(name, [repo].concat(args), "Publish1", () => keys && keys.load(), keys.failed);
    }

    spacing: Look.e2
    Component.onCompleted: load()

    Section {
        text: App.tr("Author keys")
    }

    BrowseTerm {
        text: App.tr("The public keys with which you sign the packages you upload. Create a pair in the build dialog of the editor; only the public key goes here.")
        small: true
    }

    StatusLine {
        visible: keys.problem !== ""
        text: keys.problem
        tone: "coral"
        icon: "warning"
    }

    Repeater {
        id: list

        model: keys.items

        Column {
            required property var modelData

            width: keys.width
            spacing: Look.e2

            ListRow {
                id: row

                width: parent.width
                text: modelData.fingerprint
                kind: "expand"
                status: App.tr("Registered")
                statusTone: "sprout"
            }

            Row {
                visible: row.open
                leftPadding: Look.e5
                spacing: Look.e3

                QQC2.Button {
                    id: handOver

                    text: App.tr("Hand over…")
                    icon.name: "go-next"
                    onClicked: {
                        keys.pending = modelData.fingerprint;
                        handing.opener = handOver;
                        handing.open();
                    }
                }

                QQC2.Button {
                    text: App.tr("Remove")
                    icon.name: "trash"
                    onClicked: keys.call("RemoveAuthorKey", [modelData.fingerprint])
                }
            }
        }
    }

    QQC2.Button {
        id: add

        text: App.tr("Register a key from a file…")
        icon.name: "list-add"
        onClicked: files.open()
    }

    SmallPanel {
        id: handing

        title: App.tr("Hand over to a new key")
        text: App.tr("Sign the hand-over with the old key in Manage keys… of the step “Sign it” when you publish, and paste the signature here.")
        quiet: App.tr("Cancel")
        primary: App.tr("OK")
        onAccepted: keys.call("RotateAuthorKey", [keys.pending, successor.text.trim(), signature.text.trim()])
        onClosed: {
            successor.text = "";
            signature.text = "";
        }

        QQC2.Label {
            width: handing.width - 2 * Look.e4
            text: App.tr("New public key:")
        }

        QQC2.TextField {
            id: successor

            width: handing.width - 2 * Look.e4
            Accessible.name: App.tr("New public key:")
        }

        QQC2.Label {
            width: handing.width - 2 * Look.e4
            text: App.tr("Signature:")
        }

        Area {
            id: signature

            width: handing.width - 2 * Look.e4
            font.family: "monospace"
            Accessible.name: App.tr("Signature:")
        }
    }

    Chooser {
        id: files

        nameFilters: [App.tr("Public keys") + " (*.pub)"]
        onAccepted: keys.call("AddAuthorKey", [App.readText(selectedFile)])
    }
}
