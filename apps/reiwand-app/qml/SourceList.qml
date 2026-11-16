import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

FocusScope {
    id: list

    required property var providers
    required property var playlists
    readonly property int total: providers.reduce((sum, p) => sum + p.items, 0)
    readonly property bool offline: providers.some(p => p.offline)
    property bool wide: false
    readonly property real tallest: column.implicitHeight + Look.e3 + own.height
    readonly property real lowest: Math.min(tallest, Look.altoBoton * 5 + Look.e4 + Look.e3)

    signal rotate(var provider)
    signal details(var provider)
    signal fix(var provider)
    signal addAsked
    signal refreshAsked(string id)

    function enter() {
        const first = cards.itemAt(0);
        (first || adding).forceActiveFocus(Qt.TabFocusReason);
    }

    function card(id) {
        return cards.itemAt(providers.findIndex(p => p.id === id));
    }

    function show(item) {
        const top = item.mapToItem(flick.contentItem, 0, 0).y;
        if (top < flick.contentY)
            flick.contentY = Math.max(-flick.topMargin, top - Look.e3);
        else if (top + item.height > flick.contentY + flick.height)
            flick.contentY = Math.min(flick.contentHeight + flick.bottomMargin - flick.height, top + item.height - flick.height + Look.e3);
    }

    function rotating(provider) {
        return playlists.some(l => (l.query || "").includes("source:" + provider.source) && l.outputs.length > 0);
    }

    Flickable {
        id: flick

        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.bottom: own.top
        anchors.margins: -Look.anilloLugar
        anchors.bottomMargin: Look.e3 - Look.anilloLugar
        leftMargin: Look.anilloLugar
        rightMargin: Look.anilloLugar
        topMargin: Look.anilloLugar
        bottomMargin: Look.anilloLugar
        contentHeight: Math.max(height - 2 * Look.anilloLugar, column.implicitHeight)
        interactive: contentHeight + 2 * Look.anilloLugar > height
        clip: true

        Column {
            id: column

            anchors.bottom: parent.bottom
            width: list.wide ? flick.width - 2 * Look.anilloLugar : Math.min(flick.width - 2 * Look.anilloLugar, Look.tiposAncho)
            spacing: Look.e4

            Flow {
                visible: list.offline
                width: column.width
                spacing: Look.e3

                Nota {
                    height: Math.max(again.height, implicitHeight)
                    tone: "ambar"
                    wrap: true
                    fitWidth: column.width
                    text: App.tr("No connection. Your sources cannot bring new photos. What you already have stays in your library.")
                }

                QQC2.Button {
                    id: again

                    text: App.tr("Try again")
                    icon.name: "arrows-clockwise"
                    enabled: !App.cut
                    onClicked: list.refreshAsked("")
                }
            }

            Item {
                readonly property bool wraps: filter.width + Look.e3 + buttons.width > width

                width: column.width
                height: wraps ? filter.height + Look.e3 + buttons.height : buttons.height

                Filtro {
                    id: filter

                    words: [{
                            "text": App.trc("all the sources", "All"),
                            "count": list.providers.length
                        }]
                    y: parent.wraps ? 0 : (parent.height - height) / 2
                }

                Row {
                    id: buttons

                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                    spacing: Look.e3

                    QQC2.Button {
                        enabled: list.providers.length > 0 && !App.cut
                        text: App.tr("Update all")
                        icon.name: "arrows-clockwise"
                        onClicked: list.refreshAsked("")
                    }

                    QQC2.Button {
                        id: adding

                        highlighted: list.providers.length === 0
                        text: App.tr("Add a source")
                        icon.name: "plus"
                        onClicked: list.addAsked()
                        Keys.onDownPressed: list.enter()
                    }
                }
            }

            Repeater {
                id: cards

                model: list.providers

                delegate: SourceCard {
                    id: shownCard

                    required property var modelData
                    required property int index

                    width: column.width
                    provider: modelData
                    rotating: list.rotating(modelData)
                    onRotate: list.rotate(modelData)
                    onRefresh: list.refreshAsked(modelData.id)
                    onDetails: list.details(modelData)
                    onFix: list.fix(modelData)
                    onUp: index > 0 ? cards.itemAt(index - 1).enter() : adding.forceActiveFocus(Qt.TabFocusReason)
                    onDown: index < cards.count - 1 && cards.itemAt(index + 1).enter()
                    onActiveFocusChanged: activeFocus && list.show(shownCard)
                }
            }
        }
    }

    QQC2.Button {
        id: own

        anchors.left: parent.left
        anchors.bottom: parent.bottom
        flat: true
        text: App.tr("Your own photos? Add a folder to your library")
        icon.name: "plus"
        onClicked: folder.open()
    }

    Chooser {
        id: folder

        mode: Chooser.Folder
        onAccepted: App.act("AddSource", [App.path(selectedFolder), true], "Library1")
    }
}
