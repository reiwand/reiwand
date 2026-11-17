import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Kirigami.FormLayout {
    id: look

    required property var page
    required property var node
    readonly property var modes: page.fields.find(field => field.key === "blend")?.options ?? []
    readonly property string mode: page.fields.find(field => field.key === "blend")?.value || "normal"

    readonly property var spaces: page.fields.find(field => field.key === "blend_space")?.options ?? []
    readonly property string space: page.fields.find(field => field.key === "blend_space")?.value || "canvas"

    visible: node !== null && modes.length > 0 && !["sound", "light"].includes(node.type)

    QQC2.ComboBox {
        Kirigami.FormData.label: App.tr("Blend mode:")
        model: look.modes.map(mode => SceneWords.value(mode))
        currentIndex: Math.max(0, look.modes.indexOf(look.mode))
        onActivated: index => look.page.run("layer", {
                "blend": {
                    "layer": look.node.id,
                    "blend": look.modes[index]
                }
            })
    }

    QQC2.ComboBox {
        Kirigami.FormData.label: App.tr("Blend space:")
        model: look.spaces.map(space => SceneWords.value(space))
        currentIndex: Math.max(0, look.spaces.indexOf(look.space))
        onActivated: index => look.page.run("layer", {
                "blend_space": {
                    "layer": look.node.id,
                    "space": look.spaces[index]
                }
            })
    }
}
