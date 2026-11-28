import QtQuick
import io.github.reiwand.app

QtObject {
    required property var page

    function ask(query) {
        return page.editor.rig("", JSON.stringify(query));
    }

    function exercise() {
        page.run("shader", {
                "new": {
                    "name": "bend",
                    "template": "distort"
                }
            });
        if (page.editor.error !== "")
            throw new Error("shaders: no effect was made: " + page.editor.error);
        const code = Qt.createComponent("SceneCodePanel.qml").createObject(page, {
                "page": page,
                "width": 900,
                "height": 600
            });
        code.openShader("bend");
        if (code.file?.kind !== "shader" || code.shader.saved.indexOf("fn uv_bend") < 0)
            throw new Error("shaders: the Code panel shows " + code.shader.saved);
        code.shader.reported(ask({
                    "shader_check": {
                        "effect": "bend",
                        "wgsl": "fn uv_bend(uv: vec2<f32>) -> vec2<f32> {\n    return nope;\n}\n"
                    }
                }));
        if (code.shader.line !== 2 || code.shader.report.ok)
            throw new Error("shaders: the error is not on its line: " + JSON.stringify(code.shader.report));
        code.shader.area.text = code.shader.area.text + "\n";
        // Ctrl+S in the Code panel is the file's Save: the text claims it before the scene's shortcut.
        code.shader.area.forceActiveFocus();
        if (!App.backend.claims(Qt.Key_S, Qt.ControlModifier))
            throw new Error("code: Ctrl+S in the Code panel goes to the scene's Save, not the file's");
        // The same on the list of files and on the panel itself, before the text ever had the keyboard.
        const inside = item => Array.from(item.children).reduce((found, one) => found ?? (one.objectName === "codeFiles" ? one : inside(one)), null);
        for (const item of [inside(code), code]) {
            item.forceActiveFocus();
            if (!App.backend.claims(Qt.Key_S, Qt.ControlModifier))
                throw new Error("code: Ctrl+S on " + item + " in the Code panel goes to the scene's Save");
        }
        code.shader.area.focus = false;
        code.focus = false;
        if (code.shader.area.activeFocus || App.backend.claims(Qt.Key_S, Qt.ControlModifier))
            throw new Error("code: Ctrl+S outside the Code panel no longer saves the scene");
        const listed = JSON.parse(page.editor.code("list", "{}") || "{}").scripts ?? [];
        if (listed.length > 0) {
            code.openScript(listed[0].path);
            if (code.files.length !== 2 || code.file.kind !== "script" || code.script.saved === "")
                throw new Error("code: a script did not open beside the shader: " + JSON.stringify(code.files));
            code.cycle(1);
            if (code.file.kind !== "shader" || !code.shader.changed)
                throw new Error("code: going back to the shader lost what was typed in it");
        }
        code.destroy();
        const api = Qt.createComponent("SceneApiPanel.qml").createObject(page, {
                "page": page,
                "width": 400,
                "height": 600
            });
        api.load();
        const whole = api.shown.reduce((n, part) => n + part.items.length, 0);
        if (api.parts.length === 0 || whole === 0)
            throw new Error("reference: the panel lists no part of the API");
        const named = api.shown[0].items[0].name;
        api.find.text = named;
        if (api.shown.reduce((n, part) => n + part.items.length, 0) >= whole)
            throw new Error("reference: the search did not narrow the list for " + named);
        api.destroy();
        const form = Qt.createComponent("SceneEffectForm.qml").createObject(page, {
                "page": page
            });
        form.show("bend");
        form.param("spin", {
                "set": {
                    "type": "float",
                    "unit": "degrees"
                }
            });
        if (!(form.parsed.params ?? []).some(p => p.id === "spin" && p.unit === "degrees"))
            throw new Error("shaders: the form did not add a dial: " + JSON.stringify(form.parsed));
        page.run("undo");
        form.load();
        if ((form.parsed.params ?? []).some(p => p.id === "spin"))
            throw new Error("shaders: undo kept the parameter");
        form.edit("lighter", {
                "set": {
                    "up_to": "q1"
                }
            });
        if (form.parsed.lighter?.passes?.[0]?.entry !== "fs_bend_lite" || form.parsed.lighter?.up_to !== "q1")
            throw new Error("shaders: the form made no lighter variant: " + JSON.stringify(form.parsed));
        const lite = Qt.createComponent("SceneEffectLighter.qml").createObject(page, {
                "form": form
            });
        if (lite === null || lite.variant === null)
            throw new Error("shaders: the lighter section does not show the variant");
        lite.destroy();
        page.run("undo");
        form.load();
        if ((form.parsed.lighter ?? null) !== null)
            throw new Error("shaders: undo kept the lighter variant");
        form.close();
        form.destroy();
        page.run("remove_effect", {
                "effect": "bend"
            });
    }
}
