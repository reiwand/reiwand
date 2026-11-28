import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

QQC2.Menu {
    id: menu

    required property var area
    required property string unit
    readonly property var layout: area.layout

    readonly property var others: visible ? layout.groups.filter(group => !group.panels.includes(unit) && !group.panels.includes("canvas")) : []

    function stack(beside) {
        if (layout.stack(unit, beside))
            Qt.callLater(area.focusTab, unit);
    }

    function shut() {
        const left = layout.groupOf(unit)?.panels.find(other => other !== unit) ?? "";
        if (layout.close(unit) && left)
            Qt.callLater(area.focusTab, left);
    }

    function go(side) {
        if (layout.move(unit, side))
            Qt.callLater(area.focusTab, unit);
    }

    QQC2.MenuItem {
        text: App.tr("Move left")
        enabled: menu.visible && menu.layout.canMove(menu.unit, "left")
        onTriggered: Qt.callLater(menu.go, "left")
    }

    QQC2.MenuItem {
        text: App.tr("Move right")
        enabled: menu.visible && menu.layout.canMove(menu.unit, "right")
        onTriggered: Qt.callLater(menu.go, "right")
    }

    QQC2.MenuItem {
        text: App.tr("Move up")
        enabled: menu.visible && menu.layout.canMove(menu.unit, "up")
        onTriggered: Qt.callLater(menu.go, "up")
    }

    QQC2.MenuItem {
        text: App.tr("Move down")
        enabled: menu.visible && menu.layout.canMove(menu.unit, "down")
        onTriggered: Qt.callLater(menu.go, "down")
    }

    QQC2.Menu {
        id: stacking

        title: App.tr("Stack with…")
        enabled: menu.others.length > 0 && menu.unit !== "canvas"

        Instantiator {
            model: menu.others
            onObjectAdded: (index, object) => stacking.insertItem(index, object)
            onObjectRemoved: (index, object) => stacking.removeItem(object)

            delegate: QQC2.MenuItem {
                required property var modelData

                // Its menu goes first when the tab leaves the workbench (detached, closed).
                text: menu ? modelData.panels.map(unit => menu.area.title(unit)).join(", ") : ""
                onTriggered: Qt.callLater(menu.stack, modelData.shown)
            }
        }
    }

    QQC2.MenuSeparator {}

    QQC2.MenuItem {
        text: App.tr("Detach into a window")
        enabled: menu.visible
        onTriggered: Qt.callLater(() => menu.layout.view.detach(menu.unit))
    }

    QQC2.MenuItem {
        readonly property var group: menu.visible ? menu.layout.groupOf(menu.unit) : null

        text: App.tr("Detach the whole group")
        visible: (group?.panels.length ?? 0) > 1
        height: visible ? implicitHeight : 0
        enabled: menu.visible && !(group?.panels.includes("canvas") ?? true)
        onTriggered: {
            const panels = group.panels;
            Qt.callLater(() => menu.layout.view.detachAll(panels));
        }
    }

    QQC2.MenuItem {
        text: menu.layout.view.maximised === menu.unit ? App.tr("Back to the workbench (F11)") : App.tr("Maximize (F11)")
        onTriggered: Qt.callLater(() => menu.layout.view.maximise(menu.unit))
    }

    QQC2.MenuItem {
        text: App.tr("Close the panel")
        enabled: menu.visible && menu.layout.canClose(menu.unit)
        onTriggered: Qt.callLater(menu.shut)
    }
}
