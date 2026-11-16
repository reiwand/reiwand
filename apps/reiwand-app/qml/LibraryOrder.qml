import QtQuick
import io.github.reiwand.app

QtObject {
    id: order

    required property var page
    readonly property var outputs: App.state.outputs ?? {}
    readonly property var using: unique(Object.keys(outputs).map(name => String(outputs[name].item ?? "")))
    readonly property var lists: Object.keys(outputs).map(name => outputs[name].playlist).filter(list => !!list)
    readonly property string key: unique(lists.map(list => list.uid)).join(",")
    property var listed: []
    readonly property var lead: unique(using.concat(listed.map(entry => entry.uid))).slice(0, 500)
    readonly property string term: lead.length > 0 ? "first:" + lead.join(",") : ""
    property int count: 0
    property string landing
    readonly property Connections watch: Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "playlists")
                order.load();
        }
    }

    function unique(uids) {
        return uids.filter((uid, at) => uid !== "" && uids.indexOf(uid) === at);
    }

    function load() {
        const names = {};
        lists.forEach(list => names[list.uid] = list.name);
        const uids = key === "" ? [] : key.split(",");
        const got = {};
        listed = [];
        uids.forEach(uid => App.query("Get", [uid, 0, 500], "Playlists1", reply => {
            if (!order)
                return;
            got[uid] = (reply.items ?? []).map(item => ({
                        "uid": item.uid,
                        "list": names[uid]
                    }));
            order.listed = uids.reduce((all, at) => all.concat(got[at] ?? []), []);
        }));
    }

    function land(uid) {
        landing = page.all && using.length > 0 ? using[0] : uid;
        page.search();
    }

    function reloaded() {
        const view = page.covers.view;
        const at = landing !== "" ? page.library.row(landing) : -1;
        page.sync();
        if (page.library.total > 0 && (landing !== "" || view.currentIndex < 0))
            view.currentIndex = Math.max(0, at);
        landing = "";
        measure();
        Qt.callLater(page.covers.keep);
        Qt.callLater(page.covers.look);
    }

    function refill() {
        const view = page.covers.view;
        if (page.library.total > 0 && view.currentIndex < 0)
            view.currentIndex = 0;
        page.covers.keep();
        Qt.callLater(page.covers.look);
    }

    function mark(uid) {
        if (!page.all || using.includes(uid))
            return "";
        return listed.find(entry => entry.uid === uid)?.list ?? "";
    }

    function measure() {
        const library = page.library;
        let at = 0;
        while (page.all && at < library.total && lead.includes(String(library.uid(at))))
            at++;
        count = at < library.total ? at : 0;
    }

    onKeyChanged: load()
    onTermChanged: page.all && page.search()
    Component.onCompleted: load()
}
