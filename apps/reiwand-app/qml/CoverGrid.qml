import QtQuick
import io.github.reiwand.app

Collection {
    id: coverGrid

    property alias currentIndex: grid.currentIndex
    readonly property real room: grid.width - grid.leftMargin - grid.rightMargin
    readonly property int columns: Math.max(1, Math.floor(room / (Look.coverGrid + Look.shelfGap)))
    readonly property int cell: Math.floor(room / columns)
    readonly property real cover: cell - Look.shelfGap

    view: grid
    Accessible.role: Accessible.List
    Accessible.name: label

    FontMetrics {
        id: names

        font.family: Look.titleFont
        font.pixelSize: Look.tName
    }

    FontMetrics {
        id: notes

        font.family: Look.textFont
        font.pixelSize: Look.tFoot
    }

    GridView {
        id: grid

        anchors.fill: parent
        anchors.rightMargin: -Look.shelfGap
        topMargin: Look.inset + Look.ring
        leftMargin: Look.inset + Look.ring
        rightMargin: Look.inset + Look.ring
        bottomMargin: Look.e4
        cellWidth: coverGrid.cell
        cellHeight: Math.round(coverGrid.cover * 9 / 16) + Look.e3 + Math.ceil(names.height) + Look.noteTop + Math.max(Look.icon, Math.ceil(notes.height)) + Look.e5
        focus: true
        clip: true
        reuseItems: false
        cacheBuffer: cellHeight
        boundsBehavior: Flickable.StopAtBounds
        highlightFollowsCurrentItem: true
        highlightMoveDuration: Look.pace
        highlight: Item {
        }
        model: coverGrid.model
        Keys.onPressed: event => coverGrid.press(event)

        delegate: CoverView {
            width: coverGrid.cover
            even: true
            from: coverGrid
        }
    }
}
