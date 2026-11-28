import QtQuick
import io.github.reiwand.app

Column {
    id: nowBlock

    property var page
    property string picked
    property bool none: false
    property int stocked: -1
    property bool compact: false
    readonly property var screens: App.state.outputs ?? {}
    readonly property bool several: App.outputs.length > 1
    readonly property bool strip: several && !none && !compact
    readonly property string screen: {
        const wanted = page && page.target !== undefined ? page.target : picked;
        return App.outputs.indexOf(wanted) >= 0 ? wanted : App.outputs.length > 0 ? App.outputs[0] : "";
    }
    readonly property var info: screens[screen] ?? {}
    readonly property string item: info.item ?? ""
    readonly property var playlist: info.playlist ?? null
    readonly property string title: none ? stocked > 0 ? App.tr("You have not set any wallpaper yet") : App.tr("You have no wallpapers yet") : details.uid !== undefined ? Names.of(details) : ""
    property var details: ({})
    property bool fresh: false
    readonly property alias controls: controls
    readonly property alias rotating: rotating
    readonly property alias tasks: tasks
    readonly property string kindWord: ({
            "scene": App.tr("Scene"),
            "image": App.tr("Image"),
            "video": App.tr("Video"),
            "web": App.tr("Web page")
        })[info.kind ?? ""] ?? ""

    function choose(name) {
        if (page && page.target !== undefined)
            page.target = name;
        else
            picked = name;
    }

    function load() {
        const uid = item;
        if (uid === "")
            details = {};
        else
            App.query("GetItem", [uid], "Library1", reply => nowBlock && nowBlock.item === uid && (nowBlock.details = reply.item ?? {}));
    }

    function count() {
        if (none)
            App.query("Search", ["", 0, 0], "Library1", reply => nowBlock && (nowBlock.stocked = reply.total));
    }

    spacing: 0
    Accessible.role: Accessible.Grouping
    Accessible.name: App.tr("On your desktop")
    onItemChanged: {
        fresh = nowBlock.Window.window !== null && nowBlock.Window.window.visible && details.uid !== undefined;
        load();
        if (fresh)
            freshness.restart();
    }
    onNoneChanged: count()
    Component.onCompleted: {
        load();
        count();
    }

    Timer {
        id: freshness

        interval: 60000
        onTriggered: nowBlock.fresh = false
    }

    Connections {
        target: App

        function onUpdated(topic, ids) {
            if (topic === "library" && (ids.length === 0 || ids.includes(nowBlock.item)))
                nowBlock.load();
            if (topic === "library")
                nowBlock.count();
        }
    }

    Text {
        anchors.right: parent.right
        visible: (!nowBlock.several || nowBlock.none) && !nowBlock.compact
        text: App.tr("On your desktop")
        font.family: Look.textFont
        font.pixelSize: Look.tBody
        font.weight: Font.Medium
        color: Look.mist
    }

    ScreenTiles {
        anchors.right: parent.right
        visible: nowBlock.strip
        current: nowBlock.screen
        onPicked: name => nowBlock.choose(name)
    }

    Text {
        anchors.right: parent.right
        topPadding: nowBlock.strip ? 0 : Look.e2
        visible: text !== ""
        text: nowBlock.title
        width: Math.min(implicitWidth, Look.coverFocus * 2)
        elide: Text.ElideRight
        fontSizeMode: Text.HorizontalFit
        minimumPixelSize: Look.tCover * 0.6
        horizontalAlignment: Text.AlignRight
        font.family: Look.titleFont
        font.pixelSize: Look.tCover
        font.weight: Font.DemiBold
        color: Look.frost
    }

    Text {
        anchors.right: parent.right
        topPadding: nowBlock.strip ? Look.e1 : Look.e3
        visible: text !== "" && !nowBlock.none && !nowBlock.compact
        text: [nowBlock.kindWord, nowBlock.info.credit ? nowBlock.info.credit.provider : ""].filter(t => t).join(" · ")
        font.family: Look.textFont
        font.pixelSize: Look.tName
        font.weight: Font.Medium
        color: Look.frost
    }

    NowLayers {
        anchors.right: parent.right
        info: nowBlock.none ? ({}) : nowBlock.info
    }

    Text {
        anchors.right: parent.right
        topPadding: Look.e3
        visible: nowBlock.none
        width: Math.min(implicitWidth, Look.coverFocus * 2)
        wrapMode: Text.WordWrap
        horizontalAlignment: Text.AlignRight
        text: nowBlock.stocked > 0 ? App.tr("Choose one in your library to put it on your desktop.") : App.tr("Choose where to start: the first thing that enters your library becomes your wallpaper.")
        font.family: Look.textFont
        font.pixelSize: Look.tName
        color: Look.mist
    }

    Item {
        width: 1
        height: Look.e1
    }

    StatusLine {
        anchors.right: parent.right
        visible: nowBlock.fresh && !nowBlock.none && !nowBlock.compact
        tone: "sprout"
        text: App.tr("Set a moment ago")
    }

    Item {
        width: 1
        height: Look.e5
        visible: !nowBlock.none
    }

    NowControls {
        id: controls

        anchors.right: parent.right
        visible: !nowBlock.none && !(nowBlock.compact && rotating.visible)
        screen: nowBlock.screen
        info: nowBlock.info
        item: nowBlock.item
        favorite: nowBlock.details.favorite === true
    }

    NowPlaylist {
        id: rotating

        anchors.right: parent.right
        topPadding: Look.e3
        screen: nowBlock.screen
        playlist: nowBlock.none ? null : nowBlock.playlist
    }

    NowTask {
        id: tasks

        anchors.right: parent.right
        topPadding: Look.e3
        height: nowBlock.compact ? 0 : implicitHeight
        clip: nowBlock.compact
    }
}
