import QtQuick
import io.github.reiwand.app

Loader {
    id: fixed

    required property var covers

    visible: covers.all
    active: covers.all
    y: covers.grid.height + Look.e5
    width: parent.width
    sourceComponent: covers.page.choosing ? chosenLine : litCard

    Component {
        id: chosenLine

        FixedCard {
            LibraryChosen {
                page: fixed.covers.page
            }
        }
    }

    Component {
        id: litCard

        FixedCard {
            title: fixed.covers.grid.currentName
            caption: fixed.covers.grid.currentCaption

            LibraryCard {
                page: fixed.covers.page
            }
        }
    }
}
