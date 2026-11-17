import QtQuick
import io.github.reiwand.app

QtObject {
    required property var page

    function exercise() {
        const tools = Qt.createComponent("SceneSwarmTools.qml").createObject(page, {
                "page": page,
                "system": "flakes"
            });
        const find = name => CheckKit.find(tools, item => item.objectName === name);
        const render = () => JSON.parse(page.editor.value("/scene/particles/flakes/render") || "null");
        const on = find("stretchOn");
        if (!on)
            throw new Error("stretch: the panel has no switch for it");
        on.checked = true;
        on.toggled();
        tools.load();
        if (render()?.stretch?.mode !== "add" || tools.info.stretch === null)
            throw new Error("stretch: switching it on did not write the mode: " + JSON.stringify(render()));
        const mode = find("stretchMode");
        mode.activated(1);
        tools.load();
        if (render().stretch.mode !== "relative" || tools.info.stretch.mode !== "relative")
            throw new Error("stretch: the mode selector did not set the relative mode: " + JSON.stringify(render()));
        const max = find("stretch_max");
        max.value = 3;
        max.moved();
        if (render().stretch.max !== 3)
            throw new Error("stretch: the longest slider did not write the maximum: " + JSON.stringify(render().stretch));
        on.checked = false;
        on.toggled();
        if (render()?.stretch !== undefined)
            throw new Error("stretch: switching it off left the stretch");
        find("waveAdd").clicked();
        tools.load();
        if (tools.info.waves.length !== 1 || tools.info.waves[0].from !== "floor")
            throw new Error("waves: adding a pulse did not list it from the floor: " + JSON.stringify(tools.info.waves));
        find("waveFrom").activated(0);
        tools.load();
        if (tools.info.waves[0].from !== "top")
            throw new Error("waves: the base selector did not set top");
        tools.destroy();
    }
}
