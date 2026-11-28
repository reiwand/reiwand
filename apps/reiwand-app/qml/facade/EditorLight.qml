pragma Singleton

import QtQuick
import QtCore as Core

// The editor's own theme, dark or light: one preference for the whole app, so the editor's View
// menu and Settings show and change the same one.
QtObject {
    readonly property Core.Settings saved: Core.Settings {
        property bool light: false

        location: Core.StandardPaths.writableLocation(Core.StandardPaths.ConfigLocation) + "/reiwand/editor.conf"
        category: "look"
    }

    function toggle() {
        saved.light = !saved.light;
    }
}
