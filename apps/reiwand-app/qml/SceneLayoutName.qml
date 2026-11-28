import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

// Names a layout of the author: a new one made from the workbench as it is ("Save as new
// layout…"), or one already saved ("Rename…"). The name is refused, with its reason, when it is
// empty, starts or ends with a space or is taken by another of the author's layouts.
Kirigami.PromptDialog {
    id: dialog

    required property var layout
    property int index: -1
    readonly property string fault: layout.desks.nameFault(layout.desk, field.text, index)

    function ask(at) {
        index = at;
        field.text = at >= 0 ? layout.parsed.own[at].name : "";
        open();
        field.forceActiveFocus();
    }

    function done() {
        if (fault !== "")
            return;
        if (index >= 0 ? layout.rename(index, field.text) : layout.saveAs(field.text))
            close();
    }

    title: index >= 0 ? App.tr("Rename the layout %1", layout.parsed.own[index]?.name ?? "") : App.tr("Save as new layout")
    standardButtons: QQC2.Dialog.Cancel
    customFooterActions: [
        Kirigami.Action {
            text: dialog.index >= 0 ? App.tr("Rename") : App.tr("Save")
            icon.name: "document-save"
            enabled: dialog.fault === ""
            onTriggered: dialog.done()
        }
    ]

    ColumnLayout {
        QQC2.TextField {
            id: field

            Layout.fillWidth: true
            Accessible.name: App.tr("Name of the layout")
            placeholderText: App.tr("Name of the layout")
            onAccepted: dialog.done()
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            color: dialog.fault !== "" && field.text !== "" ? Look.amber : Look.mist
            text: dialog.fault !== "" && field.text !== "" ? dialog.fault : dialog.index >= 0 ? App.tr("Its panels stay where they are.") : App.tr("It keeps the panels where they are now; Ctrl+7, Ctrl+8 and Ctrl+9 choose your first three layouts.")
        }
    }
}
