import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

ColumnLayout {
    id: panel

    required property var page

    function pick(level) {
        SceneLook.quality = level;
        SceneLook.preview(panel.page);
    }

    spacing: Look.e1

    component Head: QQC2.Label {
        Layout.fillWidth: true
        Layout.topMargin: Look.e1
        font.weight: Font.DemiBold
        wrapMode: Text.Wrap
    }

    Head {
        text: App.tr("Device")
    }

    Repeater {
        model: SceneLook.levels

        delegate: QQC2.RadioButton {
            required property int index
            required property string modelData

            Layout.fillWidth: true
            text: SceneLook.names[index]
            checked: SceneLook.quality === modelData
            Accessible.name: text + ", " + modelData.toUpperCase()
            onClicked: panel.pick(modelData)
        }
    }

    Head {
        text: App.tr("How its user would see it")
    }

    QQC2.Switch {
        Layout.fillWidth: true
        icon.name: "battery-medium"
        text: App.tr("On battery")
        checked: SceneLook.battery
        onToggled: {
            SceneLook.battery = checked;
            SceneLook.preview(panel.page);
        }
    }

    QQC2.Switch {
        Layout.fillWidth: true
        icon.name: "person-simple-walk"
        text: App.tr("Reduced motion")
        checked: SceneLook.calm
        onToggled: {
            SceneLook.calm = checked;
            SceneLook.preview(panel.page);
        }
    }

    ScenePreviewLanguage {
        Layout.fillWidth: true
        page: panel.page
    }

    Head {
        text: App.tr("Compare")
    }

    ScenePreviewCompare {
        Layout.fillWidth: true
        page: panel.page
    }

    Rectangle {
        Layout.fillWidth: true
        // Clear of the focus ring of the button above.
        Layout.topMargin: Look.ringRoom
        implicitHeight: Look.line
        color: Look.nightLine
    }

    QQC2.ToolButton {
        Layout.fillWidth: true
        icon.name: "arrows-clockwise"
        text: App.tr("Restart the preview")
        onClicked: SceneLook.again(panel.page)
    }

    SceneFold {
        Layout.fillWidth: true
        key: "see-as-effects"
        title: App.tr("Effects on this computer")
        open: false

        ScenePreviewEffects {
            Layout.fillWidth: true
            page: panel.page
        }
    }
}
