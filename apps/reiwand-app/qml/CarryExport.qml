import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

SmallPanel {
    id: dialog

    readonly property alias own: own
    readonly property alias secrets: secrets
    readonly property alias password: password
    readonly property alias again: again
    readonly property alias writing: writing
    property real ownBytes: 0
    property string target
    readonly property bool matching: password.text !== "" && password.text === again.text

    function offer() {
        ownBytes = App.backend.ownSize();
        own.checked = false;
        secrets.checked = false;
        password.text = "";
        again.text = "";
        open();
    }

    function start(path) {
        target = path;
        jobs.reset();
        const options = JSON.stringify({
            "own": own.checked,
            "secrets": secrets.checked
        });
        App.carry("Export", path, secrets.checked ? password.text : "", options, reply => dialog && jobs.started(reply[0]), error => dialog && dialog.refused(error.name, error.message));
        password.text = "";
        again.text = "";
    }

    function refused(name, message) {
        const exists = String(name).endsWith("Exists");
        applicationWindow().showPassiveNotification(exists ? App.tr("That file already exists: choose another name.") : App.tr("The configuration was not saved: %1", message), "long");
    }

    title: App.tr("Export my configuration")
    text: App.tr("One file with your repositories, photo sources, lists, favorites, wallpapers installed from repositories and the settings that make sense on another computer.")
    quiet: App.tr("Cancel")
    first: own
    width: Look.smallPanel + 2 * Look.e6

    foot: [
        QQC2.Button {
            id: choose

            enabled: !secrets.checked || dialog.matching
            highlighted: true
            text: App.tr("Choose where to save…")
            icon.name: "document-save"
            onClicked: {
                dialog.close();
                writing.open();
            }
            Keys.onReturnPressed: clicked()
            Keys.onEnterPressed: clicked()
        }
    ]

    QQC2.CheckBox {
        id: own

        enabled: dialog.ownBytes > 0
        text: App.tr("Include my own wallpapers")
        Keys.onDownPressed: nextItemInFocusChain(true).forceActiveFocus(Qt.TabFocusReason)
    }

    Text {
        x: parent.padding + Look.e6
        width: parent.width - 2 * parent.padding - Look.e6
        text: dialog.ownBytes > 0 ? App.tr("They add %1 to the file.", CarryWords.size(dialog.ownBytes)) : App.tr("You have none of your own: nothing would be added.")
        wrapMode: Text.WordWrap
        font.family: Look.textFont
        font.pixelSize: Look.tFoot
        color: Look.mist
    }

    QQC2.CheckBox {
        id: secrets

        text: App.tr("Include passwords and keys")
        Keys.onUpPressed: nextItemInFocusChain(false).forceActiveFocus(Qt.TabFocusReason)
        Keys.onDownPressed: nextItemInFocusChain(true).forceActiveFocus(Qt.TabFocusReason)
    }

    Column {
        visible: secrets.checked
        width: parent.width - 2 * parent.padding
        spacing: Look.e2

        Text {
            width: parent.width
            text: App.tr("The whole file is locked with a password you choose now. You will need it to import the file; it cannot be recovered.")
            wrapMode: Text.WordWrap
            font.family: Look.textFont
            font.pixelSize: Look.tFoot
            color: Look.mist
        }

        QQC2.TextField {
            id: password

            width: parent.width
            echoMode: TextInput.Password
            placeholderText: App.tr("Password for the file")
            onAccepted: again.forceActiveFocus(Qt.TabFocusReason)
        }

        QQC2.TextField {
            id: again

            width: parent.width
            echoMode: TextInput.Password
            placeholderText: App.tr("Write it again")
            onAccepted: dialog.matching && choose.clicked()
        }

        Text {
            visible: again.text !== "" && !dialog.matching
            width: parent.width
            text: App.tr("The two passwords are not the same.")
            font.family: Look.textFont
            font.pixelSize: Look.tFoot
            color: Look.coral
        }
    }

    Chooser {
        id: writing

        mode: Chooser.Save
        title: App.tr("Save my configuration")
        defaultSuffix: "reiwand"
        nameFilters: [App.tr("Reiwand configuration") + " (*.reiwand)"]
        onAccepted: dialog.start(App.path(selectedFile))
    }

    CarryJobs {
        id: jobs

        onEnded: (id, state, message) => {
            if (state === "done")
                applicationWindow().showPassiveNotification(App.tr("Your configuration was saved in %1", dialog.target), "long");
            else
                dialog.refused("", message);
        }
    }
}
