import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Kirigami.ApplicationWindow {
    id: root

    property int current: 0
    property var inputs: ({})
    property int from: 0
    property int waiting: 0
    property var held: []
    readonly property var names: ["library", "playlists", "profiles", "providers", "repos", "settings", "account", "scenes", "repositories"]
    readonly property alias shell: shell
    readonly property string kind: current < 0 ? "work" : pageStack.currentItem && pageStack.currentItem.skeleton ? pageStack.currentItem.skeleton : [2, 5, 6, 8].includes(current) ? "options" : "title"
    readonly property string backWord: (shell.menu.all.find(entry => entry[0] === shell.menu.owner(from)) ?? [0, ""])[1]
    readonly property var pages: [libraryPage, playlistsPage, layoutsPage, providersPage, reposPage, settingsPage, accountPage, createPage, repositoriesPage]

    function open(component, properties) {
        const old = pageStack.currentItem;
        pageStack.clear();
        old?.destroy();
        pageStack.push(component.createObject(pageStack, properties ?? {}));
    }

    function loadInputs() {
        App.query("GetInputs", [], "Inputs1", reply => root && (root.inputs = reply));
    }

    function work(component, properties) {
        if (current >= 0)
            from = current;
        current = -1;
        open(typeof component === "string" ? Qt.createComponent(component) : component, properties);
        size.roomy();
    }

    function leave() {
        showPage(from);
    }

    function showPassiveNotification(message, timeout, actionText, callBack) {
        shell.notices.show(message, timeout, actionText, callBack);
    }

    function hidePassiveNotification() {
        shell.notices.hide();
    }

    function showPage(index, properties) {
        current = index;
        open(pages[index], properties);
    }

    function favorited(count) {
        showPassiveNotification(count === 1 ? App.tr("Added to your favorites.") : App.tr("%1 wallpapers added to your favorites.", count), "long", App.tr("See favorites"), () => {
            showPage(0);
            pageStack.currentItem.bar.word = 6;
            pageStack.currentItem.search();
            Qt.callLater(() => shell && shell.arrive());
        });
    }

    function reach(index) {
        showPage(index);
        Qt.callLater(() => shell && shell.arrive());
    }

    function activate(page) {
        const index = names.indexOf(page);
        if (index >= 0 && index !== current && pageStack.currentItem?.editor?.dirty !== true)
            reach(index);
        if (visibility === Window.Minimized)
            showNormal();
        raise();
        requestActivate();
    }

    title: "Reiwand"
    minimumWidth: Kirigami.Units.gridUnit * 62
    minimumHeight: Kirigami.Units.gridUnit * 38
    pageStack.globalToolBar.style: Kirigami.ApplicationHeaderStyle.None
    Component.onCompleted: {
        size.restore();
        reach(Math.max(0, names.indexOf(App.backend.requested())));
        loadInputs();
    }
    Frame {
        id: shell

        host: root
        focus: true
    }

    WindowSize {
        id: size

        host: root
    }

    PermisosAsk {
        host: root
    }

    StillAsk {
    }

    LayoutOffer {
        host: root
    }

    MainRoutes {
        host: root
    }

    Tour {
        host: root
    }

    LookWindow {
        host: root
        tight: shell.kind !== "work" && shell.titleFrame.tight
    }

    Component {
        id: libraryPage

        LibraryPage {
        }
    }

    Component {
        id: playlistsPage

        PlaylistsPage {
        }
    }

    Component {
        id: layoutsPage

        SettingsPage {
            place: "layouts"
        }
    }

    Component {
        id: providersPage

        ProvidersPage {
        }
    }

    Component {
        id: reposPage

        ReposPage {
        }
    }

    Component {
        id: settingsPage

        SettingsPage {
        }
    }

    Component {
        id: accountPage

        AccountPage {
        }
    }

    Component {
        id: createPage

        CreatePage {
        }
    }

    Component {
        id: repositoriesPage

        RepositoriesPage {
        }
    }
}
