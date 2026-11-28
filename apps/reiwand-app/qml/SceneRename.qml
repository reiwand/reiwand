import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Kirigami.PromptDialog {
    id: dialog

    required property var page
    property string kind: "layer"
    property string current
    property var lines: []
    property alias text: field.text

    function ask(kind, id) {
        dialog.kind = kind;
        dialog.current = id;
        lines = JSON.parse(page.editor.scripts(id) || "[]");
        field.text = id;
        open();
        field.forceActiveFocus();
    }

    // What is typed or pasted becomes an id: lower case, without accents, spaces and dashes as _,
    // other signs left out, at most 32.
    function cleaned(text) {
        const plain = text.normalize("NFD").replace(/[\u0300-\u036f]/g, "").toLowerCase();
        return plain.replace(/[\s-]+/g, "_").replace(/[^a-z0-9_]/g, "").slice(0, 32);
    }

    function rename() {
        if (field.text === current)
            return close();
        if (field.text === "")
            return;
        page.run("rename", {
            "kind": kind,
            "id": current,
            "to": field.text
        });
        if (page.editor.error === "")
            close();
    }

    title: App.tr("Rename %1", current)
    standardButtons: QQC2.Dialog.Ok | QQC2.Dialog.Cancel
    onAccepted: rename()

    ColumnLayout {
        QQC2.TextField {
            id: field

            Layout.fillWidth: true
            onTextEdited: {
                const clean = dialog.cleaned(text);
                if (clean !== text) {
                    const at = cursorPosition - (text.length - clean.length);
                    text = clean;
                    cursorPosition = Math.max(0, Math.min(at, clean.length));
                }
            }
            onAccepted: dialog.rename()
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            color: Look.mist
            text: App.tr("Lower-case letters, digits and _, up to 32. Other letters and signs you type or paste are turned into these.")
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: App.tr("Bindings, parents, masks, effects, triggers and timelines that name it follow the new id.")
        }

        QQC2.Label {
            Layout.fillWidth: true
            visible: dialog.lines.length > 0
            wrapMode: Text.Wrap
            color: Look.amber
            text: App.tr("These lines of scripts name %1 and are not rewritten: %2", dialog.current, dialog.lines.join(", "))
        }
    }
}
