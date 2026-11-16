import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Flow {
    id: bar

    required property var tools
    required property int total
    property bool all: false
    readonly property bool tight: all || width < Look.tapaFoco * 2 + Look.tapa
    property int word: App.libraryFilter.word ?? 0
    property string kind: App.libraryFilter.kind ?? ""
    property string tag: App.libraryFilter.tag ?? ""
    property var tags: []
    property var counts: queries.map(() => 0)
    readonly property bool favorite: word === 6
    readonly property var queries: ["", "kind:scene", "kind:image provider:no", "kind:video", "kind:web", "provider:yes", "fav:yes"]
    readonly property var names: [App.tr("All"), App.tr("Scenes"), App.tr("Pictures"), App.tr("Videos"), App.tr("Web"), App.tr("From sources"), App.tr("Favorites")]
    readonly property var shown: names.map((name, index) => index).filter(index => index === 0 || index === 6 || index === word || counts[index] > 0)
    readonly property bool worded: kind === "" && tag === ""
    readonly property string term: kind !== "" ? "kind:" + kind : queries[word]
    readonly property bool filtered: word !== 0 || kind !== "" || tag !== "" || find.text !== ""
    readonly property alias text: find.text
    readonly property alias find: find
    readonly property alias words: words
    readonly property var extra: [{
            "text": App.tr("Packages"),
            "kind": "package"
        }, {
            "text": App.tr("Assets"),
            "kind": "assets"
        }]

    signal changed
    signal flipped
    signal turned(real dx, real dy)

    function tagsLoad() {
        App.query("ListTags", [], "Library1", reply => bar && (bar.tags = reply.tags));
    }

    function keep() {
        App.libraryFilter = {
            "word": word,
            "kind": kind,
            "tag": tag
        };
    }

    function countFavorites() {
        queries.forEach((query, index) => App.query("Search", [query, 0, 0], "Library1", reply => bar && (bar.counts = bar.counts.map((count, at) => at === index ? reply.total : count))));
    }

    function clear() {
        find.text = "";
        word = 0;
        kind = "";
        tag = "";
        changed();
    }

    spacing: Look.e4
    width: parent.width
    onWordChanged: keep()
    onKindChanged: keep()
    onTagChanged: keep()
    Component.onCompleted: {
        tagsLoad();
        countFavorites();
    }

    Filtro {
        id: words

        label: App.tr("Show")
        Keys.onDownPressed: bar.tools.page.covers.view.forceActiveFocus(Qt.TabFocusReason)
        current: bar.worded ? bar.shown.indexOf(bar.word) : -1
        words: bar.shown.map(index => ({
                    "text": bar.names[index],
                    "count": bar.counts[index]
                }))
        onPicked: index => {
            bar.word = bar.shown[index];
            bar.kind = "";
            bar.tag = "";
            bar.changed();
        }
    }

    QQC2.Button {
        id: more

        flat: true
        text: bar.kind !== "" ? bar.extra.find(entry => entry.kind === bar.kind).text : bar.tag !== "" ? Labels.tag(bar.tag) : App.tr("More")
        icon.name: "caret-down"
        LayoutMirroring.enabled: true
        onClicked: menu.open()
    }

    ListaMenu {
        id: menu

        readonly property var tagged: bar.tags.map(tag => ({
                    "text": Labels.tag(tag.name),
                    "note": String(tag.count),
                    "tag": tag.name
                }))

        opener: more
        title: App.tr("More")
        options: bar.extra.concat(bar.tags.length > 0 ? [{
                    "separator": true
                }] : []).concat(tagged).concat(bar.tag !== "" ? [{
                    "separator": true
                }, {
                    "text": App.tr("Rename tag…"),
                    "icon": "pencil-simple",
                    "rename": true
                }] : [])
        onChosen: index => {
            const option = options[index];
            if (option.rename)
                return bar.tools.renameTag(bar.tag, more);
            bar.word = 0;
            bar.kind = option.kind ?? "";
            bar.tag = option.tag ?? "";
            bar.changed();
        }
    }

    Buscar {
        id: find

        placeholder: bar.worded && bar.word > 0 ? App.tr("Search in %1", bar.names[bar.word]) : bar.all ? App.tr("Search") : App.tr("Search your library")
        onTextChanged: wait.restart()
        onAccepted: {
            wait.stop();
            bar.changed();
        }
    }

    Text {
        visible: bar.text !== "" && bar.worded
        height: find.height
        verticalAlignment: Text.AlignVCenter
        text: App.tr("%1 of %2", bar.total, bar.counts[bar.word])
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuenta
        color: Look.bruma
    }

    WheelHandler {
        target: null
        onWheel: event => bar.turned(event.angleDelta.x, event.angleDelta.y)
    }

    Timer {
        id: wait

        interval: 250
        onTriggered: bar.changed()
    }

    QQC2.Button {
        id: adder

        flat: true
        icon.name: "plus"
        text: App.tr("Add wallpapers…")
        display: bar.tight ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
        onClicked: bar.tools.dialogs.importer.start(adder)
        QQC2.ToolTip.text: text
        QQC2.ToolTip.visible: bar.tight && hovered
    }

    QQC2.Button {
        flat: true
        icon.name: bar.all ? "rows" : "squares-four"
        text: bar.all ? App.tr("Show shelf") : App.tr("See all")
        onClicked: bar.flipped()
    }

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic !== "library")
                return;
            bar.tagsLoad();
            bar.countFavorites();
        }
    }
}
