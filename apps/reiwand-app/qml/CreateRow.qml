import QtQuick
import io.github.reiwand.app

Collection {
    id: row

    property bool grows
    // Off when the page scrolls as one: the wheel then scrolls it instead of turning the row.
    property bool wheels: true
    property bool active
    property alias currentIndex: list.currentIndex
    property int chosenAt: -1
    readonly property int pad: Look.inset + Look.ring
    readonly property real contentWidth: list.contentWidth + 2 * pad
    readonly property var current: model && currentIndex >= 0 ? model[currentIndex] : null

    signal taken

    function turn(dx, dy) {
        wheel.turn(dx, dy);
    }

    view: list
    activeFocusOnTab: true
    implicitHeight: list.contentItem.childrenRect.height + 2 * pad
    onActiveFocusChanged: {
        if (activeFocus)
            taken();
    }
    Accessible.role: Accessible.List
    Accessible.name: label

    ListView {
        id: list

        anchors.fill: parent
        orientation: ListView.Horizontal
        spacing: Look.e4
        leftMargin: row.pad
        rightMargin: row.pad
        focus: true
        clip: true
        interactive: contentWidth > width
        keyNavigationEnabled: true
        boundsBehavior: Flickable.StopAtBounds
        model: row.model

        Wheel {
            id: wheel

            row: list
            enabled: row.wheels
            onMoved: row.activeFocus || row.forceActiveFocus()
        }

        delegate: Item {
            id: slot

            required property var modelData
            required property int index

            width: tile.width
            height: tile.y + tile.height

            Tile {
                id: tile

                y: row.pad + (row.grows ? Math.round((Look.coverCreateFocus - width) * 9 / 16) : 0)
                width: implicitWidth
                height: implicitHeight
                picture: slot.modelData.picture ?? ""
                icon: slot.modelData.icon ?? ""
                name: slot.modelData.name
                note: slot.modelData.note ?? ""
                noteIcon: slot.modelData.noteIcon ?? ""
                noteTone: slot.modelData.noteTone ?? ""
                off: slot.modelData.off === true
                grows: row.grows
                lit: row.active && slot.index === list.currentIndex
                chosen: slot.index === row.chosenAt
                onTapped: row.tap(slot.index, 0)
                onDoubleTapped: row.activated(slot.index)
                onAskedMenu: {
                    row.tap(slot.index, 0);
                    row.asked(slot.index, tile);
                }
            }
        }
    }
}
