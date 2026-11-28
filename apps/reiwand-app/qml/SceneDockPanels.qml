import QtQuick
import io.github.reiwand.app

// The panels of the workbench, each a unit that goes to the group holding it: the ones other parts reach
// are made at once, the others the first time they show.
Item {
    id: panels

    required property var area
    readonly property alias stage: stage
    readonly property alias layers: layers
    readonly property alias assets: assets
    readonly property alias settings: settings
    readonly property alias effects: effects
    readonly property alias foot: foot
    readonly property alias curves: curves

    SceneUnit {
        area: panels.area
        unit: "canvas"

        SceneStage {
            id: stage

            anchors.fill: parent
            page: panels.area.page
        }
    }

    SceneUnit {
        area: panels.area
        unit: "layers"

        SceneLayers {
            id: layers

            anchors.fill: parent
            page: panels.area.page
        }
    }

    SceneUnit {
        area: panels.area
        unit: "assets"

        SceneAssets {
            id: assets

            anchors.fill: parent
            page: panels.area.page
        }
    }

    SceneUnit {
        area: panels.area
        unit: "options"

        SceneProperties {
            id: settings

            anchors.fill: parent
            page: panels.area.page
        }
    }

    SceneLazyUnit {
        area: panels.area
        unit: "try"

        SceneTrial {
            anchors.fill: parent
            page: panels.area.page
        }
    }

    SceneLazyUnit {
        area: panels.area
        unit: "properties"

        SceneFields {
            anchors.fill: parent
            page: panels.area.page
        }
    }

    SceneLazyUnit {
        area: panels.area
        unit: "particles"

        SceneSwarmTree {
            anchors.fill: parent
            page: panels.area.page
        }
    }

    SceneLazyUnit {
        area: panels.area
        unit: "cost"

        SceneCost {
            anchors.fill: parent
            page: panels.area.page
        }
    }

    SceneUnit {
        area: panels.area
        unit: "effects"

        SceneEffectsPanel {
            id: effects

            anchors.fill: parent
            page: panels.area.page
        }
    }

    SceneUnit {
        area: panels.area
        unit: "curves"

        SceneCurves {
            id: curves

            anchors.fill: parent
            page: panels.area.page
            times: foot.times
            focused: panels.area.focusUnit === "curves"
        }
    }

    SceneLazyUnit {
        area: panels.area
        unit: "problems"

        SceneProblems {
            anchors.fill: parent
            anchors.margins: Look.e3
            page: panels.area.page
        }
    }

    SceneUnit {
        area: panels.area
        unit: "timeline"

        SceneFoot {
            id: foot

            anchors.fill: parent
            page: panels.area.page
            onFoldedChanged: Qt.callLater(panels.area.refold)
            onWantedChanged: Qt.callLater(panels.area.refold)
        }
    }

    SceneLazyUnit {
        area: panels.area
        unit: "viewas"

        SceneLookPanel {
            anchors.fill: parent
            page: panels.area.page
        }
    }

    SceneLazyUnit {
        area: panels.area
        unit: "inputs"

        SceneInputs {
            anchors.fill: parent
            page: panels.area.page
        }
    }

    SceneLazyUnit {
        area: panels.area
        unit: "screens"

        SceneScreensPanel {
            anchors.fill: parent
            page: panels.area.page
        }
    }
}
