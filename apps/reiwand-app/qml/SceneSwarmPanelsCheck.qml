import QtQuick
import io.github.reiwand.app

QtObject {
    required property var page

    function named(item, name) {
        return CheckKit.find(item, node => node.objectName === name);
    }

    function exercise() {
        emitterAdd();
        hullLabel();
        overLife();
    }

    function emitterAdd() {
        const tools = Qt.createComponent("SceneSwarmTools.qml").createObject(page, {
                "page": page,
                "system": "flakes"
            });
        const count = () => tools.info.emitters.length;
        const before = count();
        const add = CheckKit.find(tools, item => item.objectName === "addEmitter");
        if (!add || !add.enabled)
            throw new Error("emitters: the panel has no button to add an emitter");
        add.clicked();
        tools.load();
        if (count() !== before + 1)
            throw new Error("emitters: adding an emitter left " + count() + " of " + (before + 1));
        const drop = CheckKit.find(tools, item => item.objectName === "removeEmitter" && item.enabled);
        if (!drop)
            throw new Error("emitters: an emitter has no button to remove it");
        const combo = CheckKit.find(drop.parent.parent, item => item.hasOwnProperty("popup") && item.parent === drop.parent);
        if (combo && drop.parent.parent.implicitWidth >= 0 && drop.x > combo.x)
            throw new Error("emitters: the remove button sits after the shape selector, where a narrow panel hides it");
        drop.clicked();
        tools.load();
        if (count() !== before)
            throw new Error("emitters: removing an emitter left " + count() + " of " + before);
        page.run("undo");
        page.run("undo");
        tools.destroy();
    }

    function hullLabel() {
        const motion = {
            "page": page,
            "layerId": "",
            "bone": "",
            "bones": ["spine"],
            "info": {
                "model": true,
                "colliders": [
                    {
                        "bone": "spine",
                        "shape": "capsule",
                        "radius": 0.2
                    }
                ]
            }
        };
        const hulls = Qt.createComponent("SceneModelColliders.qml").createObject(page, {
                "motion": motion
            });
        const slider = CheckKit.find(hulls, item => item.objectName === "hullRadius");
        if (!slider || slider.Accessible.name !== App.tr("Radius"))
            throw new Error("hulls: the collider radius slider has no name");
        hulls.destroy();
    }

    function overLife() {
        const tools = Qt.createComponent("SceneSwarmTools.qml").createObject(page, {
                "page": page,
                "system": "flakes"
            });
        const sizes = () => tools.info.render.size_over_life ?? [];
        const colors = () => tools.info.render.color_over_life ?? [];
        const steps = page.view.history.done.length;
        tools.edit("over", {
            "size": [],
            "color": []
        });
        tools.load();
        if (sizes().length > 0 || colors().length > 0)
            throw new Error("over life: the curves did not clear");
        named(tools, "sizeOverLife").clicked();
        named(tools, "colorOverLife").clicked();
        tools.load();
        if (sizes().length !== 2 || colors().length !== 2)
            throw new Error("over life: the buttons did not start the size and color curves: " + page.editor.error);
        if (!named(tools, "sizeCurve").visible || !named(tools, "colorBar").visible)
            throw new Error("over life: the curve and the gradient did not show");
        named(tools, "sizeCurve").send([[0, 0], [0.5, 1.5], [1, 0]], false);
        tools.load();
        if (sizes().length !== 3)
            throw new Error("over life: the size curve did not take its new points");
        named(tools, "sizeOverLife").clicked();
        named(tools, "colorOverLife").clicked();
        tools.load();
        if (sizes().length !== 0 || colors().length !== 0)
            throw new Error("over life: the remove buttons left curves");
        while (page.view.history.done.length > steps)
            page.run("undo");
        tools.destroy();
    }
}
