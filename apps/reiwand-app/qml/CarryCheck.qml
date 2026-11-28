import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var page
    readonly property var denied: "org.freedesktop.DBus.Error.AccessDenied"
    readonly property var invalid: "org.freedesktop.DBus.Error.InvalidArgs"
    readonly property var preview: ({
            "file": {
                "program": "reiwand",
                "version": "1.0.0",
                "created": "2026-11-16",
                "encrypted": false,
                "sections": ["repositories", "lists", "favourites", "settings"]
            },
            "rows": [
                {
                    "section": "repositories",
                    "held": 2,
                    "new": 1,
                    "skipped": 1,
                    "conflicts": 0
                },
                {
                    "section": "lists",
                    "held": 2,
                    "new": 1,
                    "skipped": 0,
                    "conflicts": 1
                },
                {
                    "section": "favourites",
                    "held": 3,
                    "new": 3,
                    "skipped": 0,
                    "conflicts": 0
                },
                {
                    "section": "settings",
                    "held": 4,
                    "new": 4,
                    "skipped": 0,
                    "conflicts": 0
                }
            ],
            "lists": ["Sea", "Night"],
            "notes": []
        })

    function expect(ok, what) {
        if (!ok)
            throw new Error("configuration dialogs: " + what);
    }

    function press(key) {
        App.backend.press(key, 0);
    }

    function row(place) {
        page.query = "";
        page.tab = 2;
        page.open(place);
        return page.shown.item;
    }

    function exporting(part) {
        part.exporting();
        const out = part.exporter;
        expect(out && out.visible, "the export dialog must open");
        out.ownBytes = 5242880;
        expect(out.own.enabled && !out.secrets.checked && !out.password.visible, "own wallpapers can be ticked and the password is asked only with the secrets");
        expect(CarryWords.size(5242880) === App.tr("%1 MB", 5), "the size of the own wallpapers must be told");
        out.secrets.checked = true;
        expect(out.password.visible && out.again.visible, "ticking the secrets must ask the password twice");
        out.password.text = "correct horse";
        out.again.text = "correct hors";
        expect(!out.matching, "different passwords must not match");
        out.again.text = "correct horse";
        expect(out.matching, "equal passwords must match");
        out.secrets.checked = false;
        expect(!out.password.visible, "unticking the secrets must hide the password");
        out.close();
        out.ownBytes = 0;
        out.offer();
        expect(!out.own.enabled && !out.own.checked && !out.secrets.checked, "the dialog must open with nothing ticked");
        out.close();
    }

    function wrong(into) {
        into.refused(invalid, "the file is encrypted: give its password");
        expect(into.visible && into.stage === "password" && !into.wrong, "an encrypted file must ask for its password");
        into.password.text = "nope";
        into.refused(denied, "wrong password, or the file is damaged");
        expect(into.stage === "password" && into.wrong, "a wrong password must be said");
        into.previewed(preview, "correct horse");
        expect(into.stage === "choose" && !into.wrong && into.secret === "correct horse", "the right password must lead to the checklist");
    }

    function choosing(into) {
        expect(into.choose.rows.count === 4, "every section of the file must be listed");
        expect(into.choose.taken.join() === "repositories,lists,favourites" && !into.choose.rows.get(3).checked, "everything is ticked except the settings");
        expect(into.choose.rows.get(0).detail === App.tr("%1 new", 1) + ", " + App.tr("%1 already here", 1), "the counts must be told");
        expect(into.choose.rows.get(1).detail === App.tr("%1 new", 1) + ", " + App.tr("%1 with the same name", 1), "a name conflict must be told");
        expect(into.choose.conflicting && into.choose.join.visible && into.choose.mode === "join", "lists with the same name ask join or both, join first");
        into.choose.list.forceActiveFocus(Qt.TabFocusReason);
        press(Qt.Key_Down);
        expect(into.choose.list.currentIndex === 1, "the arrows must move between sections");
        press(Qt.Key_Space);
        expect(!into.choose.rows.get(1).checked && !into.choose.conflicting, "Space must untick the section and drop the question about lists");
        press(Qt.Key_Space);
        into.choose.both.click();
        expect(into.choose.mode === "both", "keeping both must be chosen");
        into.choose.list.forceActiveFocus(Qt.TabFocusReason);
        press(Qt.Key_Down);
        press(Qt.Key_Down);
        press(Qt.Key_Space);
        expect(into.choose.taken.join() === "repositories,lists,favourites,settings", "the settings can be ticked");
        press(Qt.Key_Space);
        expect(into.bring.enabled, "something ticked must allow importing");
        for (let i = 0; i < 4; i++)
            into.choose.rows.setProperty(i, "checked", false);
        expect(!into.bring.enabled, "nothing ticked must not allow importing");
        into.submit();
        expect(into.stage === "choose", "importing nothing must do nothing");
        into.choose.rows.setProperty(0, "checked", true);
        into.choose.list.forceActiveFocus(Qt.TabFocusReason);
        press(Qt.Key_Return);
        expect(into.stage === "working", "Enter must import what is ticked");
    }

    function finishing(into) {
        into.preview = Object.assign({}, preview, {
            "notes": ["account sessions are not imported yet"]
        });
        into.finished({
            "added": {
                "repositories": 1,
                "favourites": 3
            },
            "pending": [
                {
                    "section": "repositories",
                    "name": "https://home.example/wallpapers",
                    "reason": "the certificate of home.example is not from a known authority; its fingerprint is ab12"
                },
                {
                    "section": "lists",
                    "name": "Sea",
                    "reason": "the list is gone"
                }
            ],
            "unresolved": 2
        });
        expect(into.stage === "done" && into.result.sections.length === 2 && into.result.signIn && into.finish.visible && into.quiet === "", "the result closes with one button and must list what was added and what is pending");
        expect(CarryWords.pending({
                "section": "repositories",
                "reason": "the certificate is unknown"
            }).label === App.tr("Confirm the certificate"), "a certificate to confirm must lead to the repositories");
        expect(CarryWords.pending({
                "section": "lists",
                "reason": "gone"
            }) === null, "a pending list has nowhere to lead");
        into.close();
    }

    function waiting() {
        const jobs = Qt.createComponent("CarryJobs.qml").createObject(check);
        const ended = [];
        jobs.ended.connect((id, state, message) => ended.push(id + state));
        jobs.started(7);
        jobs.finish(8, "done", "");
        expect(ended.length === 0, "another job must not end this one");
        jobs.finish(7, "done", "");
        expect(ended.join() === "7done" && jobs.job === -1, "the job must end once");
        jobs.seen = {
            "9": {
                "state": "failed",
                "message": "no room"
            }
        };
        jobs.started(9);
        expect(ended.join() === "7done,9failed", "a job that ended before its id arrived must still end");
    }

    function exercise() {
        const part = row("carry");
        expect(JSON.stringify(page.route) === JSON.stringify([App.tr("Device"), App.tr("Move your configuration")]), "the route of the page");
        expect(SettingsReach.rows(part).map(row => row.text).join() === [App.tr("Export my configuration…"), App.tr("Import a configuration…")].join(), "the rows of the page");
        exporting(part);
        part.importing();
        const into = part.importer;
        expect(into.reading.title === App.tr("Import a configuration"), "the import starts with the file chooser");
        wrong(into);
        choosing(into);
        finishing(into);
        waiting();
    }
}
