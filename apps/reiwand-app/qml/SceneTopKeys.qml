import QtQuick
import io.github.reiwand.app

// The keys of the editor's top bar: Ctrl+Z, Ctrl+Shift+Z and Ctrl+S. They work in every window of
// the editor, the windows panels are detached into as well.
Item {
    required property var bar

    Shortcut {
        sequence: "Ctrl+Z"
        context: Qt.ApplicationShortcut
        enabled: (bar.history.done ?? []).length > 0
        onActivated: bar.page.run("undo")
    }

    Shortcut {
        sequence: "Ctrl+Shift+Z"
        context: Qt.ApplicationShortcut
        enabled: (bar.history.undone ?? []).length > 0
        onActivated: bar.page.run("redo")
    }

    Shortcut {
        sequence: "Ctrl+S"
        context: Qt.ApplicationShortcut
        enabled: bar.dirty && !bar.page.editor.busy
        onActivated: bar.page.editor.save()
    }
}
