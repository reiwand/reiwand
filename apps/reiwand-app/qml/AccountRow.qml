import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Column {
    id: details

    required property var account
    property string repos
    property string code
    property string link
    readonly property string host: String(account.server).replace(/^[a-z]+:\/\//i, "").replace(/\/+$/, "")
    readonly property alias row: row
    readonly property alias signOut: signOut
    readonly property alias signIn: signIn
    readonly property alias remove: remove
    readonly property alias remover: remover

    function again() {
        const first = account.repos.length > 0 ? account.repos[0].url : account.server;
        App.query("SignIn", [first], "Accounts1", result => {
            if (!details)
                return;
            details.code = result[0];
            details.link = result[1];
        }, failure => details && applicationWindow().showPassiveNotification(failure.message, "long"));
    }

    spacing: Look.e3

    Fila {
        id: row

        width: parent.width
        text: App.tr("%1 at %2", details.account.user, details.host)
        kind: "despliega"
        note: details.repos !== "" ? details.repos : App.tr("Not linked to a repository")
        status: details.account.pending ? App.tr("Sign in again") : App.tr("Signed in")
        statusTone: details.account.pending ? "ambar" : "brote"
    }

    Column {
        visible: row.open
        leftPadding: Look.e5
        spacing: Look.e3

        BrowseTerm {
            text: App.tr("Server · %1", details.account.server)
        }

        BrowseTerm {
            text: App.tr("User · %1", details.account.user)
        }

        BrowseTerm {
            text: details.account.repos.length > 0 ? App.tr("Used for · %1", details.repos) : App.tr("Used for · no repository yet")
        }

        Text {
            visible: details.code !== ""
            text: App.tr("Open %1 and enter the code %2", details.account.server, details.code)
            wrapMode: Text.Wrap
            width: Math.min(parent.width, Look.vacioAncho)
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            color: Look.escarcha
        }

        Row {
            spacing: Look.e3

            QQC2.Button {
                id: signIn

                visible: details.account.pending
                highlighted: true
                text: details.code !== "" ? App.tr("Open in the browser") : App.tr("Sign in again")
                icon.name: "sign-in"
                onClicked: details.code !== "" ? Qt.openUrlExternally(details.link) : details.again()
            }

            QQC2.Button {
                id: signOut

                visible: !details.account.pending
                text: App.tr("Sign out")
                icon.name: "sign-out"
                onClicked: App.act("SignOut", [details.account.id], "Accounts1")
            }

            QQC2.Button {
                id: remove

                text: App.tr("Remove the account…")
                icon.name: "trash"
                onClicked: remover.open()
            }
        }
    }

    PanelChico {
        id: remover

        opener: remove
        title: App.tr("Remove the account of %1?", details.account.user)
        text: App.tr("Its saved access is deleted and its repositories stop using it. The wallpapers you installed stay.")
        principal: App.tr("Remove")
        principalIcon: "trash"
        borra: true
        onAccepted: App.act("Remove", [details.account.id], "Accounts1")
    }
}
