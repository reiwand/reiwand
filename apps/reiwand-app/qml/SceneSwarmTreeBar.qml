import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The bar of the Particles panel: a new system, a new part of the system in hand (an emitter, a
// force, a collider or a control point, which then opens in the Inspector) and the preview from its
// start.
RowLayout {
    id: bar

    required property var page
    required property var tree
    readonly property string system: tree.system
    // The words go where the three of them do not fit beside each other.
    readonly property bool narrow: width < Look.smallPanel * 0.7
    readonly property alias menu: parts
    // [name, list under the system, what the action adds]
    readonly property var modules: [[App.tr("Add an emitter"), "emitters", "shape", count => ({
                        "emitter": count,
                        "shape": {
                            "point": [960, 540]
                        }
                    })], [App.tr("Add a force"), "forces", "force", () => ({
                        "force": {
                            "drag": 0.25
                        }
                    })], [App.tr("Add a collider"), "collide", "collider", () => ({
                        "collider": {
                            "bounds": "canvas",
                            "bounce": 0.5
                        }
                    })], [App.tr("Add a control point"), "points", "point", count => ({
                        "point": count,
                        "at": [0, 0]
                    })]]

    function fresh() {
        const ids = Object.keys(JSON.parse(page.editor.value("/scene/particles") || "null") ?? {});
        let n = 1;
        while (ids.includes("system_" + n))
            n++;
        return "system_" + n;
    }

    function addSystem() {
        const name = fresh();
        page.run("particles", {
            "create": {
                "system": name
            }
        });
        if (page.editor.error === "")
            page.editor.select(SceneSwarmKnobs.pointer(name, ""));
    }

    function addModule(module) {
        const count = (tree.parsed[module[1]] ?? []).length;
        page.run("particles", {
            [module[2]]: Object.assign({
                "system": system
            }, module[3](count))
        });
        if (page.editor.error === "")
            page.editor.select(SceneSwarmKnobs.pointer(system, "/" + module[1] + "/" + count));
    }

    spacing: 0

    QQC2.ToolButton {
        objectName: "addSystem"
        icon.name: "list-add"
        text: App.tr("System")
        display: bar.narrow ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
        Accessible.name: App.tr("Add a system without a layer")
        QQC2.ToolTip.text: Accessible.name
        QQC2.ToolTip.visible: hovered
        onClicked: bar.addSystem()
    }

    QQC2.ToolButton {
        id: module

        objectName: "addModule"
        icon.name: "list-add"
        text: App.tr("Module")
        enabled: bar.system !== ""
        display: bar.narrow ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
        Accessible.name: App.tr("Add a part to %1", bar.system)
        QQC2.ToolTip.text: enabled ? Accessible.name : App.tr("Choose a system to add a part to it")
        QQC2.ToolTip.visible: hovered
        onClicked: parts.popup(module, 0, module.height)

        QQC2.Menu {
            id: parts

            objectName: "moduleMenu"

            Repeater {
                model: bar.modules

                delegate: QQC2.MenuItem {
                    required property var modelData

                    text: modelData[0]
                    onTriggered: bar.addModule(modelData)
                }
            }
        }
    }

    Item {
        Layout.fillWidth: true
    }

    QQC2.ToolButton {
        icon.name: "arrows-clockwise"
        text: App.tr("Restart")
        display: bar.narrow ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
        Accessible.name: App.tr("Restart the preview")
        QQC2.ToolTip.text: Accessible.name
        QQC2.ToolTip.visible: hovered
        onClicked: SceneLook.again(bar.page)
    }
}
