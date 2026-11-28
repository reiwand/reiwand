import QtQuick
import QtCore as Core
import io.github.reiwand.app

// The tones of the editor: taken from the thumbnail of the scene being edited (the app's own set
// when it has none), dark or, as an editor preference, light. Leaving the editor gives back the
// tones the app had.
QtObject {
    id: tones

    required property var page
    readonly property bool on: page.visible
    readonly property bool light: on && saved.light
    readonly property string thumb: {
        tones.page.view;
        const manifest = JSON.parse(page.editor.value("/manifest") || "{}") ?? {};
        const found = Publishing.thumbnail(manifest);
        return found ? page.editor.folder + "/" + found : "";
    }
    property var kept: null
    readonly property Core.Settings saved: EditorLight.saved

    function toggle() {
        EditorLight.toggle();
    }

    function apply() {
        if (!on) {
            Look.light = false;
            if (kept)
                Look.tone(kept);
            kept = null;
            return;
        }
        if (!kept)
            kept = {
                "night": String(Look.night),
                "nightHigh": String(Look.nightHigh),
                "nightLine": String(Look.nightLine),
                "mist": String(Look.mist),
                "mistLow": String(Look.mistLow),
                "moon": String(Look.moon),
                "moonDeep": String(Look.moonDeep),
                "veil": Look.veil
            };
        Look.tone(JSON.parse(light ? Look.bridge.deriveLight(thumb) : Look.bridge.derive(thumb)));
        Look.light = light;
    }

    onOnChanged: apply()
    onLightChanged: apply()
    onThumbChanged: on && apply()
}
