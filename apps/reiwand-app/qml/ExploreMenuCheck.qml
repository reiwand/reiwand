import QtQuick
import io.github.reiwand.app

QtObject {
    required property var page

    function exercise() {
        if (!page.pkg)
            return;
        const menu = page.menu;
        menu.ask(page.shelf);
        const found = [menu.list.opened, menu.picks.map(p => p.kind).join()];
        menu.list.close();
        if (found.join("|") !== "true|main,details,copy")
            throw new Error("the menu of a card in Explore does not offer its actions and Edit a copy: " + found.join("|"));
    }
}
