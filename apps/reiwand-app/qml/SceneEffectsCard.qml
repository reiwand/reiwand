import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

SceneFold {
    id: card

    required property var page
    required property var node
    property var rows: []
    readonly property alias stack: stack

    key: "layer-effects"
    title: stack.list.length > 0 ? App.tr("Effects · %1", stack.list.length) : App.tr("Effects")

    SceneEffectStack {
        id: stack

        Layout.fillWidth: true
        page: card.page
        node: card.node
        rows: card.rows
    }
}
