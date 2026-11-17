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
    property string animated: ""
    readonly property bool live: session.active === true
    readonly property var candidate: session.candidate ?? null
    readonly property var current: session.now ?? null
    readonly property var plays: session.plays ?? ({})
    readonly property bool calm: plays.why === "calm" || plays.why === "still"
    readonly property bool alone: !candidate || session.count <= 1 && current !== null && candidate.uid === current.uid
    readonly property var screens: App.outputs
    readonly property real unit: PreviewLook.unit(width)
    readonly property real aspect: width > 0 && height > 0 ? width / height : 16 / 9
    readonly property string mode: failure !== "" ? "error" : began && (!live || alone) ? "vacio" : "normal"
    readonly property string heading: screens.length > 1 && output !== "" ? App.tr("Change wallpaper · %1", output) : App.tr("Change wallpaper")
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

    function look() {
        const uid = candidate ? candidate.uid : "";
        animated = "";
        if (uid === "")
            return;
        App.query("GetItem", [uid], "Library1", reply => preview && preview.candidate && preview.candidate.uid === uid && !(App.state.on_battery ?? false) && reply.about?.photosensitive !== true && (preview.animated = App.url(reply.about?.animated ?? "")), () => {});
    }

    function take(state) {
        began = true;
        failure = "";
        if (state.ended)
            return finish(0);
        if (state.active)
            output = state.output;
        const was = candidate ? candidate.uid : "";
        session = state;
        panel.reel.sync(state);
        if (candidate && candidate.uid !== was)
            look();
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
        failure = error.message || App.tr("The wallpaper service does not answer.");
    }

    function heard(where, state) {
        if (where === output || state.active && !live)
            take(state);
    }

    function finish(code) {
        left = code;
        closing = true;
        if (!App.checking)
            leaving.start();
    }

    function choose(offset) {
        if (!working())
            return;
        offset === 0 ? ask("BrowseApply", [output, ""]) : ask("Browse", [output, session.source ?? "", offset]);
    }

    function working() {
        return mode !== "vacio" && mode !== "error";
    }

    function scroll(delta) {
        wheel += delta;
        const notches = Math.trunc(Math.abs(wheel) / PreviewLook.ruedaPaso);
        if (notches === 0 || !working())
            return;
        const steps = (wheel < 0 ? 1 : -1) * notches;
        wheel = 0;
        ask("Browse", [output, session.source ?? "", steps]);
    }

    function press(key) {
        if (key === Qt.Key_Escape)
            live ? App.query("BrowseCancel", [output], "Playlists1", reply => preview && preview.take(reply), () => preview && preview.finish(0)) : finish(0);
        else if (key === Qt.Key_Return || key === Qt.Key_Enter)
            mode === "vacio" ? finish(3) : mode === "error" ? load() : choose(0);
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
        const found = Qt.application.screens.find(s => s.name === output);
        if (found)
            screen = found;
        if (args.includes("--shots")) {
            Qt.createComponent("PreviewShots.qml").createObject(preview, {
                "preview": preview
            });
        } else {
            showFullScreen();
            requestActivate();
            load();
        }
        opening.start();
    }

    Kirigami.Theme.colorSet: Kirigami.Theme.Window
    Kirigami.Theme.inherit: false

    Timer {
        interval: PreviewLook.espera
        running: !preview.began
        onTriggered: preview.fail({})
    }

    NumberAnimation {
        id: opening

        target: dim
        property: "shade"
        from: 0
        to: 1
        duration: preview.calm ? 0 : PreviewLook.aparece
        easing.type: Easing.OutCubic
    }

    NumberAnimation {
        id: leaving

        target: dim
        property: "shade"
        to: 0
        duration: preview.calm ? 0 : PreviewLook.sale
        easing.type: Easing.OutCubic
        onFinished: Qt.exit(preview.left)
    }

    Connections {
        function onUpdated(topic, ids) {
            if (topic === "browse")
                preview.heard(ids[0], JSON.parse(ids[1]));
        }

        target: App
    }

    PreviewScreen {
        id: dim

        anchors.fill: parent
        preview: preview
    }
}
