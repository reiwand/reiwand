import QtQuick
import io.github.reiwand.app

Column {
    id: nowLayers

    property var info: ({})
    property var jobs: Layers.jobs
    readonly property alias layers: layers
    readonly property alias bar: bar
    readonly property bool poster: info.quality_auto === "still"
    readonly property var layering: Layers.onScreen(info, jobs)
    readonly property string note: Layers.note(layering)

    topPadding: Look.e3
    spacing: Look.e1
    visible: poster || note !== ""

    StatusLine {
        anchors.right: parent.right
        visible: nowLayers.poster
        tone: "amber"
        text: App.tr("Still at its poster")
    }

    StatusLine {
        id: layers

        anchors.right: parent.right
        visible: nowLayers.note !== ""
        tone: Layers.tone(nowLayers.layering)
        icon: Layers.icon(nowLayers.layering)
        text: nowLayers.note
    }

    ThinProgress {
        id: bar

        anchors.right: parent.right
        width: layers.width
        value: nowLayers.note !== "" ? Layers.bar(nowLayers.layering) : -1
    }
}
