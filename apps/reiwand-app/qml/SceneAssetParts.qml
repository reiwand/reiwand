import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

ColumnLayout {
    id: parts

    required property var page

    SceneFold {
        Layout.fillWidth: true
        key: "effects"
        visible: effects.count > 0
        title: App.tr("Effects")

        SceneEffectsList {
            id: effects

            Layout.fillWidth: true
            page: parts.page
        }
    }

    SceneFold {
        Layout.fillWidth: true
        key: "pools"
        title: App.tr("Shape pools")

        ScenePoolsPanel {
            Layout.fillWidth: true
            page: parts.page
        }
    }
}
