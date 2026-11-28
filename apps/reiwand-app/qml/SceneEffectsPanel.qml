import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The Effects panel: the effect stack of the layer chosen (or of the layer of the effect chosen),
// each effect with its values and its cost per level, the catalog, and the package's own effects.
Rectangle {
    id: effects

    required property var page
    readonly property string pointer: page.editor.selected
    readonly property string home: pointer.match(/^\/scene(\/layers\/\d+)+/)?.[0] ?? ""
    readonly property var node: (page.view.layers ?? []).find(item => item.pointer === home) ?? null
    readonly property var rows: {
        effects.page.fields;
        return node ? JSON.parse(page.editor.inspect(home) || "[]") : [];
    }
    readonly property alias stack: stack

    // Choosing an effect anywhere (in Layers, on the canvas) opens it in the stack.
    function follow() {
        const found = pointer.slice(home.length).match(/^\/effects\/(\d+)/);
        stack.chosen = found ? Number(found[1]) : -1;
    }

    onPointerChanged: Qt.callLater(follow)
    Component.onCompleted: follow()

    color: Look.night
    clip: true

    QQC2.ScrollView {
        id: scroll

        anchors.fill: parent
        anchors.leftMargin: Look.e3
        anchors.rightMargin: Look.e2
        anchors.topMargin: Look.e2
        contentWidth: availableWidth

        ScrollColumn {
            view: scroll

            QQC2.Label {
                Layout.fillWidth: true
                visible: effects.node !== null
                text: App.tr("Of “%1”", effects.node?.id ?? "")
                color: Look.mist
                font.pixelSize: Look.tFoot
                elide: Text.ElideRight
            }

            SceneEffectStack {
                id: stack

                Layout.fillWidth: true
                visible: effects.node !== null && effects.node.type !== "sound"
                page: effects.page
                node: effects.node ?? {}
                rows: effects.rows
                costed: true
            }

            QQC2.Label {
                Layout.fillWidth: true
                visible: effects.node === null || effects.node.type === "sound"
                wrapMode: Text.Wrap
                color: Look.mist
                text: effects.node === null ? App.tr("Choose a layer to see its effects.") : App.tr("A sound layer has no effects.")
            }

            SceneFold {
                Layout.fillWidth: true
                Layout.topMargin: Look.e2
                key: "effects-package"
                title: own.count > 0 ? App.tr("Package effects · %1", own.count) : App.tr("Package effects")
                open: false

                SceneEffectsList {
                    id: own

                    Layout.fillWidth: true
                    page: effects.page
                }
            }
        }
    }
}
