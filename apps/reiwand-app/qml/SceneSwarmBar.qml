import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// In the canvas bar, while a particle system or a particles layer is chosen: which of its handles
// the canvas draws, each a word that turns them on and off.
RowLayout {
    id: handles

    required property var page
    readonly property string pointer: page.editor.selected
    readonly property bool swarm: /^\/scene\/particles\//.test(pointer) || (page.view.layers ?? []).some(item => item.pointer === pointer && item.type === "particles")
    readonly property var words: ({
            "emitters": [App.tr("Emitter handles"), "selection"],
            "forces": [App.tr("Force arrows"), "wind"],
            "points": [App.tr("Control points"), "dot-outline"],
            "colliders": [App.tr("Colliders"), "polygon"]
        })

    visible: swarm
    spacing: Look.e1

    Repeater {
        model: SceneSwarmShow.kinds

        delegate: QQC2.ToolButton {
            required property string modelData

            checkable: true
            checked: SceneSwarmShow.shown(modelData)
            icon.name: handles.words[modelData][1]
            text: handles.words[modelData][0]
            display: (handles.parent?.width ?? 0) < Look.smallPanel * 2.5 ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
            QQC2.ToolTip.text: checked ? App.tr("Hide the %1 on the canvas", text.toLowerCase()) : App.tr("Show the %1 on the canvas", text.toLowerCase())
            QQC2.ToolTip.visible: hovered
            onClicked: SceneSwarmShow.flip(modelData)
        }
    }
}
