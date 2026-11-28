import QtQuick
import io.github.reiwand.app

Item {
    id: panels

    required property var area
    readonly property alias code: code

    SceneLazyUnit {
        area: panels.area
        unit: "scripts"

        SceneScriptsPanel {
            page: panels.area.page
            onEdit: path => code.openScript(path)
        }
    }

    SceneUnit {
        area: panels.area
        unit: "code"

        SceneCodePanel {
            id: code

            anchors.fill: parent
            page: panels.area.page
        }
    }

    SceneLazyUnit {
        area: panels.area
        unit: "reference"

        SceneApiPanel {
            page: panels.area.page
            onInserted: text => code.insert(text)
        }
    }

    SceneLazyUnit {
        area: panels.area
        unit: "console"

        SceneConsolePanel {
            page: panels.area.page
        }
    }
}
