import QtQuick
import io.github.reiwand.app

Item {
    id: routes

    required property var host

    function offer(page) {
        const at = page.startsWith("add:") ? 4 : page.startsWith("open:") ? 0 : -1;
        if (at < 0)
            return false;
        host.reach(at);
        const here = () => host.pageStack.currentItem;
        Qt.callLater(() => at === 4 ? here()?.bar?.adder?.offer(page.slice(4)) : here()?.tools?.dialogs?.importer?.receive(JSON.parse(page.slice(5))));
        return true;
    }

    Connections {
        target: App

        function onNavigate(page, query) {
            routes.host.showPage(page, {
                    "query": query
                });
        }

        function onEdit(item) {
            routes.host.work(editor, {
                    "item": item
                });
        }

        function onCompose(folder, starter, text) {
            const kind = starter ? (["web", "preset", "image", "video"].includes(starter) ? starter : "scene") : probe.kindOf(folder, text);
            routes.host.work(["scene", "assets"].includes(kind) ? composer : packager, {
                    "folder": folder,
                    "starter": starter,
                    "heading": starter ? text : "",
                    "source": starter ? "" : text
                });
        }
    }

    Connections {
        target: App.backend

        function onActivated(page) {
            routes.host.activate(page);
        }

        function onChanged(topic) {
            if (topic === "inputs" || topic === "state")
                routes.host.loadInputs();
        }
    }

    Component {
        id: editor

        EditPage {
        }
    }

    Component {
        id: composer

        ScenePage {
        }
    }

    Component {
        id: packager

        PackagePage {
        }
    }

    SceneEditor {
        id: probe
    }
}
