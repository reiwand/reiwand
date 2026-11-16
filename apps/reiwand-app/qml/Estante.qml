import QtQuick
import io.github.reiwand.app

Coleccion {
    id: estante

    property Component actions
    property Component selection
    property alias currentIndex: list.currentIndex
    readonly property Item ficha: list.currentItem?.ficha ?? null
    readonly property real place: Look.filete + Look.anillo + Look.tapa + Look.huecoEstante
    readonly property bool turning: wheel.turning

    function turn(dx, dy) {
        wheel.turn(dx, dy);
    }

    view: list
    implicitHeight: Math.max(list.currentItem?.height ?? 0, Math.round(Look.tapaFoco * 9 / 16) + Look.e7 + Look.e4 + Look.filete + Look.anillo)
    Accessible.role: Accessible.List
    Accessible.name: label

    Item {
        x: -Look.margen
        width: parent.width + Look.margen
        height: parent.height + Look.anilloLugar
        clip: true

        ListView {
            id: list

            x: Look.margen
            width: parent.width - Look.margen
            height: parent.height
            orientation: ListView.Horizontal
            spacing: Look.huecoEstante
            leftMargin: Look.filete + Look.anillo
            rightMargin: Math.max(Look.filete + Look.anillo, width - estante.place - Look.tapaFoco)
            displayMarginBeginning: Look.margen
            focus: true
            reuseItems: false
            cacheBuffer: Look.tapa
            boundsBehavior: Flickable.StopAtBounds
            highlightFollowsCurrentItem: true
            highlightMoveDuration: Look.paso
            highlightRangeMode: ListView.ApplyRange
            preferredHighlightBegin: estante.place
            preferredHighlightEnd: estante.place + Look.tapaFoco + Look.filete + Look.anillo
            highlight: Item {
            }

            Rueda {
                id: wheel

                fila: list
            }
            model: estante.model
            Keys.onPressed: event => estante.press(event)
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
                readonly property Item ficha: deck.item

                activeFocusOnTab: ListView.isCurrentItem || activeFocus
                width: cover.width + (cover.lit ? Look.filete + Look.anillo : 0)
                height: cover.y + cover.height
                z: cover.lit ? 1 : 0

                TapaVista {
                    id: cover

                    y: Look.filete + Look.anillo + Math.round((Look.tapaFoco - width) * 9 / 16)
                    width: implicitWidth
                    height: implicitHeight
                    model: slot.model
                    index: slot.index
                    from: estante

                    Loader {
                        id: deck

                        readonly property bool choosing: estante.choosing

                        active: cover.lit
                        x: choosing || !item ? 0 : Math.min(0, list.width - Look.filete - Look.anillo - (slot.x - list.contentX) - Math.max(item.width, item.implicitWidth))
                        width: choosing && item ? Math.min(item.implicitWidth, list.width - Look.margen - (slot.x - list.contentX)) : undefined
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
                sourceComponent: estante.selection
            }
        }
    }

    Component {
        id: card

        Ficha {
            title: list.currentItem?.name ?? ""
            caption: list.currentItem?.caption ?? ""

            Loader {
                sourceComponent: estante.actions
            }
        }
    }
}
