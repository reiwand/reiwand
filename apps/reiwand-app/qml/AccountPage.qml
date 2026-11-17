import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Kirigami.ScrollablePage {
    id: page

    property var accounts: []
    property bool loaded: false
    property string error
    readonly property alias list: list
    readonly property alias empty: empty
    readonly property alias move: move
    readonly property var keyHints: [{
            "keys": ["up", "down"],
            "text": App.tr("Move")
        }, {
            "keys": ["Intro"],
            "text": App.tr("Open")
        }, {
            "keys": ["Esc"],
            "text": App.tr("Menu")
        }]

    function load() {
        App.query("List", [], "Accounts1", reply => {
            if (!page)
                return;
            page.accounts = reply.accounts;
            page.error = "";
            page.loaded = true;
        }, failure => page && (page.error = failure.message));
    }

    function repoNames(account) {
        return account.repos.map(r => r.url.replace(/^[a-z]+:\/\//i, "").replace(/\/+$/, "")).join(", ");
    }

    function exercise() {
        Qt.createComponent("AccountCheck.qml").createObject(page, {
            "page": page
        }).exercise();
    }

    title: App.tr("Account")
    header: PrivateLine {}
    Component.onCompleted: load()

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "repos" || topic === "accounts")
                page.load();
        }
    }

    Column {
        spacing: Look.e4

        Vacio {
            id: empty

            visible: page.loaded && page.accounts.length === 0
            title: App.tr("No accounts yet")
            text: App.tr("An account appears when a private repository asks for one or when you publish to a repository. Each account is linked to the repositories it is used for.")
            action: App.tr("Add a repository")
            actionIcon: "link-simple"
            onActed: applicationWindow().activate("add:")
        }

        Filas {
            visible: page.accounts.length > 0

            Seccion {
                text: App.tr("Your accounts")
            }

            Estado {
                visible: page.error !== ""
                text: page.error
                tone: "coral"
                icon: "warning"
            }

            Repeater {
                id: list

                model: page.accounts

                AccountRow {
                    required property var modelData

                    width: parent.width
                    account: modelData
                    repos: page.repoNames(modelData)
                }
            }
        }

        Filas {
            Fila {
                id: move

                text: App.tr("Move your configuration")
                kind: "pliega"
                note: App.tr("Settings › Device")
                explain: App.tr("Accounts are not synchronized. To take your repositories, lists and favorites to another computer, export them to a file in Settings and import it there.")
                onOpened: applicationWindow().activate("settings")
            }
        }
    }
}
