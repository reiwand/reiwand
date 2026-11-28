import QtQuick
import QtQml
import io.github.reiwand.app

Item {
    id: area

    required property var page
    readonly property alias layout: layout
    readonly property alias keys: benchKeys
    readonly property alias layers: panels.layers
    readonly property alias settings: panels.settings
    readonly property alias foot: panels.foot
    readonly property alias drop: drop
    readonly property alias stage: panels.stage
    readonly property alias assets: panels.assets
    readonly property alias effects: panels.effects
    readonly property alias curves: panels.curves
    readonly property alias code: scripting.code
    // The panel that had the keyboard last: the Window menu and its keys act on it while the
    // keyboard is in the menus.
    property string lastUnit: ""
    readonly property string focusUnit: {
        for (let at = 0; at < groups.count; at++)
            if (groups.itemAt(at)?.focused)
                return groups.itemAt(at).current;
        return "";
    }
    readonly property Item docks: area
    readonly property alias groups: groups
    property int serial: 0

    function title(unit) {
        const count = SceneScriptFault.count(page);
        return {
            "canvas": App.tr("Canvas"),
            "layers": App.tr("Layers"),
            "assets": App.tr("Assets"),
            "options": App.tr("Options"),
            "try": App.tr("Try"),
            "properties": App.tr("Inspector"),
            "effects": App.tr("Effects"),
            "curves": App.tr("Curves"),
            "viewas": App.tr("See as"),
            "inputs": App.tr("Inputs"),
            "screens": App.tr("Screens"),
            "sheet": App.tr("Wallpaper sheet"),
            "texts": App.tr("Texts and languages"),
            "cover": App.tr("How it will look"),
            "review": App.tr("Review"),
            "scripts": App.tr("Scripts"),
            "code": App.tr("Code"),
            "reference": App.tr("Reference"),
            "console": App.tr("Console"),
            "problems": count ? App.tr("Problems (%1)", count) : App.tr("Problems"),
            "timeline": App.tr("Timeline"),
            "particles": App.tr("Particles"),
            "cost": App.tr("Cost")
        }[unit] ?? unit;
    }

    onFocusUnitChanged: {
        if (focusUnit === "")
            return;
        SceneFocus.group = groupItem(focusUnit);
        lastUnit = focusUnit;
    }

    function groupItem(unit) {
        let found = null;
        for (let at = 0; at < groups.count; at++) {
            const item = groups.itemAt(at);
            if (item?.units.includes(unit) && (!found || found.rail))
                found = item;
        }
        return found;
    }

    function bodyOf(unit, serial) {
        for (let at = 0; at < windows.count; at++)
            if (windows.objectAt(at)?.unit === unit && !windows.objectAt(at).leaving)
                return windows.objectAt(at).body;
        return groupItem(unit)?.body ?? null;
    }

    // The timeline is a footer while its group sits right under the canvas, in the canvas's column.
    function underCanvas() {
        const line = layout.groupOf("timeline");
        const canvas = layout.canvasBox;
        return !!line && Math.abs(line.x - canvas.x) < 1 && Math.abs(line.width - canvas.width) < 1 && line.y > canvas.y;
    }

    function refold() {
        foot.footer = underCanvas();
        const strip = groupItem("timeline")?.strip ?? 0;
        layout.foldedHigh = foot.folded && layout.isShown("timeline") && strip ? strip + foot.wanted : 0;
        layout.openHigh = !foot.folded && strip ? strip + foot.wanted : 0;
    }

    function focusTab(unit) {
        groupItem(unit)?.focusTab(unit);
    }

    Component.onCompleted: {
        if (layout.refused)
            page.notice = layout.held ? layout.refused + " " + App.tr("The editor opens with the usual layouts and does not write over that file until you choose Window › Reset this layout.") : layout.refused;
        layout.opening("scene", (page.starter ?? "") !== "");
    }

    SceneLayout {
        id: layout

        width: area.width
        height: area.height
        onRootChanged: Qt.callLater(area.refold)
    }

    Repeater {
        id: groups

        model: layout.groups.length

        delegate: SceneGroup {
            area: docks
            layout: docks.layout
            onStripChanged: Qt.callLater(docks.refold)
        }

        onItemAdded: {
            area.serial++;
            Qt.callLater(area.refold);
        }
        onItemRemoved: area.serial++
    }

    SceneDockPanels {
        id: panels

        area: area
    }

    SceneDockSheets {
        area: area
    }

    SceneScriptPanels {
        id: scripting

        area: area
    }

    SceneBenchKeys {
        id: benchKeys

        area: area
    }

    Instantiator {
        id: windows

        // By count, so choosing another tab of a window does not open it again.
        model: layout.view.windows.length
        onObjectAdded: area.serial++
        onObjectRemoved: Qt.callLater(() => area.serial++)

        delegate: SceneWindow {
            required property int index

            area: docks
            held: docks.layout.view.windows[index] ?? {
                "units": [],
                "shown": ""
            }
        }
    }

    Repeater {
        model: layout.edges.length

        delegate: SceneGrip {
            layout: docks.layout
        }
    }

    SceneDrop {
        id: drop

        anchors.fill: parent
        area: area
    }
}
