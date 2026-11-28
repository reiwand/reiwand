import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

ColumnLayout {
    id: column

    required property var host
    readonly property string shown: String(host.value ?? "")
    readonly property alias area: area

    // A run of this many characters with no space or line break is wrapped anywhere: wrapping it at
    // word boundaries, which it has none of, takes Qt seconds (half a minute for a pasted megabyte)
    // and freezes the window. Text with words keeps wrapping between them.
    function unbroken(text) {
        return /\S{2000}/.test(text);
    }

    function fit(text) {
        area.wrapMode = unbroken(text) ? TextEdit.WrapAnywhere : TextEdit.Wrap;
        return text;
    }

    // Paste reads the clipboard first, so the wrap is chosen before the text is laid out.
    function paste() {
        probe.clear();
        probe.paste();
        const pasted = probe.text;
        probe.clear();
        const kept = area.text.slice(0, area.selectionStart) + area.text.slice(area.selectionEnd);
        fit(unbroken(kept) ? kept : pasted);
        area.remove(area.selectionStart, area.selectionEnd);
        area.insert(area.cursorPosition, pasted);
    }

    Layout.fillWidth: true

    TextEdit {
        id: probe

        visible: false
        textFormat: TextEdit.PlainText
    }

    Area {
        id: area

        Layout.fillWidth: true
        enterApplies: true
        QQC2.ToolTip.visible: hovered && activeFocus
        QQC2.ToolTip.text: App.tr("Enter applies the text; Shift+Enter starts a new line")
        text: column.fit(column.shown)
        wrapMode: TextEdit.Wrap
        Keys.onShortcutOverride: event => event.accepted = event.matches(StandardKey.Paste)
        Keys.onPressed: event => {
            if (!event.matches(StandardKey.Paste))
                return;
            event.accepted = true;
            column.paste();
        }
        selectByMouse: true
        Accessible.name: SceneWords.label(column.host.field.key)
        onEditingFinished: {
            if (text !== column.shown)
                column.host.commit(text);
        }
    }

    SceneTranslate {
        visible: column.host.field.translatable
        host: column.host
    }
}
