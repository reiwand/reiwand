import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

DropArea {
    id: drop

    required property var page
    property bool layers: false
    readonly property var kinds: ({
            "image": ["png", "jpg", "jpeg", "webp", "gif", "bmp", "ktx2"],
            "video": ["webm", "mp4", "mkv"],
            "sound": ["ogg", "opus", "flac", "wav", "mp3"]
        })

    // The sound layers a scene plays at most (MAX_SOUNDS of the scene format).
    readonly property int soundsAtMost: 8
    // What the editor imports besides the kinds above (asset_kind of the editor).
    readonly property var others: ["hdr", "exr", "ttf", "otf", "ttc", "otc", "woff2", "cube"]

    function kindOf(file) {
        const ext = file.split(".").pop().toLowerCase();
        return Object.keys(kinds).find(kind => kinds[kind].includes(ext)) ?? "";
    }

    // Many files dropped at once are one change, undone in one go, and the sounds stop where the
    // scene stops playing them; the notice says how many were left out.
    function take(files) {
        let room = soundsAtMost - (page.view.layers ?? []).filter(layer => layer.type === "sound").length;
        let left = 0, foreign = 0;
        const actions = [];
        for (const file of files) {
            const kind = layers ? kindOf(file) : "";
            if (kind === "sound" && room-- <= 0)
                left++;
            else if (!kindOf(file) && !others.includes(file.split(".").pop().toLowerCase()))
                foreign++;
            else
                actions.push(kind ? {
                    "add_layer": {
                        "kind": kind,
                        "file": file
                    }
                } : {
                    "import": {
                        "file": file
                    }
                });
        }
        const one = action => page.run(Object.keys(action)[0], Object.values(action)[0]);
        if (actions.length > 1)
            page.run("steps", actions);
        // A file the editor refuses undoes the whole step: then the others go in one by one.
        if (actions.length === 1 || actions.length > 1 && page.editor.error !== "")
            actions.forEach(one);
        const told = [];
        if (left > 0)
            told.push(App.tr("%1 sound files were left out: a scene plays at most %2 sound layers.", left, soundsAtMost));
        if (foreign > 0)
            told.push(App.tr("%1 files were left out: only pictures, videos, sounds, fonts and .cube tables can be imported.", foreign));
        if (told.length > 0)
            page.notice = told.join(" ");
    }

    keys: ["text/uri-list"]
    onDropped: event => take(event.urls.map(url => App.path(url)).filter(file => file))

    Rectangle {
        anchors.fill: parent
        visible: drop.containsDrag
        color: Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.2)
        border.color: Kirigami.Theme.highlightColor
        border.width: 2

        QQC2.Label {
            anchors.centerIn: parent
            text: drop.layers ? App.tr("Drop files to add them as layers") : App.tr("Drop files to import them")
        }
    }
}
