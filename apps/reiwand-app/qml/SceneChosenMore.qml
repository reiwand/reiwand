import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

QQC2.ToolButton {
    id: more

    required property var bar
    readonly property var page: bar.page
    readonly property alias window: window

    icon.name: "dots-three"
    text: App.tr("More")
    checked: window.visible
    onClicked: window.visible ? window.close() : window.open()

    QQC2.Popup {
        id: window

        y: more.height
        width: Look.menuChico
        padding: Look.e2
        focus: true
        closePolicy: QQC2.Popup.CloseOnEscape | QQC2.Popup.CloseOnPressOutsideParent
        onOpened: menu.move(0)
        onClosed: more.forceActiveFocus()

        contentItem: ColumnLayout {
            id: menu

            function move(by) {
                const items = Array.from(menu.children).filter(item => item.visible && item.enabled && item.activeFocusOnTab);
                const at = items.findIndex(item => item.activeFocus);
                items[Math.max(0, Math.min(items.length - 1, at + by))]?.forceActiveFocus(Qt.TabFocusReason);
            }

            spacing: Look.e1
            Keys.onUpPressed: move(-1)
            Keys.onDownPressed: move(1)
            Keys.onReturnPressed: Array.from(menu.children).find(item => item.activeFocus)?.click()
            Keys.onEnterPressed: Array.from(menu.children).find(item => item.activeFocus)?.click()

            QQC2.Switch {
                Layout.fillWidth: true
                text: App.tr("Magnets")
                checked: more.page.magnets
                onToggled: more.page.magnets = checked
                QQC2.ToolTip.text: App.tr("A dragged layer snaps to the edges and the middle of the canvas and of the other layers; hold Alt to place it freely, use the arrow keys (Shift for 10 px) to nudge it")
                QQC2.ToolTip.visible: hovered
            }

            SceneOverwrite {
                Layout.fillWidth: true
                page: more.page
                display: QQC2.AbstractButton.TextBesideIcon
            }

            QQC2.ToolButton {
                Layout.fillWidth: true
                visible: more.bar.chosen !== null
                icon.name: "link"
                text: App.tr("Follow a layer")
                enabled: more.bar.free
                checkable: true
                checked: more.page.picking === "parent"
                onToggled: {
                    more.page.picking = checked ? "parent" : "";
                    window.close();
                }
                QQC2.ToolTip.text: App.tr("Pick on the canvas the layer that the selected one follows, keeping it where it is")
                QQC2.ToolTip.visible: hovered
            }

            QQC2.ToolButton {
                Layout.fillWidth: true
                visible: more.bar.chosen !== null && !!more.bar.chosen.parent
                icon.name: "link-break"
                text: App.tr("Stop following")
                enabled: more.bar.free
                onClicked: more.page.run("layer", {
                        "parent": {
                            "layer": more.bar.chosen.id,
                            "keep": true,
                            "at": more.page.editor.time,
                            "replace": more.page.overwrite
                        }
                    })
            }

            SceneCameraTool {
                Layout.fillWidth: true
                page: more.page
                display: QQC2.AbstractButton.TextBesideIcon
            }

            SceneProbeTool {
                Layout.fillWidth: true
                page: more.page
                display: QQC2.AbstractButton.TextBesideIcon
            }

            QQC2.ToolButton {
                Layout.fillWidth: true
                icon.name: "sidebar-simple"
                text: App.tr("Reset the view")
                onClicked: {
                    more.page.layout.reset();
                    window.close();
                }
                QQC2.ToolTip.text: App.tr("Put every panel back on its first side and at its default size")
                QQC2.ToolTip.visible: hovered
            }
        }
    }
}
