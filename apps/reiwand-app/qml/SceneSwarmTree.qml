import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The Particles panel: the scene's particle systems, as the systems list has them (make, copy,
// rename, remove), and under the system in hand its parts in order (its settings, each emitter,
// how particles start, each force, each collider, the look, the control points and the child
// systems), then the layers that draw it. Choosing a part or a layer shows it in the Inspector, as
// choosing a layer does.
QQC2.ScrollView {
    id: tree

    required property var page
    readonly property string system: {
        const found = /^\/scene\/particles\/([^\/]+)/.exec(page.editor.selected ?? "");
        return found ? found[1].replace(/~1/g, "/").replace(/~0/g, "~") : "";
    }
    readonly property var parsed: {
        page.view;
        return system ? JSON.parse(page.editor.value(SceneSwarmKnobs.pointer(system, "")) || "null") ?? {} : {};
    }
    // [label, pointer under the system, depth]
    readonly property var parts: {
        if (!system)
            return [];
        const out = [[App.tr("Settings"), "", 0]];
        const kind = (item, skip) => Object.keys(item ?? {}).find(key => !skip.includes(key) && item[key] !== null && typeof item[key] === "object" && !Array.isArray(item[key])) ?? "";
        const listed = (key, title, word) => {
            const items = parsed[key] ?? [];
            if (items.length > 0)
                out.push([title, "", -1]);
            items.forEach((item, at) => out.push([word(item, at), "/" + key + "/" + at, 1]));
        };
        listed("emitters", App.tr("Emitters"), (item, at) => App.tr("%1. %2", at + 1, SceneWords.label(kind(item.shape, []) || "emitter")));
        if (parsed.init)
            out.push([App.tr("How particles start"), "/init", 0]);
        listed("forces", App.tr("Forces"), (item, at) => App.tr("%1. %2", at + 1, SceneWords.label(kind(item, ["window"]) || "force")));
        listed("collide", App.tr("Colliders"), (item, at) => App.tr("%1. %2", at + 1, SceneWords.label(kind(item.bounds, []) || "collider")));
        if (parsed.render)
            out.push([App.tr("Look"), "/render", 0]);
        listed("points", App.tr("Control points"), (item, at) => App.tr("Point %1", at));
        listed("children", App.tr("Child systems"), (item, at) => App.tr("%1. %2", at + 1, String(item.system ?? item.id ?? App.tr("Child"))));
        return out;
    }
    // The particle layers that draw the system in hand.
    readonly property var users: {
        page.view;
        return system ? (page.view.layers ?? []).filter(item => item.type === "particles" && JSON.parse(page.editor.value(item.pointer + "/system") || "null") === system) : [];
    }

    contentWidth: availableWidth
    clip: true

    ScrollColumn {
        view: tree

        SceneSwarmTreeBar {
            Layout.fillWidth: true
            // The panel sets the width of the bar, never the bar that of the panel.
            Layout.preferredWidth: 0
            page: tree.page
            tree: tree
        }

        SceneSwarmSystems {
            Layout.fillWidth: true
            page: tree.page
            system: tree.system
        }

        PanelHeading {
            Layout.fillWidth: true
            Layout.topMargin: Look.e2
            visible: tree.system !== ""
            level: 4
            text: App.tr("Parts of %1", tree.system)
            elide: Text.ElideRight
        }

        QQC2.Label {
            Layout.fillWidth: true
            visible: tree.system === ""
            wrapMode: Text.Wrap
            color: Look.mist
            text: App.tr("Choose a system to see its parts here; each part opens in the Inspector.")
        }

        Repeater {
            model: tree.parts

            delegate: QQC2.ItemDelegate {
                id: part

                required property var modelData
                readonly property string pointer: SceneSwarmKnobs.pointer(tree.system, modelData[1])
                readonly property bool heading: modelData[2] < 0

                Layout.fillWidth: true
                implicitHeight: DeskLook.row
                topPadding: 0
                bottomPadding: 0
                leftPadding: Look.e2 + Math.max(0, modelData[2]) * Look.e4
                enabled: !heading
                marked: !heading && tree.page.editor.selected === pointer
                highlighted: marked && SceneFocus.lit(part)
                text: modelData[0]
                font.weight: heading ? Font.DemiBold : Font.Normal
                Accessible.name: heading ? text : App.tr("%1 of %2", text, tree.system)
                onClicked: tree.page.editor.select(pointer)
            }
        }

        PanelHeading {
            Layout.fillWidth: true
            Layout.topMargin: Look.e2
            visible: tree.system !== ""
            level: 4
            text: App.tr("Layers that use the system (%1)", tree.users.length)
        }

        QQC2.Label {
            Layout.fillWidth: true
            visible: tree.system !== "" && tree.users.length === 0
            wrapMode: Text.Wrap
            color: Look.mist
            text: App.tr("No layer draws %1 yet.", tree.system)
        }

        Repeater {
            model: tree.users

            delegate: QQC2.ItemDelegate {
                id: user

                required property var modelData

                Layout.fillWidth: true
                implicitHeight: DeskLook.row
                topPadding: 0
                bottomPadding: 0
                leftPadding: Look.e2
                icon.name: "stack"
                marked: tree.page.editor.selected === modelData.pointer
                highlighted: marked && SceneFocus.lit(user)
                text: modelData.id
                onClicked: tree.page.editor.select(modelData.pointer)
            }
        }
    }
}
