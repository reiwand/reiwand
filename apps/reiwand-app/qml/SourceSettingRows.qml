import QtQuick
import io.github.reiwand.app

// the settings of a provider plugin, prefilled with its own values, when a source of it is added
Repeater {
    id: rows

    required property var options
    required property var values

    signal changed(var next)

    function set(key, text) {
        changed(Object.assign({}, values, {
            [key]: text
        }));
    }

    model: options

    delegate: ListRow {
        required property var modelData
        readonly property string given: rows.values[modelData.key] ?? String(modelData.default ?? "")

        text: modelData.label ?? SourceKinds.setting(modelData.key)
        kind: modelData.kind === "bool" ? "switch" : "field"
        checked: given === "true"
        fieldText: given
        onToggled: rows.set(modelData.key, checked ? "true" : "false")
        onEdited: text => rows.set(modelData.key, text)
    }
}
