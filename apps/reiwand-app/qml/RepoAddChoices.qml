import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

FocusList {
    id: choices

    property var entries: []

    signal chosen(var value)

    function pick() {
        if (currentIndex >= 0 && currentIndex < entries.length)
            chosen(entries[currentIndex].value);
    }

    clip: true
    height: Math.min(contentHeight, Look.e6 * 5)
    model: entries
    Accessible.role: Accessible.List
    Keys.onReturnPressed: pick()
    Keys.onEnterPressed: pick()

    delegate: QQC2.ItemDelegate {
        required property var modelData
        required property int index

        width: ListView.view.width
        text: modelData.label
        icon.name: modelData.icon ?? ""
        onClicked: {
            choices.currentIndex = index;
            choices.pick();
        }
    }
}
