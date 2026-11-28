import QtQuick
import io.github.reiwand.app

QtObject {
    id: jobs

    property var job: -1
    property var seen: ({})

    signal ended(var id, string state, string message)

    function reset() {
        job = -1;
    }

    function started(id) {
        job = id;
        const done = seen[id];
        if (done)
            finish(id, done.state, done.message);
    }

    function finish(id, state, message) {
        if (id !== job)
            return;
        job = -1;
        ended(id, state, message);
    }

    property Connections watch: Connections {
        target: App.backend

        function onJobFinished(id, state, name, message) {
            const keep = Object.keys(jobs.seen).slice(-15);
            const next = {};
            keep.forEach(k => next[k] = jobs.seen[k]);
            next[id] = {
                "state": state,
                "message": message
            };
            jobs.seen = next;
            jobs.finish(id, state, message);
        }
    }
}
