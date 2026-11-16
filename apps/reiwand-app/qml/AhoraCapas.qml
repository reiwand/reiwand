import QtQuick
import io.github.reiwand.app

Column {
    id: capas

    property var info: ({})
    property var jobs: Capas.jobs
    readonly property alias layers: layers
    readonly property alias bar: bar
    readonly property bool poster: info.quality_auto === "still"
    readonly property var layering: Capas.onScreen(info, jobs)
    readonly property string note: Capas.note(layering)

    topPadding: Look.e3
    spacing: Look.e1
    visible: poster || note !== ""

    Estado {
        anchors.right: parent.right
        visible: capas.poster
        tone: "ambar"
        text: App.tr("Still at its poster")
    }

    Estado {
        id: layers

        anchors.right: parent.right
        visible: capas.note !== ""
        tone: Capas.tone(capas.layering)
        icon: Capas.icon(capas.layering)
        text: capas.note
    }

    Barra {
        id: bar

        anchors.right: parent.right
        width: layers.width
        value: capas.note !== "" ? Capas.bar(capas.layering) : -1
    }
}
