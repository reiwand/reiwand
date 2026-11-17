import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Column {
    id: again

    required property string repo
    property string code
    property string link
    readonly property alias button: button

    function address(accounts) {
        const all = accounts.flatMap(a => a.repos);
        const found = all.find(r => r.repo_id === again.repo || r.url === again.repo);
        return found ? found.url : again.repo;
    }

    function ask() {
        App.query("List", [], "Accounts1", reply => {
            if (!again)
                return;
            App.query("SignIn", [again.address(reply.accounts)], "Accounts1", result => {
                if (!again)
                    return;
                again.code = result[0];
                again.link = result[1];
            }, failure => again && applicationWindow().showPassiveNotification(failure.message, "long"));
        }, failure => again && applicationWindow().showPassiveNotification(failure.message, "long"));
    }

    spacing: Look.e2

    Text {
        visible: again.code !== ""
        text: App.tr("Open %1 and enter the code %2", again.link, again.code)
        wrapMode: Text.Wrap
        width: Math.min(parent.width, Look.vacioAncho)
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.escarcha
    }

    QQC2.Button {
        id: button

        highlighted: true
        text: again.code !== "" ? App.tr("Open in the browser") : App.tr("Sign in again")
        icon.name: "sign-in"
        onClicked: again.code !== "" ? Qt.openUrlExternally(again.link) : again.ask()
    }
}
