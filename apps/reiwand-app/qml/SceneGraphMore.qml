import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The "⋯" of the curve editor's row in a narrow panel: the tools that left the row.
QQC2.ToolButton {
    id: more

    required property var tools

    visible: more.tools.graph.compact
    icon.name: "dots-three"
    text: App.tr("More tools of the curve editor")
    display: QQC2.AbstractButton.IconOnly
    QQC2.ToolTip.visible: hovered
    QQC2.ToolTip.text: text
    onClicked: menu.popup(more, 0, more.height)

    QQC2.Menu {
        id: menu

        QQC2.MenuItem {
            text: App.tr("Aligned handles")
            checkable: true
            checked: more.tools.graph.aligned
            onTriggered: more.tools.graph.aligned = checked
        }

        QQC2.MenuItem {
            text: App.tr("Linked parts")
            checkable: true
            checked: more.tools.graph.linked
            onTriggered: {
                more.tools.graph.linked = checked;
                if (checked && more.tools.lone && more.tools.graph.ops.split(more.tools.graph.chosen[0].s, more.tools.graph.chosen[0].t))
                    more.tools.graph.ops.link();
            }
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            text: App.tr("Copy")
            enabled: more.tools.graph.chosen.length > 0
            onTriggered: more.tools.graph.ops.copy()
        }

        QQC2.MenuItem {
            text: App.tr("Paste at the playhead")
            enabled: more.tools.graph.pasteable
            onTriggered: more.tools.graph.ops.paste()
        }

        QQC2.MenuItem {
            text: App.tr("Remove the picked keys")
            enabled: more.tools.graph.chosen.length > 0
            onTriggered: more.tools.graph.ops.act({
                "remove": true
            })
        }

        QQC2.MenuItem {
            text: App.tr("Fit the picked keys (F)")
            enabled: more.tools.graph.chosen.length > 0
            onTriggered: more.tools.graph.plot.view.fitPicked()
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            text: App.tr("Stretch or scale…")
            checkable: true
            checked: more.tools.wide
            onTriggered: more.tools.wide = checked
        }
    }
}
