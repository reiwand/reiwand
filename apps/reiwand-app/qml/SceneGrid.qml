import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

// Two columns of names and their controls, in a panel of any width: the names wrap in a column of
// at most the workbench's name width (less in a narrow panel), so a long name never pushes its
// control past the panel's edge.
GridLayout {
    id: grid

    function fitNames() {
        for (const item of children) {
            // The names are the words of the grid: anything with text that wraps and is not pressed.
            if (item.wrapMode === undefined || item.text === undefined || item.pressed !== undefined)
                continue;
            item.wrapMode = Text.Wrap;
            item.Layout.fillWidth = true;
            item.Layout.minimumWidth = Look.e7;
            item.Layout.preferredWidth = DeskLook.label;
            item.Layout.maximumWidth = DeskLook.label * 1.5;
        }
    }

    columns: 2
    Component.onCompleted: fitNames()
    onChildrenChanged: Qt.callLater(fitNames)
}
