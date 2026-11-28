import QtQuick
import io.github.reiwand.app

Column {
    id: rules

    required property var playlist
    readonly property var values: playlist.rules ?? {}
    readonly property int count: repeater.count

    readonly property var days: dayNames()

    function dayNames() {
        return {
            "Mon": App.tr("Mon"),
            "Tue": App.tr("Tue"),
            "Wed": App.tr("Wed"),
            "Thu": App.tr("Thu"),
            "Fri": App.tr("Fri"),
            "Sat": App.tr("Sat"),
            "Sun": App.tr("Sun")
        };
    }

    function shown(key) {
        const value = values[key];
        const named = key === "days" && Array.isArray(value) ? value.map(day => days[day] ?? day) : value;
        return Array.isArray(named) ? named.join(",") : named === null || named === undefined ? "" : String(named);
    }

    function read(key, text) {
        if (key !== "days")
            return text.trim();
        const code = word => Object.keys(days).find(day => [day, days[day]].some(name => name.toLowerCase() === word.toLowerCase())) ?? word;
        return text.split(",").map(word => word.trim()).filter(word => word !== "").map(code).join(",");
    }

    function set(key, value) {
        App.act("SetRule", [playlist.uid, key, value], "Playlists1");
    }

    spacing: Look.e2

    Repeater {
        id: repeater

        model: (App.schema.rules ?? []).filter(field => field.key !== "interval")

        ListRow {
            required property var modelData
            readonly property var values: modelData.values ?? []

            width: rules.width
            text: App.field(modelData.key)
            kind: modelData.kind === "bool" ? "switch" : modelData.kind === "enum" ? "choice" : "field"
            checked: rules.values[modelData.key] === true
            options: values.map(value => App.value(value))
            current: Math.max(0, values.indexOf(rules.shown(modelData.key)))
            fieldText: rules.shown(modelData.key)
            placeholder: modelData.kind === "weekdays" ? values.map(day => rules.days[day] ?? day).join(",") : modelData.default === null ? "" : String(modelData.default)
            onToggled: rules.set(modelData.key, String(checked))
            onChosen: index => rules.set(modelData.key, values[index])
            onEdited: text => rules.set(modelData.key, rules.read(modelData.key, text))
        }
    }
}
