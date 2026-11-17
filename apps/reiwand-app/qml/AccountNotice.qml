import QtQuick
import io.github.reiwand.app

Aviso {
    id: notice

    required property var host
    property string phase
    property bool dismissed: false

    function load() {
        notice.phase = "";
    }

    text: App.tr("This computer can no longer sync: the server no longer accepts it, because it was removed from the account or the account was deleted.")
    tone: "coral"
    action: App.tr("Open Account")
    onPhaseChanged: dismissed = false
    onActed: notice.host.activate("account")
    onClosed: dismissed = true
    Component.onCompleted: load()

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "account")
                notice.load();
        }
    }
}
