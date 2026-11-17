import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

ColumnLayout {
    id: server

    required property var where
    readonly property var publish: where.publish
    readonly property alias fault: fault
    readonly property alias targets: targets
    readonly property alias choices: choices
    readonly property alias reply: reply

    function entries() {
        return targets.candidates.map(r => ({
                    "label": (r.name || r.url) + " · " + (/^[a-z]+:\/\/([^/?#]+)/i.exec(r.url) ?? ["", ""])[1],
                    "icon": r.url === targets.chosen ? "radio-button-fill" : "circle",
                    "value": r.url
                }));
    }

    Layout.fillWidth: true
    spacing: Look.e1

    PublishTargets {
        id: targets

        publish: server.publish
    }

    Connections {
        target: App.backend

        function onJobFinished(id, state, name, message) {
            targets.ended(id, state, message);
        }

        function onChanged(topic) {
            if (topic === "repos" || topic === "accounts")
                targets.load();
        }
    }

    PublishPlace {
        icon: "cloud-arrow-up"
        title: App.tr("In a repository")
        note: targets.repo ? (targets.account ? App.tr("%1, with the account %2", targets.name, targets.account.user) : targets.name) : targets.candidates.length > 0 ? App.tr("One of the repositories you added that takes uploads.") : App.tr("None of your repositories takes uploads yet.")
        lit: server.publish.place === "server"
        off: targets.candidates.length === 0
        last: server.publish.memory.place === "server" && targets.candidates.length > 0
        onPicked: server.where.pick("server")

        RepoAddChoices {
            id: choices

            Layout.fillWidth: true
            visible: targets.candidates.length > 0
            entries: server.entries()
            onChosen: value => targets.choose(value)
        }

        RepoAddAnswer {
            id: reply

            Layout.fillWidth: true
            publishing: true
            stage: targets.asking && server.publish.place === "server" ? targets.stage === "approve" ? "approve" : targets.detail !== "" ? "failed" : "account" : ""
            host: targets.host
            detail: targets.detail
            code: targets.code
            link: targets.link
            accounts: targets.known
            onUseAccount: id => targets.use(id)
            onSignIn: targets.signIn()
            onOpened: Qt.openUrlExternally(link)
        }

        Opcion {
            Layout.fillWidth: true
            icon: server.where.hidden ? "circle" : "radio-button-fill"
            lit: !server.where.hidden
            text: App.tr("Public")
            note: App.tr("in the catalog and its search")
            onClicked: server.where.hidden = false
        }

        Opcion {
            Layout.fillWidth: true
            icon: server.where.hidden ? "radio-button-fill" : "circle"
            lit: server.where.hidden
            text: App.tr("Hidden")
            note: App.tr("only whoever has the link; it is not private")
            onClicked: server.where.hidden = true
        }

        PublishNote {
            Layout.fillWidth: true
            visible: !server.publish.unsigned
            text: App.tr("The first time, your signature is registered in your account by itself; we let you know.")
        }

        PublishFault {
            id: fault

            Layout.fillWidth: true
            where: server.where
            place: "server"
        }
    }

    QQC2.Button {
        Layout.leftMargin: PublishLook.iconoDestino + 2 * Look.e3
        visible: targets.candidates.length === 0
        flat: true
        text: App.tr("Add a repository in Explore ›")
        onClicked: applicationWindow().activate("add:")
    }
}
