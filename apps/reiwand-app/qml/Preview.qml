import QtQuick
import QtQuick.Window
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Window {
    id: preview

    readonly property var args: Qt.application.arguments
    property string output: args.indexOf("--browse") >= 0 ? (args[args.indexOf("--browse") + 1] ?? "") : ""
    property var session: ({})
    property var log: []
    property string failure: ""
    property bool began: false
    property bool moved: false
    property bool closing: false
    property bool heldFocus: false
    property bool retargeted: false
    property int left: -1
    property real wheel: 0
    property alias panel: dim.panel
    property alias animated: motion.animated
    property alias playUid: motion.playUid
    property alias warm: warm
    property alias opening: fades.opening
    property int quiet: App.checking ? 0 : PreviewLook.rest
    readonly property bool touching: dim.touching
    readonly property bool live: session.active === true
    readonly property var candidate: session.candidate ?? null
    readonly property var current: session.now ?? null
    readonly property var plays: session.plays ?? ({})
    readonly property bool calm: plays.why === "calm" || plays.why === "still"
    readonly property bool alone: !candidate || session.count <= 1 && current !== null && candidate.uid === current.uid
    readonly property var screens: App.outputs
    readonly property real unit: PreviewLook.unit(width)
    readonly property real aspect: width > 0 && height > 0 ? width / height : 16 / 9
    readonly property string mode: failure !== "" ? "error" : began && (!live || alone) ? "empty" : "normal"
    readonly property string heading: screens.length > 1 && output !== "" ? App.tr("Change wallpaper · %1", App.state.outputs[output]?.screen?.identity || output) : App.tr("Change wallpaper")
    readonly property string origin: session.source === "playlist" ? App.tr("List «%1»", session.playlist_name ?? "") : App.tr("Your library")
    readonly property string position: live ? App.tr("%1 of %2", (session.position ?? 0) + 1, session.count ?? 0) : ""

    function ask(member, values) {
        log = log.concat([member + " " + JSON.stringify(values)]);
        App.query(member, values, "Playlists1", reply => preview && preview.take(reply), error => preview && preview.fail(error));
    }

    function load() {
        failure = "";
        ask("GetBrowse", [output]);
    }

    function take(state) {
        began = true;
        failure = "";
        if (state.ended)
            return finish(0);
        if (state.active)
            output = state.output;
        warm.show();
        const was = candidate ? candidate.uid : "";
        session = state;
        rest();
        panel.reel.sync(state);
        if (candidate && candidate.uid !== was)
            motion.look();
        if (state.active && output === "" && !retargeted && screens.length > 1 && screen.name !== "")
            retarget();
    }

    function retarget() {
        retargeted = true;
        const was = output;
        output = screen.name;
        ask("Browse", [output, "", 1]);
        App.query("BrowseCancel", [was], "Playlists1", () => {}, () => {});
    }

    function fail(error) {
        began = true;
        warm.show();
        failure = error.message || App.tr("The wallpaper service does not answer.");
    }

    function heard(where, state) {
        if (warm.resting)
            state.active && warm.wake(state);
        else if (where === output || state.active && !live)
            take(state);
    }

    function finish(code) {
        fades.idle.stop();
        left = code;
        closing = true;
        if (!App.checking)
            fades.leaving.start();
    }

    function choose(offset) {
        if (!working())
            return;
        offset === 0 ? ask("BrowseApply", [output, ""]) : ask("Browse", [output, session.source ?? "", offset]);
    }

    function working() {
        return mode !== "empty" && mode !== "error";
    }

    function rest() {
        fades.idle.stop();
        if (quiet > 0 && live && !closing && !touching)
            fades.idle.start();
    }

    function scroll(delta) {
        rest();
        wheel += delta;
        const notches = Math.trunc(Math.abs(wheel) / PreviewLook.wheelStep);
        if (notches === 0 || !working())
            return;
        const steps = (wheel < 0 ? 1 : -1) * notches;
        wheel = 0;
        ask("Browse", [output, session.source ?? "", steps]);
    }

    function press(key) {
        rest();
        if (key === Qt.Key_Escape)
            live ? ask("BrowseCancel", [output]) : finish(0);
        else if (key === Qt.Key_Return || key === Qt.Key_Enter)
            mode === "empty" ? finish(3) : mode === "error" ? load() : choose(0);
        else if (working() && (key === Qt.Key_Left || key === Qt.Key_Right))
            choose(key === Qt.Key_Left ? -1 : 1);
        else
            return false;
        return true;
    }

    visible: false
    color: "transparent"
    flags: Qt.Window | Qt.FramelessWindowHint | Qt.WindowStaysOnTopHint
    title: App.tr("Change wallpaper")
    onActiveChanged: {
        if (active)
            heldFocus = true;
        else if (heldFocus && !closing && !App.checking)
            press(Qt.Key_Escape);
    }
    Component.onCompleted: {
        if (args.includes("--shots")) {
            Qt.createComponent("PreviewShots.qml").createObject(preview, {
                "preview": preview
            });
        } else {
            load();
        }
    }

    Kirigami.Theme.colorSet: Kirigami.Theme.Window
    Kirigami.Theme.inherit: false

    Connections {
        function onUpdated(topic, ids) {
            if (topic === "browse")
                preview.heard(ids[0], JSON.parse(ids[1]));
        }

        target: App
    }

    PreviewTheme {
        host: preview
    }

    PreviewWarm {
        id: warm

        preview: preview
    }

    PreviewMotion {
        id: motion

        preview: preview
    }

    PreviewFades {
        id: fades

        dim: dim
        preview: preview
    }

    PreviewScreen {
        id: dim

        anchors.fill: parent
        preview: preview
    }
}
