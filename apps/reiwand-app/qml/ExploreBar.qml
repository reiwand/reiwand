import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Row {
    id: bar

    required property var catalog
    property bool full
    property bool roomy: true
    readonly property real room: parent ? parent.width : 0
    readonly property real others: [filter, hint, link, off, toggle].filter(item => item.visible).reduce((sum, item) => sum + item.width + spacing, 0)
    readonly property alias filter: filter
    readonly property alias search: search
    readonly property alias link: link
    readonly property alias off: off
    readonly property alias adder: adder
    readonly property alias toggle: toggle

    signal flipped

    spacing: Look.e5

    Filtro {
        id: filter

        anchors.verticalCenter: parent.verticalCenter
        visible: bar.catalog.words.length > 0
        label: App.tr("Repositories")
        words: bar.catalog.words
        current: bar.catalog.word
        onPicked: index => bar.catalog.word = index
    }

    Text {
        id: hint

        anchors.verticalCenter: parent.verticalCenter
        visible: App.cut || (bar.catalog.lone && bar.roomy)
        text: App.cut ? App.tr("Private mode: the network is cut.") : App.tr("Do you have wallpapers elsewhere? Add a repository with its address.")
        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
        color: Look.bruma
    }

    QQC2.Button {
        id: link

        anchors.verticalCenter: parent.verticalCenter
        text: App.tr("Add a repository")
        icon.name: "link-simple"
        onClicked: adder.open()
        Keys.onReturnPressed: clicked()
        Keys.onEnterPressed: clicked()
    }

    QQC2.Button {
        id: off

        anchors.verticalCenter: parent.verticalCenter
        visible: App.cut
        text: App.tr("Turn off")
        onClicked: App.act("SetOption", ["network.private", "false"], "Daemon1")
        Keys.onReturnPressed: clicked()
        Keys.onEnterPressed: clicked()
    }

    Buscar {
        id: search

        width: bar.room > 0 ? Math.max(Look.buscarAncho / 2, Math.min(Look.buscarAncho, bar.room - bar.others)) : Look.buscarAncho
        placeholder: App.tr("Search the repositories")
        onAccepted: bar.catalog.query = text.trim()
        onTextChanged: text === "" && (bar.catalog.query = "")
    }

    QQC2.Button {
        id: toggle

        anchors.verticalCenter: parent.verticalCenter
        visible: bar.catalog.covers.length > 0
        flat: true
        text: bar.full ? App.tr("See the shelf") : App.tr("See all")
        icon.name: bar.full ? "rows" : "squares-four"
        onClicked: bar.flipped()
    }

    RepoAddLink {
        id: adder

        opener: link
    }
}
