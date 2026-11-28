import QtQuick
import QtQml
import io.github.reiwand.app

// The workbench of a package that is not a scene (web, image, video, preset): the same groups and
// layouts as the scene editor, in the Wallpaper layout, with the panels such a package has. The
// panels of a scene (layers, effects, timeline…) are left out of its tree.
Item {
    id: area

    required property var page
    readonly property alias layout: layout
    readonly property alias drop: drop
    readonly property var owned: ["canvas", "assets", "try", "properties", "inputs", "problems", "sheet", "texts", "cover", "review"].concat(page.kind === "web" || page.kind === "video" ? ["options"] : [], page.kind === "web" ? ["console"] : [])
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
    readonly property alias keys: benchKeys
    property int serial: 0

    function title(unit) {
        const count = SceneScriptFault.count(page);
        return {
            "canvas": App.tr("See as"),
            "assets": App.tr("Files"),
            "options": page.kind === "video" ? App.tr("Presets") : App.tr("Options"),
            "try": App.tr("Try"),
            "properties": App.tr("Inspector"),
            "inputs": App.tr("Player"),
            "problems": count ? App.tr("Problems (%1)", count) : App.tr("Problems"),
            "sheet": App.tr("Wallpaper sheet"),
            "texts": App.tr("Texts and languages"),
            "cover": App.tr("How it will look"),
            "review": App.tr("Review"),
            "console": App.tr("Console")
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

    function refold() {
    }

    function focusTab(unit) {
        groupItem(unit)?.focusTab(unit);
    }

    // The package is read after the page is made: the layout is chosen once its kind is known. A
    // package that is not a scene always opens in Wallpaper (approved decision 3); the author moves
    // to another layout while it is open. Each kind keeps its own arrangement of every layout (and
    // the layouts the author saves there), in a file apart from the scene's.
    readonly property string kind: page.kind

    function opened() {
        if (!kind)
            return;
        if (layout.desks.place !== kind) {
            layout.flush();
            layout.desks.place = kind;
            layout.restore();
        }
        layout.opening(kind, false);
    }

    onKindChanged: opened()
    Component.onCompleted: opened()

    SceneLayout {
        id: layout

        width: area.width
        height: area.height
        absent: units.filter(unit => !area.owned.includes(unit))
    }

    Repeater {
        id: groups

        model: layout.groups.length

        delegate: SceneGroup {
            area: docks
            layout: docks.layout
        }

        onItemAdded: area.serial++
        onItemRemoved: area.serial++
    }

    PackageDockPanels {
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
