import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Column {
    id: choose

    readonly property alias list: list
    readonly property alias join: join
    readonly property alias both: both
    readonly property alias rows: rows
    property string mode: "join"
    readonly property bool conflicting: {
        for (let i = 0; i < rows.count; i++) {
            const row = rows.get(i);
            if (row.section === "lists" && row.checked && row.conflicts > 0)
                return true;
        }
        return false;
    }
    readonly property var taken: {
        const found = [];
        for (let i = 0; i < rows.count; i++) {
            if (rows.get(i).checked)
                found.push(rows.get(i).section);
        }
        return found;
    }

    signal accepted

    function fill(entries) {
        rows.clear();
        entries.forEach(entry => rows.append(entry));
        list.currentIndex = 0;
    }

    function toggle() {
        if (list.currentIndex >= 0)
            rows.setProperty(list.currentIndex, "checked", !rows.get(list.currentIndex).checked);
    }

    spacing: Look.e3

    ListModel {
        id: rows
    }

    FocusList {
        id: list

        width: parent.width
        height: Math.min(contentHeight, Look.e6 * 8)
        clip: true
        model: rows
        Accessible.role: Accessible.List
        Keys.onSpacePressed: choose.toggle()
        Keys.onReturnPressed: choose.accepted()
        Keys.onEnterPressed: choose.accepted()

        delegate: QQC2.CheckDelegate {
            required property var model
            required property int index

            width: ListView.view.width
            checked: model.checked
            text: model.label + ", " + model.detail
            onToggled: rows.setProperty(index, "checked", checked)

            contentItem: Column {
                Text {
                    width: parent.width
                    text: model.label
                    elide: Text.ElideRight
                    font.family: Look.textFont
                    font.pixelSize: Look.tBody
                    color: Look.frost
                }

                Text {
                    width: parent.width
                    text: model.detail
                    elide: Text.ElideRight
                    font.family: Look.textFont
                    font.pixelSize: Look.tFoot
                    color: Look.mist
                }
            }
        }
    }

    Column {
        visible: choose.conflicting
        width: parent.width

        Text {
            width: parent.width
            text: App.tr("Some lists have the same name as one of yours:")
            wrapMode: Text.WordWrap
            font.family: Look.textFont
            font.pixelSize: Look.tBody
            color: Look.frost
        }

        QQC2.RadioButton {
            id: join

            checked: choose.mode === "join"
            text: App.tr("Join them: add to your list what it lacks")
            onToggled: choose.mode = "join"
        }

        QQC2.RadioButton {
            id: both

            checked: choose.mode === "both"
            text: App.tr("Keep both: the imported one is told apart by its name")
            onToggled: choose.mode = "both"
        }
    }
}
