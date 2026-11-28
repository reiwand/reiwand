import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The blend mode and the opacity of the chosen layer at the head of Layers, as the inspector has
// them, to change while looking at the stack.
RowLayout {
    id: blend

    required property var owner
    readonly property var page: owner.page
    readonly property var node: owner.current
    readonly property var modeField: page.fields.find(field => field.key === "blend") ?? null
    readonly property var modes: modeField?.options ?? []
    readonly property var alpha: page.fields.find(field => field.key === "opacity" && field.depth === 0) ?? null

    visible: node !== null && node.pointer === page.editor.selected && modes.length > 0 && alpha !== null && !["sound", "light"].includes(node.type)
    spacing: Look.e2

    QQC2.ComboBox {
        Layout.fillWidth: true
        Layout.minimumWidth: 0
        Accessible.name: App.tr("Blend mode:")
        model: blend.modes.map(mode => SceneWords.value(mode))
        currentIndex: Math.max(0, blend.modes.indexOf(blend.modeField?.value || "normal"))
        onActivated: index => blend.page.run("layer", {
                "blend": {
                    "layer": blend.node.id,
                    "blend": blend.modes[index]
                }
            })
    }

    QQC2.TextField {
        id: amount

        property bool typed: false

        Layout.preferredWidth: Look.e8 + Look.e2
        Accessible.name: App.tr("Opacity")
        text: typeof blend.alpha?.shown === "number" ? Math.round(blend.alpha.shown * 100) + " %" : ""
        onTextEdited: typed = true
        onEditingFinished: {
            const value = Number(String(text).replace("%", "").replace(",", ".").trim());
            if (typed && Number.isFinite(value))
                blend.page.change(blend.alpha.pointer, Math.max(0, Math.min(100, value)) / 100);
            typed = false;
        }
    }
}
