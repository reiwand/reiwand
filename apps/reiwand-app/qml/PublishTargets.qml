import QtQuick
import io.github.reiwand.app

QtObject {
    id: targets

    required property var publish
    property var repos: []
    property var accounts: []
    property string chosen: ""
    property string stage: ""
    property string code
    property string link
    property string detail
    property var job: -1
    readonly property var candidates: repos.filter(r => !r.bundled && /^[a-z][a-z0-9+.-]*:\/\/[^/?#]+\/r\/[^/?#]+/i.test(r.url))
    readonly property var repo: candidates.find(r => r.url === chosen) ?? null
    readonly property string site: repo ? (/^([a-z][a-z0-9+.-]*:\/\/[^/?#]+)/i.exec(repo.url) ?? ["", ""])[1].toLowerCase() : ""
    readonly property string host: site.replace(/^[a-z][a-z0-9+.-]*:\/\//i, "")
    readonly property string name: repo ? repo.name || host : ""
    readonly property var account: repo ? accounts.find(a => !a.pending && a.repos.some(l => same(l.url, repo.url))) ?? null : null
    readonly property var known: repo ? accounts.filter(a => !a.pending && a.server.replace(/\/+$/, "").toLowerCase() === site) : []
    readonly property bool asking: repo !== null && account === null

    function same(one, other) {
        return String(one).replace(/\/+$/, "") === String(other).replace(/\/+$/, "");
    }

    function load() {
        App.query("List", [], "Repos1", reply => targets && targets.have(reply.repos, targets.accounts), () => targets && targets.have([], targets.accounts));
        App.query("List", [], "Accounts1", reply => targets && targets.have(targets.repos, reply.accounts), () => targets && targets.have(targets.repos, []));
    }

    function have(listed, signed) {
        repos = listed;
        accounts = signed;
        if (repo === null) {
            const wanted = candidates.find(r => r.url === publish.memory.target) ?? (candidates.length === 1 ? candidates[0] : null);
            chosen = wanted ? wanted.url : "";
        }
        settle();
    }

    function choose(url) {
        chosen = url;
        publish.memory.target = url;
        stage = "";
        settle();
    }

    function settle() {
        publish.account = repo !== null && account !== null ? {
                "state": "ready",
                "id": account.id,
                "user": account.user,
                "name": name,
                "url": site,
                "repo": repo.url
            } : null;
    }

    function use(id) {
        App.act("Use", [repo.url, id], "Accounts1", () => targets && targets.load(), error => targets && (targets.detail = error.message));
    }

    function signIn() {
        App.query("SignIn", [repo.url], "Accounts1", result => targets && targets.approving(result), error => targets && (targets.detail = error.message));
    }

    function approving(result) {
        code = result[0];
        link = result[1];
        job = result[2];
        stage = "approve";
    }

    function ended(id, state, message) {
        if (id !== job)
            return;
        job = -1;
        stage = "";
        detail = state === "done" ? "" : message;
        load();
    }

    onAccountChanged: settle()
    onRepoChanged: settle()
}
