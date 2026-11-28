import QtQuick
import io.github.reiwand.app

QtObject {
    id: check

    required property var host
    property var removal: null

    function must(ok, text) {
        if (ok)
            return;
        finish();
        throw new Error("removing a playlist: " + text);
    }

    function finish() {
        host.waiting--;
        host.held = host.held.filter(c => c !== check);
    }

    function failed(error) {
        must(false, error.message);
    }

    function restored(p, uids, made) {
        must(made === p.uid, "undo does not create the playlist again");
        finish();
    }

    function removed(p, uids) {
        must(uids.length === 2 && uids.every(uid => typeof uid === "string" && uid.length > 0), "the wallpapers of the playlist are not kept for undo: " + uids.length);
        removal.restore(p, uids, made => check && check.restored(p, uids, made));
    }

    function start() {
        host.waiting++;
        host.held = host.held.concat([check]);
        removal = Qt.createComponent("PlaylistRemove.qml").createObject(check);
        App.query("List", [], "Playlists1", reply => {
            if (!check)
                return;
            const p = reply.playlists[0];
            removal.take(p, uids => check && check.removed(p, uids), error => check && check.failed(error));
        }, error => check && check.failed(error));
    }
}
