pragma Singleton
import QtQuick

QtObject {
    property string quality: "q3"
    property int restart: 0
    // A seed for the preview only, so "New seed" shows other random values without changing the
    // scene; 0 keeps the seed of the scene.
    property int seed: 0
    property bool battery: false
    property bool calm: false
    property string language: ""
    // The time of day the preview sees, in seconds from midnight; -1 for the clock of the computer.
    property int timeOfDay: -1
    property bool rival: false
    property string versus: "q1"
    property bool versusGlow: false
    readonly property string effective: battery && levels.indexOf(quality) > 2 ? "q2" : quality
    property var media: null
    property var values: null
    property string preset: ""
    property string viewed: ""
    property var listen: null
    readonly property var sample: ({
            "title": "Night Drive",
            "artist": "Example Artist",
            "album": "Example Album",
            "playing": true,
            "position": 65,
            "duration": 200
        })
    readonly property var levels: ["q0", "q1", "q2", "q3"]
    // xgettext:no-c-format
    readonly property var drawn: [App.tr("half size"), App.tr("60 to 75 % size"), "", ""]
    readonly property var names: [App.tr("Slow computer"), App.tr("Medium computer"), App.tr("Fast computer"), App.tr("Very fast computer")]

    function preview(page) {
        const look = JSON.parse(page.editor.look || "{}");
        look.quality = rival ? versus : quality;
        look.restart = restart;
        if (seed > 0)
            look.seed = seed;
        else
            delete look.seed;
        look.battery = battery;
        look.calm = calm;
        if (language !== "")
            look.language = language;
        else
            delete look.language;
        if (timeOfDay >= 0)
            look.time_of_day = timeOfDay;
        else
            delete look.time_of_day;
        if (rival && versusGlow)
            look.bloom = false;
        else
            delete look.bloom;
        if (media)
            look.media = media;
        else
            delete look.media;
        if (values !== null || preset !== "") {
            look.values = values ?? {};
            look.preset = preset;
        } else {
            delete look.values;
            delete look.preset;
        }
        if (listen !== null)
            look.listen = listen;
        else
            delete look.listen;
        page.editor.look = JSON.stringify(look);
        page.editor.seek(listen !== null ? 0 : page.clock, listen !== null || page.editor.playing);
    }

    function hear(page, what) {
        listen = what;
        again(page);
    }

    function newSeed(page) {
        seed = 1 + Math.floor(Math.random() * 2147483646);
        again(page);
    }

    function again(page) {
        restart += 1;
        preview(page);
    }

    function tryValue(page, key, value) {
        const next = Object.assign({}, values ?? {});
        next[key] = value;
        values = next;
        preview(page);
    }

    function tryPreset(page, id) {
        values = {};
        preset = id;
        preview(page);
    }

    function untry(page) {
        values = null;
        preset = "";
        preview(page);
    }
}
