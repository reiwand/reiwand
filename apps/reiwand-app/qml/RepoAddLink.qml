import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

SmallPanel {
    id: adder

    readonly property alias address: address
    readonly property alias reply: reply
    readonly property alias submitButton: submitButton
    property string stage: "ask"
    property string kind: "add"
    property var job: -1
    property var seen: ({})
    property string fingerprint

    function offer(url) {
        reset();
        address.text = url;
        open();
    }

    function reset() {
        stage = "ask";
        job = -1;
        reply.detail = "";
    }

    function site() {
        return (/^([a-z][a-z0-9+.-]*:\/\/[^/?#]+)/i.exec(address.text.trim()) ?? ["", ""])[1].toLowerCase();
    }

    function submit() {
        if (address.text.trim() === "" || stage === "working")
            return;
        begin("");
    }

    function begin(trust) {
        const url = address.text.trim();
        stage = "working";
        kind = "add";
        job = -1;
        const ok = reply => adder && adder.started(reply[0]);
        const bad = error => adder && adder.told(error.name, error.message);
        if (trust === "")
            App.act("Add", [url], "Repos1", ok, bad);
        else
            App.act("AddTrusting", [url, trust], "Repos1", ok, bad);
    }

    function started(id) {
        job = id;
        const done = seen[id];
        if (done)
            ended(id, done.state, done.name, done.message);
    }

    function ended(id, state, name, message) {
        if (id !== job)
            return;
        job = -1;
        if (state === "done")
            kind === "add" ? added() : begin("");
        else if (String(name).endsWith("NotARepository"))
            App.query("GetJob", [id], "Daemon1", reply => adder && adder.told(name, message, reply.job.output?.repositories ?? []), () => adder && adder.told(name, message));
        else
            told(name, message);
    }

    function added() {
        address.text = "";
        close();
        applicationWindow().showPassiveNotification(App.tr("Repository added."), "long");
    }

    function told(name, message, found = []) {
        const reason = String(name).split(".").pop();
        reply.detail = message;
        if (reason === "CertificateUnknown" && /fingerprint is ([0-9a-f]+)/.test(message)) {
            fingerprint = /fingerprint is ([0-9a-f]+)/.exec(message)[1];
            reply.digest = fingerprint;
            stage = "certificate";
        } else if (reason === "AccountNeeded") {
            App.query("List", [], "Accounts1", list => adder && adder.listed(list.accounts), () => adder && adder.listed([]));
        } else if (reason === "NotARepository") {
            reply.found = found;
            stage = "server";
        } else if (reason === "CertificateChanged" || reason === "SignatureChanged") {
            stage = "warning";
        } else {
            stage = "failed";
        }
        Qt.callLater(() => adder && adder.settle());
    }

    function listed(accounts) {
        reply.host = site().replace(/^[a-z][a-z0-9+.-]*:\/\//i, "");
        reply.accounts = accounts.filter(a => !a.pending && a.server.replace(/\/$/, "").toLowerCase() === site());
        stage = "account";
        Qt.callLater(() => adder && adder.settle());
    }

    function use(id) {
        stage = "working";
        App.act("Use", [address.text.trim(), id], "Accounts1", () => adder && adder.begin(""), error => adder && adder.told(error.name, error.message));
    }

    function signIn() {
        stage = "working";
        App.query("SignIn", [address.text.trim()], "Accounts1", result => adder && adder.approving(result), error => adder && adder.told(error.name, error.message));
    }

    function approving(result) {
        reply.code = result[0];
        reply.link = result[1];
        kind = "signin";
        stage = "approve";
        started(result[2]);
        Qt.callLater(() => adder && adder.settle());
    }

    function settle() {
        const target = stage === "account" ? (reply.choices.visible ? reply.choices : reply.another) : stage === "certificate" ? reply.trust : stage === "server" && reply.servers.visible ? reply.servers : stage === "approve" ? reply.open : address;
        target.forceActiveFocus(Qt.TabFocusReason);
    }

    title: App.tr("Add a repository")
    text: App.tr("Paste the address of a repository of yours or someone else's")
    quiet: App.tr("Close")
    first: address
    onClosed: reset()

    foot: [
        QQC2.Button {
            id: submitButton

            visible: adder.stage === "ask" || adder.stage === "working"
            enabled: adder.stage === "ask" && address.text.trim() !== ""
            highlighted: true
            text: App.tr("Add")
            icon.name: "link-simple"
            onClicked: adder.submit()
            Keys.onReturnPressed: clicked()
            Keys.onEnterPressed: clicked()
        }
    ]

    QQC2.TextField {
        id: address

        width: parent.width - 2 * parent.padding
        placeholderText: App.tr("Example: https://home.example/wallpapers")
        onTextEdited: adder.stage !== "ask" && adder.reset()
        onAccepted: adder.submit()
    }

    Text {
        visible: adder.stage === "working"
        width: parent.width - 2 * parent.padding
        text: App.tr("Checking the repository…")
        font.family: Look.textFont
        font.pixelSize: Look.tBody
        color: Look.mist
    }

    RepoAddAnswer {
        id: reply

        width: parent.width - 2 * parent.padding
        stage: adder.stage
        onUseAccount: id => adder.use(id)
        onSignIn: adder.signIn()
        onTrusted: adder.begin(adder.fingerprint)
        onPicked: url => {
            address.text = url;
            adder.submit();
        }
        onOpened: Qt.openUrlExternally(link)
    }

    Connections {
        target: App.backend

        function onJobFinished(id, state, name, message) {
            const keep = Object.keys(adder.seen).slice(-15);
            const next = {};
            keep.forEach(k => next[k] = adder.seen[k]);
            next[id] = {
                "state": state,
                "name": name,
                "message": message
            };
            adder.seen = next;
            adder.ended(id, state, name, message);
        }
    }
}
