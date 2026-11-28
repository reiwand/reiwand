import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

// The head of a group of assets of one kind: a caret, the kind and how many; a click folds or
// opens the group.
QQC2.ItemDelegate {
    id: head

    required property var owner
    required property string section
    readonly property bool open: !owner.folded[section]
    readonly property int count: owner.shown.filter(asset => asset.kind === section).length

    width: ListView.view.width
    implicitHeight: DeskLook.row
    topPadding: 0
    bottomPadding: 0
    leftPadding: Look.e1
    rightPadding: Look.e2
    text: owner.kinds.title(section)
    Accessible.role: Accessible.Button
    Accessible.checkable: true
    Accessible.checked: open
    onClicked: {
        const next = Object.assign({}, owner.folded);
        next[section] = open;
        owner.folded = next;
    }

    contentItem: RowLayout {
        spacing: Look.e2

        IconImage {
            name: head.open ? "caret-down" : "caret-right"
            sourceSize: Qt.size(DeskLook.tabIcon, DeskLook.tabIcon)
            color: Look.mist
        }

        QQC2.Label {
            Layout.fillWidth: true
            text: head.text
            font.pixelSize: DeskLook.tRow
            font.weight: Font.DemiBold
            elide: Text.ElideRight
        }

        QQC2.Label {
            text: head.count
            color: Look.mist
            font.pixelSize: DeskLook.tKind
        }
    }
}
