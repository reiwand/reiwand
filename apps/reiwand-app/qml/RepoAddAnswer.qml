import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Column {
    id: answer

    property string stage
    property string detail
    property string digest
    property string host
    property string code
    property string link
    property var accounts: []
    property bool publishing: false
    property var found: []
    readonly property alias choices: choices
    readonly property alias trust: trust
    readonly property alias another: another
    readonly property alias open: open
    readonly property alias servers: servers
    readonly property bool asking: ["account", "certificate", "server", "approve", "warning", "failed"].includes(stage)

    signal useAccount(string id)
    signal signIn
    signal trusted
    signal picked(string url)
    signal opened

    visible: asking
    spacing: Look.e3

    function head() {
        return {
            "account": publishing ? App.tr("Publishing here needs an account") : App.tr("This repository is private"),
            "certificate": App.tr("This server uses its own certificate"),
            "server": App.tr("This is the address of a server"),
            "approve": App.tr("Approve the sign-in in your browser"),
            "warning": App.tr("It was not added"),
            "failed": App.tr("It was not added")
        }[stage] ?? "";
    }

    function body() {
        if (stage === "account")
            return publishing ? (accounts.length > 0 ? App.tr("Use one of your accounts on %1 or sign in with another one to publish here.", host) : App.tr("Sign in with an account of %1 to publish here.", host)) : accounts.length > 0 ? App.tr("Use one of your accounts on %1 or sign in with another one.", host) : App.tr("Sign in with an account of %1 to open it.", host);
        if (stage === "certificate")
            return App.tr("Its fingerprint is %1. Add it only if you trust this server: the fingerprint is then remembered for this server alone.", digest);
        if (stage === "server")
            return found.length > 0 ? App.tr("Pick one of its repositories.") : detail;
        if (stage === "approve")
            return App.tr("Open this address and enter the code. The repository is added once you approve.");
        return detail;
    }

    Text {
        width: parent.width
        text: answer.head()
        wrapMode: Text.WordWrap
        font.family: Look.titleFont
        font.pixelSize: Look.tBody
        font.weight: Font.Bold
        color: answer.stage === "warning" ? Look.coral : Look.frost
    }

    Text {
        width: parent.width
        text: answer.body()
        wrapMode: Text.Wrap
        font.family: Look.textFont
        font.pixelSize: Look.tBody
        color: Look.mist
    }

    Text {
        visible: answer.stage === "approve"
        width: parent.width
        text: answer.code
        font.family: Look.titleFont
        font.pixelSize: Look.tRow
        font.weight: Font.Bold
        color: Look.frost
    }

    Text {
        visible: answer.stage === "approve"
        width: parent.width
        text: answer.link
        elide: Text.ElideMiddle
        font.family: Look.textFont
        font.pixelSize: Look.tFoot
        color: Look.mist
    }

    RepoAddChoices {
        id: choices

        visible: answer.stage === "account" && answer.accounts.length > 0
        width: parent.width
        entries: answer.accounts.map(a => ({
                    "label": App.tr("Use the account %1", a.user),
                    "icon": "identification-card",
                    "value": a.id
                }))
        onChosen: value => answer.useAccount(value)
    }

    RepoAddChoices {
        id: servers

        visible: answer.stage === "server" && answer.found.length > 0
        width: parent.width
        entries: answer.found.map(f => ({
                    "label": f.name ? f.name + " · " + f.url : f.url,
                    "icon": "link-simple",
                    "value": f.url
                }))
        onChosen: value => answer.picked(value)
    }

    QQC2.Button {
        id: another

        visible: answer.stage === "account"
        text: answer.accounts.length > 0 ? App.tr("Use another account") : App.tr("Sign in")
        icon.name: "sign-in"
        onClicked: answer.signIn()
        Keys.onReturnPressed: clicked()
        Keys.onEnterPressed: clicked()
    }

    QQC2.Button {
        id: trust

        visible: answer.stage === "certificate"
        highlighted: true
        text: App.tr("Trust and add")
        icon.name: "shield-check"
        onClicked: answer.trusted()
        Keys.onReturnPressed: clicked()
        Keys.onEnterPressed: clicked()
    }

    QQC2.Button {
        id: open

        visible: answer.stage === "approve"
        text: App.tr("Open in the browser")
        icon.name: "link-simple"
        onClicked: answer.opened()
        Keys.onReturnPressed: clicked()
        Keys.onEnterPressed: clicked()
    }
}
