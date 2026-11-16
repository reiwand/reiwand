import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var page

    function must(ok, text) {
        if (!ok)
            throw new Error(text);
    }

    function made(kind, broken) {
        return {
            "target": "",
            "covers": check.page.covers,
            "entry": {
                "uid": "01J9Z0X9Z6Q7X4F3K5E1V0R1MS",
                "title": "Shown",
                "kind": kind,
                "missing": false,
                "broken": broken
            }
        };
    }

    function offered(kind, broken) {
        const fake = made(kind, broken);
        const card = Qt.createComponent("LibraryCard.qml").createObject(page, {
            "page": fake
        });
        const menu = Qt.createComponent("LibraryMenu.qml").createObject(page, {
            "page": fake
        });
        menu.ask(card);
        const copy = menu.picks.find(p => p.kind === "copy");
        const found = [card.copy.visible, menu.list.opened, copy.text, copy.off === true];
        menu.list.close();
        card.destroy();
        menu.destroy();
        return found.join("|");
    }

    function retried() {
        const menu = Qt.createComponent("LibraryMenu.qml").createObject(page, {
            "page": made("image", "")
        });
        menu.entry = made("image", "").entry;
        const plain = menu.picks.some(p => p.kind === "retry");
        menu.layering = {
            "state": "failed",
            "file": "/pictures/hill.png"
        };
        const retry = menu.picks.find(p => p.kind === "retry");
        menu.destroy();
        must(!plain, "a picture whose layers did not fail offers to try them again");
        must(retry && retry.text === App.tr("Try again"), "a picture whose layers failed does not offer to try again");
    }

    function exercise() {
        retried();
        const scene = offered("scene", "");
        must(scene === ["true", "true", App.tr("Edit a copy…"), "false"].join("|"), "a scene of the library does not offer to edit a copy on its card and its menu: " + scene);
        const picture = offered("image", "");
        must(picture === ["false", "true", App.tr("Edit a copy…"), "true"].join("|"), "a picture of the library offers to edit a copy: " + picture);
        const hurt = offered("scene", "cut short");
        must(hurt.startsWith("false|true|") && hurt.endsWith("|true"), "a damaged scene offers to edit a copy: " + hurt);
        must(!page.covers.menu.picks.some(p => p.kind === "edit"), "the menu of the library offers to open the original");
    }
}
