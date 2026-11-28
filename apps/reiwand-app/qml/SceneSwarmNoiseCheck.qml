import QtQuick
import io.github.reiwand.app

QtObject {
    required property var page

    function read(pointer) {
        return JSON.parse(page.editor.value(pointer) || "null");
    }

    function exercise(check) {
        const tools = Qt.createQmlObject("import QtQuick; QtObject { property var info; property var page; property var check; property string system: 'flakes'; function edit(what, args, gesture) { check.edit(what, args); info = check.ask(); } }", page);
        tools.page = page;
        tools.check = check;
        tools.info = check.ask();
        const panel = Qt.createComponent("SceneSwarmNoise.qml").createObject(page, {
                "tools": tools
            });
        tools.edit("force", {
                "force": {
                    "noise_field": App.fill("Offer.noise_field")
                }
            });
        if (panel.list.length !== 1)
            throw new Error("noise: the added noise field is not listed: " + JSON.stringify(tools.info.noises));
        const index = panel.list[0].index, at = "/scene/particles/flakes/forces/" + index;
        const written = read(at + "/noise_field");
        for (const key of ["strength", "phase", "scale", "drift", "mask"])
            if (written?.[key] === undefined)
                throw new Error("noise: the added noise field does not write " + key + ": " + JSON.stringify(written));
        if (JSON.stringify(read(at + "/window")) !== "[0,0,1,1]")
            throw new Error("noise: the added noise field has no window");
        panel.patch(index, n => n.strength = {
                "min": 15,
                "max": 90
            });
        panel.patch(index, n => n.mask[2] = 0.5);
        panel.patch(index, n => n.drift = -25);
        const changed = read(at + "/noise_field");
        if (changed.strength.min !== 15 || changed.strength.max !== 90 || changed.mask[2] !== 0.5 || changed.drift !== -25)
            throw new Error("noise: the panel did not change the noise field: " + JSON.stringify(changed));
        if (panel.list[0].strength.max !== 90)
            throw new Error("noise: the panel does not show the new strength");
        const windows = Qt.createComponent("SceneSwarmWindows.qml").createObject(page, {
                "tools": tools
            });
        tools.edit("ramp", {
                "index": index,
                "ramp": "linear"
            });
        if (read(at + "/ramp") !== "linear" || windows.forces[index]?.ramp !== "linear" || windows.name(windows.forces[index]) !== "noise_field")
            throw new Error("windows: the linear ramp was not written or the force lost its name: " + JSON.stringify(read(at)));
        tools.edit("ramp", {
                "index": index
            });
        if (read(at + "/ramp") !== null)
            throw new Error("windows: removing the ramp left it: " + JSON.stringify(read(at)));
        windows.destroy();
        tools.edit("force", {
                "index": index,
                "remove": true
            });
        if (panel.list.length !== 0)
            throw new Error("noise: the removed noise field is still listed");
        panel.destroy();
        tools.destroy();
    }
}
