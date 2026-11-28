import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

ColumnLayout {
    id: panel

    required property var page
    readonly property var layers: page.view.layers ?? []
    readonly property var current: layers.find(layer => layer.pointer === page.editor.selected) ?? null
    readonly property var siblings: current ? layers.filter(layer => layer.pointer.replace(/\/\d+$/, "") === current.pointer.replace(/\/\d+$/, "")) : []
    readonly property var shown: {
        const out = [];
        const visit = parent => layers.filter(layer => layer.pointer.replace(/\/\d+$/, "") === parent).reverse().forEach(layer => {
                out.push(layer);
                visit(layer.pointer + "/layers");
            });
        visit("/scene/layers");
        return out;
    }
    readonly property int index: current ? Number(current.pointer.split("/").pop()) : -1
    readonly property string group: {
        const parent = current ? current.pointer.replace(/\/layers\/\d+$/, "") : "";
        return layers.find(layer => layer.pointer === parent)?.id ?? "";
    }

    property string target
    readonly property alias menu: menu
    readonly property alias list: list
    readonly property alias tools: tools

    readonly property var groups: current ? layers.filter(layer => layer.type === "group" && layer.pointer !== current.pointer && !layer.pointer.startsWith(current.pointer + "/")) : []

    function pick(layer, add) {
        if (!add)
            page.picked = [];
        else if (page.picked.includes(layer.id))
            page.picked = page.picked.filter(id => id !== layer.id);
        else
            page.picked = (page.picked.length > 0 || !current ? page.picked : [current.id]).concat([layer.id]);
        page.editor.select(layer.pointer);
        SceneFocus.canvasWanted();
    }

    function reach(at) {
        const layer = shown[Math.max(0, Math.min(shown.length - 1, at))];
        if (!layer)
            return;
        page.picked = [];
        page.editor.select(layer.pointer);
        list.positionViewAtIndex(shown.indexOf(layer), ListView.Contain);
    }

    function move(to, into) {
        const args = {
            "id": current.id,
            "to": to
        };
        if (into)
            args.into = into;
        page.run("move_layer", args);
    }

    function drop(layer, onto) {
        target = "";
        if (!onto || onto.pointer === layer.pointer || onto.pointer.startsWith(layer.pointer + "/"))
            return;
        const parent = onto.pointer.replace(/\/layers\/\d+$/, "");
        const args = {
            "id": layer.id,
            "to": Number(onto.pointer.split("/").pop())
        };
        const group = layers.find(other => other.pointer === parent)?.id;
        if (group)
            args.into = group;
        page.run("move_layer", args);
    }

    function add() {
        tools.add();
    }

    function opensMenu() {
        let item = Window.activeFocusItem;
        while (item && typeof item.opensMenu !== "function")
            item = item.parent;
        if (item && item !== panel)
            return item.opensMenu();
        const at = shown.indexOf(current);
        if (at < 0 || !list.visible)
            return;
        list.positionViewAtIndex(at, ListView.Contain);
        const row = list.itemAtIndex(at);
        if (row)
            row.opensMenu();
    }

    function icon(type) {
        return ({
                "image": "image-x-generic",
                "video": "video-x-generic",
                "text": "draw-text",
                "shape": "draw-rectangle",
                "solid": "fill-color",
                "spectrum": "view-statistics",
                "particles": "weather-snow",
                "sound": "audio-x-generic",
                "group": "folder",
                "adjustment": "color-management",
                "light": "help-hint",
                "model": "draw-cuboid",
                "group3d": "folder-3d",
                "shapes": "draw-polyline"
            })[type] ?? "layer-visible-on";
    }

    Shortcut {
        sequences: ["Menu", "Shift+F10"]
        enabled: panel.page.visible && !menu.opened
        onActivated: panel.opensMenu()
    }

    SceneLayerMenu {
        id: menu

        visible: false
        owner: panel
        tools: tools
    }

    SceneLayerTools {
        id: tools

        Layout.fillWidth: true
        Layout.margins: Look.e3
        owner: panel
    }

    QQC2.Label {
        Layout.fillWidth: true
        Layout.margins: Look.e3
        visible: (panel.page.view.layers ?? []).length === 0
        wrapMode: Text.WordWrap
        color: Look.mistLow
        font.pixelSize: Look.tFoot
        text: App.tr("There are no layers yet. The first one you add stays at the back; the next ones go on top.")
    }

    QQC2.ScrollView {
        Layout.fillWidth: true
        Layout.fillHeight: true
        background: Rectangle {
            color: "transparent"
            border.color: Kirigami.Theme.focusColor
            border.width: list.activeFocus && list.currentIndex < 0 ? 2 : 0
        }

        ListView {
            id: list

            model: panel.shown
            clip: true
            readonly property bool focusList: true
            activeFocusOnTab: true
            currentIndex: panel.shown.indexOf(panel.current)
            Accessible.role: Accessible.List
            Accessible.name: App.tr("Layers")
            Keys.onShortcutOverride: event => event.accepted = [Qt.Key_Delete, Qt.Key_Home, Qt.Key_End].includes(event.key)
            Keys.onUpPressed: panel.reach(currentIndex - 1)
            Keys.onDownPressed: panel.reach(currentIndex + 1)
            Keys.onPressed: event => {
                event.accepted = [Qt.Key_Home, Qt.Key_End, Qt.Key_Delete].includes(event.key);
                if (event.key === Qt.Key_Home)
                    panel.reach(0);
                else if (event.key === Qt.Key_End)
                    panel.reach(panel.shown.length - 1);
                else if (event.key === Qt.Key_Delete && panel.current)
                    menu.remove();
            }

            delegate: SceneLayerRow {
                owner: panel
            }
        }
    }
}
