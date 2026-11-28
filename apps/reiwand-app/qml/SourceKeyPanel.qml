import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

SmallPanel {
    id: panel

    property string provider
    property string error

    function ask(id) {
        provider = id;
        error = "";
        open();
    }

    title: App.tr("Change key")
    primary: App.tr("Save")
    primaryIcon: "key"
    quiet: App.tr("Cancel")
    onAccepted: App.setSecret(provider, entry.text, () => {
        if (panel) {
            entry.clear();
            panel.close();
        }
    }, e => panel && (panel.error = e.message))
    onRejected: {
        entry.clear();
        close();
    }

    QQC2.TextField {
        id: entry

        width: Look.smallPanel - 2 * Look.e4
        echoMode: TextInput.Password
        focus: true
    }

    Note {
        visible: panel.error !== ""
        tone: "coral"
        text: panel.error
    }
}
