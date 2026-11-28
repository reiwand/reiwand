import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

RowLayout {
    id: head

    required property var input
    property Component editor: null
    readonly property var field: input.field
    readonly property real indent: input.Layout.leftMargin

    Layout.fillWidth: true
    Layout.minimumHeight: Look.fieldHeight
    spacing: Look.desk ? Look.e1 : Look.e2

    QQC2.Label {
        Layout.fillWidth: true
        Layout.maximumWidth: head.editor === null || !Look.desk ? Number.POSITIVE_INFINITY : DeskLook.label - head.indent
        // On the workbench the names of one-row values make a column of their own, so the
        // editors beside them line up, also below an indented group; a row too narrow for it
        // takes room from the name first. (A width read from the row would feed back into it.)
        Layout.minimumWidth: head.editor === null ? 0 : Math.min(implicitWidth, Look.e7)
        Layout.preferredWidth: head.editor === null ? -1 : Look.desk ? DeskLook.label - head.indent : Math.min(implicitWidth, Look.e8 * 2)
        text: (head.field.title || SceneWords.label(head.field.key, head.field.pointer)) + (head.field.unit ? " (" + head.field.unit + ")" : "")
        color: !enabled ? Look.mistLow : head.editor === null ? Look.frost : Look.mist
        font.bold: head.field.widget === "group" || head.field.widget === "variant"
        wrapMode: Text.Wrap
        QQC2.ToolTip.text: head.field.help ?? ""
        QQC2.ToolTip.visible: hint.hovered && (head.field.help ?? "") !== ""

        HoverHandler {
            id: hint
        }
    }

    Loader {
        id: edit

        // Above its neighbours (Default, ◇) while it has the keyboard, so its focus ring shows whole.
        z: {
            for (let at = Window.activeFocusItem; at; at = at.parent)
                if (at === edit)
                    return 1;
            return 0;
        }
        Layout.fillWidth: true
        // The editor takes what the name leaves, down to its own floor; only below that does a
        // narrow row take room from the name.
        Layout.preferredWidth: 0
        Layout.minimumWidth: item?.Layout.minimumWidth ?? 0
        active: head.editor !== null
        visible: active
        sourceComponent: head.editor
    }

    QQC2.ToolButton {
        visible: head.field.widget === "group" && !head.field.set
        icon.name: "list-add"
        text: App.tr("Add")
        onClicked: head.input.commit(head.field.sample)
    }

    QQC2.ToolButton {
        visible: head.field.set && !head.field.required && head.field.widget !== "computed"
        icon.name: "edit-reset"
        text: head.field.widget === "group" ? App.tr("Remove") : App.tr("Default")
        display: QQC2.AbstractButton.IconOnly
        QQC2.ToolTip.text: text
        QQC2.ToolTip.visible: hovered
        onClicked: head.input.page.run("remove", {
            "pointer": head.field.pointer
        })
    }

    SceneBind {
        visible: head.field.bindable
        page: head.input.page
        field: head.field
    }
}
