import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var page

    function expect(ok, what) {
        if (!ok)
            throw new Error("account page: " + what);
    }

    function exercise() {
        page.loaded = true;
        page.accounts = [];
        expect(page.empty.visible && page.empty.button.text === App.tr("Add a repository"), "without accounts it explains and offers to add a repository");
        expect(page.list.count === 0, "no rows without accounts");
        expect(page.move.kind === "pliega" && page.move.text === App.tr("Move your configuration"), "it points to moving the configuration");
        page.accounts = [{
                "id": "a1",
                "server": "https://repo.example.org",
                "user": "reader",
                "label": "reader@repo.example.org",
                "pending": false,
                "repos": [{
                        "url": "https://repo.example.org/r/example/",
                        "repo_id": "3f"
                    }]
            }, {
                "id": "a2",
                "server": "https://casa.example",
                "user": "maker",
                "label": "maker@casa.example",
                "pending": true,
                "repos": []
            }];
        expect(!page.empty.visible && page.list.count === 2, "each account gets its row");
        const first = page.list.itemAt(0);
        const second = page.list.itemAt(1);
        first.row.open = true;
        second.row.open = true;
        expect(first.row.text === App.tr("%1 at %2", "reader", "repo.example.org") && first.row.note === "repo.example.org/r/example", "a row names the user, the server and its repositories");
        expect(first.signOut.visible && !first.signIn.visible && first.remove.visible, "a signed-in account offers to sign out and to remove");
        expect(second.signIn.visible && !second.signOut.visible && second.row.note === App.tr("Not linked to a repository"), "a signed-out account offers to sign in again");
        second.code = "ABCD-EFGH";
        second.link = "https://casa.example/link";
        expect(second.signIn.text === App.tr("Open in the browser"), "after asking, the code is shown and the browser is offered");
        const repo = first.repoList.itemAt(0).item;
        expect(repo !== null && repo.address === "https://repo.example.org/r/example/", "a signed-in account shows its repository");
        expect(second.repoList.count === 0 && second.row.open, "a signed-out account opens by itself and shows no repository");
        repo.uploads.versions = [{
                "package": "org.example.pond",
                "title": "Pond",
                "version": "1.0.0",
                "state": "published",
                "visibility": "public"
            }, {
                "package": "org.example.lake",
                "version": "0.2.0",
                "state": "pending_review",
                "visibility": "unlisted"
            }];
        repo.uploads.sessions = [{
                "id": "9d9d",
                "state": "open"
            }, {
                "id": "8e8e",
                "state": "failed"
            }];
        expect(repo.uploads.list.count === 2 && repo.uploads.list.itemAt(1).current === 1 && repo.uploads.list.itemAt(0).options.length === 2, "each upload shows its visibility and lets it change");
        expect(repo.uploads.cancels.count === 1 && repo.uploads.cancels.itemAt(0).note === "9d9d", "only the unfinished upload can be cancelled");
        repo.keys.items = [{
                "fingerprint": "5a5a"
            }];
        expect(repo.keys.list.count === 1 && repo.keys.add.visible, "the author keys are listed and a new one can be registered");
        expect(!repo.again.visible, "a working account offers no new sign-in");
        repo.uploads.failed({
            "name": "io.github.reiwand.Error.SignedOut",
            "message": "sign in again"
        });
        expect(repo.lost && repo.again.visible && repo.again.button.text === App.tr("Sign in again"), "a refused account offers to sign in again right there");
        first.remove.clicked();
        expect(first.remover.opened || first.remover.visible, "removing asks first");
        first.remover.close();
        expect(page.keyHints.length === 3 && page.keyHints[1].keys[0] === "Intro", "the key bar says what the keys do");
    }
}
