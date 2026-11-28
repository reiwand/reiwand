import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

QQC2.ToolButton {
    id: button

    required property var page
    property var sources: []
    property real now: 0
    readonly property alias graph: graph

    icon.name: "labplot-xy-curve"
    text: App.tr("Curve editor")
    display: QQC2.AbstractButton.IconOnly
    enabled: sources.length > 0
    // With the Curves panel on the workbench the button brings it into view; without it, the curve
    // editor opens as a dialog.
    onClicked: {
        const layout = page.layout;
        if (layout?.groupOf("curves")) {
            layout.reveal("curves");
            Qt.callLater(page.docks.focusTab, "curves");
        } else {
            graph.open();
        }
    }
    QQC2.ToolTip.visible: hovered
    QQC2.ToolTip.text: App.tr("Curve editor: the curves of the values part by part, with handles, several keys at once, copy and paste")

    SceneGraph {
        id: graph

        parent: QQC2.Overlay.overlay
        page: button.page
        sources: button.sources
        now: button.now
    }
}
