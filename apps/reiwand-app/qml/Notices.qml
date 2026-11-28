import QtQuick
import io.github.reiwand.app

Column {
    id: notices

    required property var host
    property string text: ""
    property string tone: ""
    property string action: ""
    property var callback: null
    property string dismissed: ""
    property bool told: false
    readonly property string banner: App.backend.banner
    readonly property bool failing: banner !== "" && banner !== dismissed && !told
    readonly property string refusal: App.backend.lastError
    property string waved: ""
    readonly property bool refusing: refusal !== "" && refusal !== banner && refusal !== waved && !told
    readonly property bool busy: failing
    readonly property alias passive: passive
    readonly property alias refused: refused

    function show(message, timeout, actionText, onAction) {
        const notice = message !== null && typeof message === "object" ? message : {
            "text": message
        };
        text = notice.text ?? "";
        tone = notice.tone ?? "";
        action = actionText ?? notice.action ?? "";
        callback = onAction ?? notice.undo ?? null;
        later.interval = typeof timeout === "number" && timeout > 0 ? timeout : 7000;
        later.restart();
    }

    function hide() {
        later.stop();
        text = "";
        callback = null;
    }

    width: Look.toastWidth
    spacing: Look.e2
    Accessible.role: Accessible.StaticText
    onBannerChanged: dismissed = banner === "" ? "" : dismissed
    onRefusalChanged: waved = refusal === "" ? "" : waved

    Timer {
        id: later

        onTriggered: notices.hide()
    }

    Toast {
        visible: notices.failing
        anchors.right: parent.right
        width: Math.min(notices.width, implicitWidth)
        text: notices.banner.split("\n")[0]
        tone: "coral"
        onClosed: notices.dismissed = notices.banner
    }

    Toast {
        id: refused

        visible: notices.refusing
        anchors.right: parent.right
        width: Math.min(notices.width, implicitWidth)
        // the first line only: a failed plugin sync carries its stack, whole under the source's Details
        text: notices.refusal.split("\n")[0]
        tone: "coral"
        onClosed: notices.waved = notices.refusal
    }

    Toast {
        id: passive

        visible: notices.text !== "" && !notices.busy
        anchors.right: parent.right
        width: Math.min(notices.width, implicitWidth)
        text: notices.text
        tone: notices.tone
        action: notices.action
        onHeldChanged: held ? later.stop() : notices.text !== "" && later.restart()
        onActed: {
            const run = notices.callback;
            notices.hide();
            if (run)
                run();
        }
        onClosed: notices.hide()
    }
}
