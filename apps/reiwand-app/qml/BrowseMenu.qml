import QtQuick
import io.github.reiwand.app

Item {
    id: menu

    required property var explore
    required property var acts
    readonly property alias list: list
    readonly property alias copy: copy
    readonly property var picks: !explore.pkg ? [] : [
        {
            "kind": "main",
            "text": acts.main.text,
            "icon": acts.main.icon.name,
            "off": !acts.main.visible
        },
        {
            "kind": "details",
            "text": App.tr("Details"),
            "icon": "caret-right",
            "off": explore.full
        },
        {
            "kind": "copy",
            "text": App.tr("Edit a copy…"),
            "icon": "edit-copy",
            "off": !copy.pkg || !String(copy.item.path ?? "").endsWith(".wpak")
        }
    ]

    function ask(at) {
        if (!explore.pkg)
            return;
        list.opener = at;
        list.current = 0;
        list.open();
    }

    BrowseCopy {
        id: copy

        wallpaper: menu.explore.pkg
        catalog: menu.explore.catalog
    }

    ItemCopy {
        id: copying
    }

    OptionMenu {
        id: list

        title: menu.explore.pkg ? menu.explore.pkg.title : ""
        options: menu.picks
        onChosen: index => {
            const kind = menu.picks[index].kind;
            if (kind === "main")
                menu.acts.act();
            else if (kind === "details")
                menu.explore.open = true;
            else if (kind === "copy")
                copying.start(String(menu.copy.item.path));
        }
    }
}
