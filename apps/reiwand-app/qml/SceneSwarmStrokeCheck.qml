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
        const depth = find("stretchDepth");
        depth.activated(1);
        tools.load();
        if (render().stretch.depth !== "projected" || tools.info.stretch.depth !== "projected")
            throw new Error("stretch: the depth selector did not set the projected depth: " + JSON.stringify(render()));
        const max = find("stretch_max");
        max.value = 3;
        max.moved();
        if (render().stretch.max !== 3)
            throw new Error("stretch: the longest slider did not write the maximum: " + JSON.stringify(render().stretch));
        const centre = find("stretchCentre");
        centre.checked = true;
        centre.toggled();
        tools.load();
        mode.activated(0);
        tools.load();
        if (render().stretch.anchor !== "centre" || render().stretch.mode !== "add" || tools.info.stretch.anchor !== "centre")
            throw new Error("stretch: the centred anchor was not written or a mode change lost it: " + JSON.stringify(render().stretch));
        centre.checked = false;
        centre.toggled();
        tools.load();
        if ("anchor" in render().stretch)
            throw new Error("stretch: turning the centred anchor off left the field: " + JSON.stringify(render().stretch));
        on.checked = false;
        on.toggled();
        if (render()?.stretch !== undefined)
            throw new Error("stretch: switching it off left the stretch");
        const slim = find("aspectSlider");
        slim.value = Math.log(0.25);
        slim.moved();
        tools.load();
        if (render()?.aspect !== 0.25 || tools.info.sprite.width !== tools.info.sprite.side * 0.25 || !find("aspectSprite"))
            throw new Error("aspect: the slider did not write the proportion or the rectangle is not drawn: " + JSON.stringify(render()));
        find("aspectSquare").clicked();
        if (render()?.aspect !== 1)
            throw new Error("aspect: the square button did not write 1: " + JSON.stringify(render()));
        const init = () => JSON.parse(page.editor.value("/scene/particles/flakes/init") || "null");
        if (JSON.stringify(init()?.tilt?.angle) !== "[[0,0],[0,0]]" || !find("tiltSprite0.5"))
            throw new Error("tilt: the system lacks its zero ranges or the preview is not drawn: " + JSON.stringify(init()?.tilt));
        const high = find("tilt_angle11");
        high.value = 60;
        high.moved();
        if (init().tilt.angle[1][1] !== 60)
            throw new Error("tilt: the slider did not write the angle: " + JSON.stringify(init().tilt));
        find("waveAdd").clicked();
        tools.load();
        if (tools.info.waves.length !== 1 || tools.info.waves[0].from !== "floor")
            throw new Error("waves: adding a pulse did not list it from the floor: " + JSON.stringify(tools.info.waves));
        find("waveFrom").activated(0);
        tools.load();
        if (tools.info.waves[0].from !== "top")
            throw new Error("waves: the base selector did not set top");
        const pulse = () => JSON.parse(page.editor.value("/scene/particles/flakes/forces/" + tools.info.waves[0].index) || "null")?.oscillate;
        find("waveShared").toggle();
        find("waveShared").toggled();
        tools.load();
        if (pulse()?.random !== "shared" || !tools.info.waves[0].shared)
            throw new Error("waves: the shared draw was not written: " + JSON.stringify(pulse()));
        find("waveShared").toggle();
        find("waveShared").toggled();
        tools.load();
        if (pulse() === null || "random" in pulse())
            throw new Error("waves: turning the shared draw off left the field: " + JSON.stringify(pulse()));
        const first = () => JSON.parse(page.editor.value("/scene/particles/flakes/emitters/0/shape") || "null");
        tools.edit("shape", {
                "emitter": 0,
                "shape": {
                    "sphere": {
                        "center": [960, 540],
                        "depth": 0,
                        "radius": [200, 100, 300],
                        "inner": 0,
                        "sign": [0, 0, 0]
                    }
                }
            });
        tools.load();
        if (!find("ballSide") || !find("ballInner"))
            throw new Error("sphere: the side view and the hollow slider are not shown");
        find("ballSign2").clicked();
        tools.load();
        if (JSON.stringify(first()?.sphere?.sign) !== "[0,0,1]")
            throw new Error("sphere: the toggle did not keep the births on the far side: " + JSON.stringify(first()));
        const inner = find("ballInner");
        inner.value = 0.5;
        inner.moved();
        if (first().sphere.inner !== 0.5)
            throw new Error("sphere: the hollow slider did not write the inner fraction: " + JSON.stringify(first()));
        const push = find("ballPush");
        push.value = 200;
        push.moved();
        tools.load();
        find("ballSign0").clicked();
        tools.load();
        if (first().sphere.push !== 200 || first().sphere.inner !== 0.5)
            throw new Error("sphere: the push slider did not write push, or another change dropped it: " + JSON.stringify(first()));
        find("ballPush").value = 0;
        find("ballPush").moved();
        if ("push" in first().sphere)
            throw new Error("sphere: a push of 0 stayed in the shape: " + JSON.stringify(first()));
        tools.destroy();
    }
}
