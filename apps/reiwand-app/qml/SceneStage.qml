import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

Rectangle {
    id: stage

    required property var page
    readonly property alias canvas: canvas
    readonly property bool empty: page.editor.ready && (page.view.layers ?? []).length === 0

    color: DeskLook.ground

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        SceneNotes {
            id: notes

            Layout.fillWidth: true
            page: stage.page
            desk: true
        }

        SceneCanvas {
            id: canvas

            Layout.fillWidth: true
            Layout.fillHeight: true
            page: stage.page
        }

        // The time of the canvas, at its foot, while the Timeline panel is out of view.
        SceneCanvasTime {
            Layout.fillWidth: true
            visible: !(stage.page.layout?.isShown("timeline") ?? true)
            page: stage.page
        }
    }

    SceneEmpty {
        anchors.fill: parent
        anchors.topMargin: notes.height + Look.canvasBarHeight
        visible: stage.empty
        page: stage.page
    }
}
