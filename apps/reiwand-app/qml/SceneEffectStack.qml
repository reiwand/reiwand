import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The effects of a layer, in their order: pick one to see its values, add one from the catalog.
// The Effects panel and the inspector's Effects section both draw it.
ColumnLayout {
    id: stack

    required property var page
    required property var node
    property var rows: []
    property int chosen: -1
    readonly property var list: {
        const found = rows.find(field => field.key === "effects" && field.depth === 0);
        return found && Array.isArray(found.shown) ? found.shown : [];
    }
    readonly property string at: node.pointer + "/effects/" + chosen
    readonly property var fields: {
        stack.page.fields;
        return chosen >= 0 && chosen < list.length ? JSON.parse(page.editor.inspect(at) || "[]") : [];
    }
    property bool costed: false

    function addEffect() {
        catalog.open();
    }

    spacing: Look.e1
    onNodeChanged: chosen = -1

    Repeater {
        model: stack.list

        delegate: SceneEffectRow {
            required property int index
            required property var modelData

            Layout.fillWidth: true
            page: stack.page
            effect: modelData
            lit: stack.chosen === index
            onClicked: stack.chosen = stack.chosen === index ? -1 : index
        }
    }

    ColumnLayout {
        Layout.fillWidth: true
        Layout.leftMargin: Look.e3
        visible: stack.fields.length > 0
        spacing: Look.e1

        SceneFieldList {
            Layout.fillWidth: true
            page: stack.page
            rows: stack.fields.filter(field => field.key !== "effect")
        }

        SceneEffectCost {
            Layout.fillWidth: true
            visible: stack.costed
            page: stack.page
            effect: String(stack.fields.find(field => field.key === "effect")?.value ?? "")
            optional: stack.fields.find(field => field.key === "optional")?.value ?? false
        }

        QQC2.ToolButton {
            icon.name: "arrow-square-out"
            text: App.tr("Open the effect")
            onClicked: stack.page.editor.select(stack.at)
        }
    }

    QQC2.ToolButton {
        Layout.fillWidth: true
        icon.name: "plus"
        text: App.tr("Add an effect")
        onClicked: catalog.open()
    }

    SceneEffectCatalog {
        id: catalog

        inspector: ({
                "page": stack.page,
                "addEffect": id => stack.page.run("effects", {
                        "add": {
                            "layer": stack.node.id,
                            "effect": id
                        }
                    })
            })
    }
}
