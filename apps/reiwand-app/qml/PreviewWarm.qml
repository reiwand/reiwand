import QtQuick
import io.github.reiwand.app

QtObject {
    id: warm

    required property var preview
    property bool resting: false

    function rest() {
        resting = true;
        preview.visible = false;
        preview.session = ({});
        preview.began = false;
        preview.failure = "";
        preview.moved = false;
        preview.closing = false;
        preview.heldFocus = false;
        preview.retargeted = false;
        preview.wheel = 0;
        preview.left = -1;
        preview.playUid = "";
        preview.animated = "";
    }

    function cover() {
        const found = Qt.application.screens.find(s => s.name === preview.output);
        if (found)
            preview.screen = found;
        preview.visible = true;
        if (found)
            preview.screen = found;
        preview.visibility = Window.FullScreen;
    }

    function show() {
        if (preview.visible)
            return;
        cover();
        preview.requestActivate();
        preview.opening.start();
    }

    function wake(state) {
        resting = false;
        preview.take(state);
    }

    readonly property Timer expiry: Timer {
        interval: PreviewLook.tibio
        running: warm.resting && !App.checking
        onTriggered: Qt.exit(0)
    }
}
