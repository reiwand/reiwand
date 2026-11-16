import QtQuick
import io.github.reiwand.app

QtObject {
    id: quitar

    function gather(uid, got, done, failed) {
        App.query("Get", [uid, got.length, 500], "Playlists1", reply => {
            if (!quitar)
                return;
            const all = got.concat(reply.items.map(item => item.uid));
            if (reply.items.length > 0 && all.length < reply.total)
                gather(uid, all, done, failed);
            else
                done(all);
        }, failed);
    }

    function restore(p, uids, done) {
        App.act("Create", [p.name, p.query ?? ""], "Playlists1", made => {
            if (p.query === null && uids.length > 0)
                App.act("AddItems", [made[0], uids], "Playlists1");
            ["order", "interval", "synced", "resume", "advance_while_paused", "skip_videos_on_battery"].forEach(key => App.act("SetRule", [made[0], key, String(p.rules[key])], "Playlists1"));
            if (done)
                done(made[0]);
        });
    }

    function take(p, done, failed) {
        const removing = uids => App.act("Delete", [p.uid], "Playlists1", () => quitar && done(uids), failed);
        if (p.query === null)
            gather(p.uid, [], removing, failed);
        else
            removing([]);
    }
}
