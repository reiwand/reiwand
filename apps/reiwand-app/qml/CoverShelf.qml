import QtQuick
import io.github.reiwand.app

Collection {
    id: shelf

    property Component actions
    property Component selection
    property alias currentIndex: list.currentIndex
    readonly property Item cardItem: list.currentItem?.cardItem ?? null
    readonly property real place: Look.inset + Look.ring + Look.cover + Look.shelfGap
    readonly property bool turning: wheel.turning

    function turn(dx, dy) {
        wheel.turn(dx, dy);
    }

    view: list
    implicitHeight: Math.max(list.currentItem?.height ?? 0, Math.round(Look.coverFocus * 9 / 16) + Look.e7 + Look.e4 + Look.inset + Look.ring)
    Accessible.role: Accessible.List
    Accessible.name: label

    Item {
        x: -Look.margin
        width: parent.width + Look.margin
        height: parent.height + Look.ringRoom
        clip: true

        ListView {
            id: list

            x: Look.margin
            width: parent.width - Look.margin
            height: parent.height
            orientation: ListView.Horizontal
            spacing: Look.shelfGap
            leftMargin: Look.inset + Look.ring
            rightMargin: Math.max(Look.inset + Look.ring, width - shelf.place - Look.coverFocus)
            displayMarginBeginning: Look.margin
            focus: true
            reuseItems: false
            cacheBuffer: Look.cover
            boundsBehavior: Flickable.StopAtBounds
            highlightFollowsCurrentItem: true
            highlightMoveDuration: Look.pace
            highlightRangeMode: ListView.ApplyRange
            preferredHighlightBegin: shelf.place
            preferredHighlightEnd: shelf.place + Look.coverFocus + Look.inset + Look.ring
            highlight: Item {
            }

            Wheel {
                id: wheel

                row: list
            }
            model: shelf.model
            Keys.onPressed: event => shelf.press(event)
            Component.onCompleted: Qt.callLater(() => {
                positionViewAtBeginning();
                positionViewAtIndex(currentIndex, ListView.Contain);
            })

            delegate: Item {
                id: slot

                required property var model
                required property int index
                readonly property alias name: cover.name
                readonly property alias caption: cover.caption
                readonly property real room: cover.room
                readonly property Item cardItem: deck.item
                readonly property alias picture: cover.picture

                activeFocusOnTab: ListView.isCurrentItem || activeFocus
                width: cover.width + (cover.lit ? Look.inset + Look.ring : 0)
                height: cover.y + cover.height
                z: cover.lit ? 1 : 0

                CoverView {
                    id: cover

                    y: Look.inset + Look.ring + Math.round((Look.coverFocus - width) * 9 / 16)
                    width: implicitWidth
                    height: implicitHeight
                    model: slot.model
                    index: slot.index
                    from: shelf

                    Loader {
                        id: deck

                        readonly property bool choosing: shelf.choosing

                        active: cover.lit
                        x: choosing || !item ? 0 : Math.min(0, list.width - Look.inset - Look.ring - (slot.x - list.contentX) - Math.max(item.width, item.implicitWidth))
                        width: choosing && item ? Math.min(item.implicitWidth, list.width - Look.margin - (slot.x - list.contentX)) : undefined
                        sourceComponent: choosing ? apart : card
                    }
                }
            }
        }
    }

    Component {
        id: apart

        Item {
            implicitWidth: chosenRow.implicitWidth
            implicitHeight: chosenRow.y + chosenRow.implicitHeight

            Loader {
                id: chosenRow

                y: list.currentItem?.room ?? 0
                width: parent.width
                sourceComponent: shelf.selection
            }
        }
    }

    Component {
        id: card

        Card {
            title: list.currentItem?.name ?? ""
            caption: list.currentItem?.caption ?? ""

            Loader {
                sourceComponent: shelf.actions
            }
        }
    }
}
