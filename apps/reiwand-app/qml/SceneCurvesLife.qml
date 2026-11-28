import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

// The curves along the life of the particles of one system, in the Curves panel: the same editor the
// Inspector has, fed with the same query and actions.
ColumnLayout {
    id: life

    required property var page
    required property string system
    property var info: null

    function load() {
        const q = {
            "particles": {
                "layer": null,
                "time": page.editor.time,
                "sound": page.heard
            }
        };
        info = system ? JSON.parse(page.editor.rig(system, JSON.stringify(q)) || "null") : null;
    }

    function edit(what, args, gesture) {
        page.run("particles", {
            [what]: Object.assign({
                "system": system
            }, args)
        }, gesture === true);
    }

    onSystemChanged: load()
    Component.onCompleted: load()

    Connections {
        function onViewChanged() {
            if (life.system)
                life.load();
        }

        target: life.page.editor
    }

    SceneSwarmOverLife {
        Layout.fillWidth: true
        visible: life.info !== null
        tools: life
    }
}
