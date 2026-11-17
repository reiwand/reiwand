import QtQuick
import io.github.reiwand.app

Timer {
    id: check

    required property var host
    property var publish: null
    readonly property Stall probe: Stall {}
    readonly property string home: "https://a.example/r/one/"

    function expect(ok, what) {
        if (!ok)
            throw new Error("publishing destination: " + what);
    }

    function repo(url, extra) {
        return Object.assign({
            "repo_id": "id" + url.length,
            "url": url,
            "name": "",
            "bundled": false
        }, extra || {});
    }

    function account(id, user, linked) {
        return {
            "id": id,
            "server": "https://a.example",
            "user": user,
            "pending": false,
            "repos": linked ? [{
                    "url": check.home,
                    "repo_id": "id" + check.home.length
                }] : []
        };
    }

    function run() {
        const sample = JSON.parse(probe.sample("target"));
        publish = Qt.createComponent("Publish.qml").createObject(host.contentItem, {
            "width": 900,
            "height": 600
        });
        publish.openWith(sample.wpak);
        publish.memory.target = "";
        const where = publish.where;
        const targets = where.server.targets;
        const repos = [repo(home, {
                "name": "One"
            }), repo("https://b.example/r/two/"), repo("file:///examples", {
                "bundled": true
            }), repo("https://c.example/fondos")];
        targets.have([repos[2], repos[3]], []);
        expect(targets.candidates.length === 0 && where.server.choices.entries.length === 0, "a folder, an example or a plain address must not be offered");
        targets.have(repos, []);
        expect(where.server.choices.entries.length === 2, "the repositories that accept uploads must be listed");
        expect(targets.repo === null && publish.account === null && !where.can, "nothing is chosen while two repositories are offered");
        where.server.choices.currentIndex = 0;
        where.server.choices.pick();
        expect(targets.chosen === home && publish.memory.target === home, "picking a repository must choose and remember it");
        expect(where.server.reply.visible && where.server.reply.stage === "account" && where.server.reply.another.text === App.tr("Sign in"), "with no account the repository asks to sign in");
        expect(publish.account === null && !where.can, "without an account it cannot publish");
        targets.have(repos, [account("a1", "maker", false), Object.assign(account("a2", "other", false), {
                "server": "https://z.example"
            })]);
        expect(where.server.reply.choices.entries.length === 1 && where.server.reply.choices.entries[0].label === App.tr("Use the account %1", "maker"), "only the accounts of that server must be offered");
        expect(where.server.reply.another.text === App.tr("Use another account"), "another account must be offered");
        targets.approving(["ABCD-EFGH", "https://a.example/link", 31]);
        expect(where.server.reply.stage === "approve" && where.server.reply.code === "ABCD-EFGH" && where.server.reply.open.visible, "the sign-in must show its code and its link");
        targets.ended(31, "failed", "denied");
        expect(where.server.reply.stage === "failed" && where.server.reply.detail === "denied", "a refused sign-in must say so");
        targets.have(repos, [account("a1", "maker", true)]);
        expect(publish.account !== null && publish.account.repo === home && publish.account.id === "a1" && where.can, "a linked account must be the account that publishes");
        expect(!where.server.reply.visible, "no prompt while the repository has its account");
        targets.have(repos, []);
        expect(publish.account === null && !where.can, "losing the account must stop the publishing");
        publish.destroy();
        host.waiting--;
        host.held = host.held.filter(c => c !== check);
    }

    function start() {
        host.waiting++;
        host.held = host.held.concat([check]);
        running = true;
    }

    interval: 20
    onTriggered: {
        running = false;
        run();
    }
}
