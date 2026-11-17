import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

PanelChico {
    id: dialog

    readonly property alias reading: reading
    readonly property alias password: password
    readonly property alias choose: choose
    readonly property alias result: result
    readonly property alias unlock: unlock
    readonly property alias bring: bring
    readonly property alias finish: finish
    property string stage: "password"
    property string path
    property string secret
    property bool wrong: false
    property var preview: ({})

    function begin() {
        reading.open();
    }

    function look(typed) {
        App.carry("PreviewImport", path, typed, "{}", reply => dialog && dialog.previewed(reply, typed), error => dialog && dialog.refused(error.name, error.message));
    }

    function previewed(reply, typed) {
        preview = reply;
        secret = typed;
        wrong = false;
        choose.fill(CarryWords.entries(reply.rows ?? []));
        choose.mode = "join";
        stage = "choose";
        visible ? choose.list.forceActiveFocus(Qt.TabFocusReason) : open();
    }

    function refused(name, message) {
        const reason = String(name).split(".").pop();
        if (reason === "InvalidArgs" && /password/i.test(message)) {
            wrong = false;
            stage = "password";
            visible ? password.forceActiveFocus(Qt.TabFocusReason) : open();
        } else if (reason === "AccessDenied" && stage === "password") {
            wrong = true;
            password.selectAll();
            password.forceActiveFocus(Qt.TabFocusReason);
        } else {
            applicationWindow().showPassiveNotification(App.tr("The configuration was not imported: %1", message), "long");
            if (stage === "working")
                stage = "choose";
        }
    }

    function submit() {
        if (stage !== "choose" || choose.taken.length === 0)
            return;
        const choices = JSON.stringify({
            "take": choose.taken,
            "lists": choose.mode,
            "suffix": " " + App.tr("(imported)")
        });
        stage = "working";
        jobs.reset();
        App.carry("ImportChosen", path, secret, choices, reply => dialog && dialog.stage === "working" && jobs.started(reply[0]), error => dialog && dialog.refused(error.name, error.message));
    }

    function finished(output) {
        result.added = output.added ?? {};
        result.pending = output.pending ?? [];
        result.unresolved = output.unresolved ?? 0;
        result.signIn = (preview.notes ?? []).length > 0;
        stage = "done";
        secret = "";
        visible ? finish.forceActiveFocus(Qt.TabFocusReason) : open();
    }

    function lead(page) {
        close();
        App.navigate(page, "");
    }

    title: App.tr("Import a configuration")
    text: stage === "choose" ? App.tr("Choose what to bring. What you already have is never replaced.") : stage === "password" ? App.tr("This file is locked with a password.") : stage === "working" ? App.tr("Importing… you can follow it in the list of tasks.") : ""
    quiet: stage === "done" ? "" : App.tr("Cancel")
    first: stage === "password" ? password : stage === "choose" ? choose.list : null
    width: Look.panelChico + 2 * Look.e6
    onClosed: {
        secret = "";
        password.text = "";
    }

    foot: [
        QQC2.Button {
            id: unlock

            visible: dialog.stage === "password"
            enabled: password.text !== ""
            highlighted: true
            text: App.tr("Open")
            icon.name: "document-open"
            onClicked: dialog.look(password.text)
            Keys.onReturnPressed: clicked()
            Keys.onEnterPressed: clicked()
        },
        QQC2.Button {
            id: bring

            visible: dialog.stage === "choose"
            enabled: choose.taken.length > 0
            highlighted: true
            text: App.tr("Import")
            icon.name: "document-import"
            onClicked: dialog.submit()
            Keys.onReturnPressed: clicked()
            Keys.onEnterPressed: clicked()
        },
        QQC2.Button {
            id: finish

            visible: dialog.stage === "done"
            highlighted: true
            text: App.tr("Close")
            onClicked: dialog.close()
            Keys.onReturnPressed: clicked()
            Keys.onEnterPressed: clicked()
        }
    ]

    QQC2.TextField {
        id: password

        visible: dialog.stage === "password"
        width: parent.width - 2 * parent.padding
        echoMode: TextInput.Password
        placeholderText: App.tr("Password of the file")
        onAccepted: unlock.enabled && unlock.clicked()
    }

    Text {
        visible: dialog.stage === "password" && dialog.wrong
        width: parent.width - 2 * parent.padding
        text: App.tr("That password does not open the file, or the file is damaged.")
        wrapMode: Text.WordWrap
        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
        color: Look.coral
    }

    CarryChoose {
        id: choose

        visible: dialog.stage === "choose"
        width: parent.width - 2 * parent.padding
        onAccepted: dialog.submit()
    }

    CarryResult {
        id: result

        visible: dialog.stage === "done"
        width: parent.width - 2 * parent.padding
        onLeads: page => dialog.lead(page)
    }

    Chooser {
        id: reading

        title: App.tr("Import a configuration")
        nameFilters: [App.tr("Reiwand configuration") + " (*.reiwand)", App.tr("All files") + " (*)"]
        onAccepted: {
            dialog.path = App.path(selectedFile);
            dialog.stage = "password";
            dialog.wrong = false;
            dialog.look("");
        }
    }

    CarryJobs {
        id: jobs

        onEnded: (id, state, message) => {
            if (state === "done")
                App.query("GetJob", [id], "Daemon1", reply => dialog && dialog.finished(reply.job.output ?? {}), error => dialog && dialog.finished({}));
            else
                dialog.refused("", message);
        }
    }
}
