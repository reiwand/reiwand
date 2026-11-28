import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Column {
    id: all

    required property var page
    readonly property alias grid: grid
    readonly property alias find: find

    spacing: Look.e4

    Row {
        id: head

        x: Look.inset + Look.ring
        spacing: Look.e5

        QQC2.Button {
            anchors.verticalCenter: parent.verticalCenter
            flat: true
            icon.name: "caret-left"
            text: App.tr("Back")
            onClicked: all.page.showAll(false)
            Keys.onDownPressed: grid.forceActiveFocus(Qt.TabFocusReason)
            Keys.onEscapePressed: all.page.showAll(false)
        }

        Text {
            anchors.verticalCenter: parent.verticalCenter
            text: App.tr("All my projects")
            font.family: Look.titleFont
            font.pixelSize: Look.tRow
            font.weight: Font.Bold
            font.letterSpacing: Look.tRow * Look.tight
            color: Look.frost
        }

        SearchBox {
            id: find

            anchors.verticalCenter: parent.verticalCenter
            placeholder: App.tr("Search your projects")
            onTextChanged: all.page.query = text
            onAccepted: grid.forceActiveFocus(Qt.TabFocusReason)
            Keys.onDownPressed: grid.forceActiveFocus(Qt.TabFocusReason)
            Keys.onEscapePressed: all.page.showAll(false)
        }
    }

    Text {
        x: Look.inset + Look.ring
        visible: grid.count === 0
        text: App.tr("No project is called like that")
        font.family: Look.textFont
        font.pixelSize: Look.tExplain
        color: Look.mist
    }

    CoverGrid {
        id: grid

        readonly property int count: model ? model.length : 0
        readonly property var current: model && currentIndex >= 0 && currentIndex < count ? model[currentIndex] : null

        width: parent.width
        height: Math.max(0, all.height - head.height - all.spacing)
        label: App.tr("All my projects")
        model: all.page.facts.allModel
        iconRole: "icon"
        onActiveFocusChanged: activeFocus && (all.page.focused = grid)
        onActivated: all.page.act(grid)
        onAsked: (index, at) => all.page.menu.ask(grid, at)
        Keys.onEscapePressed: all.page.showAll(false)
    }
}
