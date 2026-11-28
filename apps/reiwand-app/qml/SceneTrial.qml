import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

ColumnLayout {
    id: trial

    required property var page
    property bool player: false
    readonly property bool inputs: !!page.layout?.groupOf("inputs")

    spacing: 0

    SceneTry {
        Layout.fillWidth: true
        Layout.fillHeight: true
        page: trial.page
    }

    // While the workbench has the Inputs panel, the simulated player is there.
    Rectangle {
        Layout.fillWidth: true
        visible: !trial.inputs
        implicitHeight: Look.line
        color: Look.nightLine
    }

    QQC2.ToolButton {
        Layout.fillWidth: true
        visible: !trial.inputs
        text: App.tr("Simulated player")
        icon.name: trial.player ? "caret-down" : "caret-right"
        checkable: true
        checked: trial.player
        onToggled: trial.player = checked
    }

    SceneMedia {
        Layout.fillWidth: true
        Layout.fillHeight: true
        visible: trial.player && !trial.inputs
        page: trial.page
    }
}
