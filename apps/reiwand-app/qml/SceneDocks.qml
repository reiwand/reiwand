import QtQuick
import io.github.reiwand.app

Item {
    id: area

    required property var page
    readonly property alias layout: layout
    readonly property alias layers: layers
    readonly property alias settings: settings
    readonly property alias foot: foot
    readonly property alias drop: drop
    readonly property real middle: width - layout.leftWide - layout.rightWide
    readonly property bool shut: layout.current("bottom") === "timeline" && foot.folded
    readonly property real low: !layout.places.bottom.length ? 0 : shut ? bottom.strip + foot.wanted : layout.footHigh

    function dock(place) {
        return place === "left" ? left : place === "right" ? right : bottom;
    }

    SceneLayout {
        id: layout

        width: area.width
        height: area.height
        leftNeed: left.need
        rightNeed: right.need
    }

    SceneDock {
        id: left

        width: layout.leftWide
        height: parent.height
        area: area
        page: area.page
        layout: layout
        place: "left"
    }

    SceneStage {
        x: layout.leftWide
        width: area.middle
        height: parent.height - area.low
        page: area.page
    }

    SceneDock {
        id: bottom

        x: layout.leftWide
        y: parent.height - height
        width: area.middle
        height: area.low
        area: area
        page: area.page
        layout: layout
        place: "bottom"
    }

    SceneDock {
        id: right

        x: parent.width - width
        width: layout.rightWide
        height: parent.height
        area: area
        page: area.page
        layout: layout
        place: "right"
    }

    SceneUnit {
        area: area
        unit: "layers"

        SceneLayers {
            id: layers

            anchors.fill: parent
            page: area.page
        }
    }

    SceneUnit {
        area: area
        unit: "assets"

        SceneAssets {
            anchors.fill: parent
            page: area.page
        }
    }

    SceneUnit {
        area: area
        unit: "options"

        SceneProperties {
            id: settings

            anchors.fill: parent
            page: area.page
        }
    }

    SceneUnit {
        area: area
        unit: "try"

        SceneTrial {
            anchors.fill: parent
            page: area.page
        }
    }

    SceneUnit {
        area: area
        unit: "properties"

        SceneFields {
            anchors.fill: parent
            page: area.page
        }
    }

    SceneUnit {
        area: area
        unit: "problems"

        SceneProblems {
            anchors.fill: parent
            anchors.margins: Look.e3
            page: area.page
        }
    }

    SceneUnit {
        area: area
        unit: "timeline"

        SceneFoot {
            id: foot

            anchors.fill: parent
            page: area.page
        }
    }

    SceneGrip {
        x: layout.leftWide - width / 2
        height: parent.height
        layout: layout
        place: "left"
    }

    SceneGrip {
        x: parent.width - layout.rightWide - width / 2
        height: parent.height
        layout: layout
        place: "right"
    }

    SceneGrip {
        x: layout.leftWide
        y: parent.height - area.low - height / 2
        width: area.middle
        visible: layout.places.bottom.length > 0 && !area.shut
        layout: layout
        place: "bottom"
    }

    SceneDrop {
        id: drop

        anchors.fill: parent
        area: area
    }
}
