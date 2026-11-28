import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

// A fixed row of numbers, such as the room of a layer for its effects ([left, top, right, bottom]):
// one field each. While the row is not set the fields start from its sample, and the first change
// sets the whole row; Default in the head of the field clears it again.
RowLayout {
    id: row

    required property var input
    readonly property var shown: Array.isArray(input.value) ? input.value : input.field.sample ?? []

    Repeater {
        model: row.shown.length

        delegate: SceneNumberText {
            required property int index

            Layout.fillWidth: true
            Layout.minimumWidth: Look.e6
            number: Number(row.shown[index])
            Accessible.name: SceneWords.label(row.input.field.key) + " " + (index + 1)
            onChosen: typed => {
                const next = row.shown.slice();
                next[index] = row.input.parse(typed, next[index]);
                row.input.commit(next);
            }
        }
    }
}
