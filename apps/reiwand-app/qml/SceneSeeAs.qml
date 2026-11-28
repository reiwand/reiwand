import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

QQC2.ToolButton {
    id: seeAs

    required property var page
    readonly property alias window: window

    icon.name: "caret-down"
    text: App.tr("See as: %1", SceneLook.names[Math.max(0, SceneLook.levels.indexOf(SceneLook.quality))])
    checked: window.visible
    QQC2.ToolTip.text: App.tr("Show the preview as the daemon draws it at a quality level, on battery or with reduced motion, and compare two levels")
    QQC2.ToolTip.visible: hovered && !window.visible
    // With the See as panel on the workbench the button brings it into view; without it, the
    // panel opens under the button.
    onClicked: {
        const layout = page.layout;
        if (layout?.groupOf("viewas")) {
            layout.reveal("viewas");
            Qt.callLater(page.docks.focusTab, "viewas");
        } else if (window.visible) {
            window.close();
        } else {
            window.open();
        }
    }

    QQC2.Popup {
        id: window

        x: seeAs.width - width
        y: seeAs.height
        width: Look.smallPanel
        padding: Look.e3
        focus: true
        closePolicy: QQC2.Popup.CloseOnEscape | QQC2.Popup.CloseOnPressOutsideParent
        onClosed: seeAs.forceActiveFocus()

        contentItem: ScenePreviewPanel {
            page: seeAs.page
        }
    }
}
