import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Kirigami.Page {
    id: page

    property var scenes: []
    property var projects: JSON.parse(recents.projects() || "[]")
    property bool learn: false
    property bool all: false
    property string query: ""
    property bool changing: false
    property bool fixed: false
    property bool loaded: false
    property var focused: null
    property string named: App.tr("My scene %1", projects.length + 1)
    property string placed: ""
    readonly property var current: focused ? focused.current : null
    readonly property bool onRecent: focused === scroll.recent.row || focused === every.grid
    readonly property bool onAll: onRecent && current !== null && current.all === true
    readonly property int fits: Math.max(1, Math.floor((high.width - Look.coverCreateFocus + Look.coverCreate - 2 * Look.inset - 2 * Look.ring + Look.e4) / (Look.coverCreate + Look.e4)) - 1)
    readonly property bool atExamples: learn && focused !== null && !onRecent
    readonly property bool missing: onRecent && current !== null && current.missing === true
    readonly property string where: recents.shown(placed !== "" ? placed : helper.folderFor(named))
    readonly property real bandLow: all ? 0 : footer.height + Look.e4 + rows.oneLine
    readonly property real bandHigh: bandLow
    readonly property bool shared: scroll.shared
    readonly property real upperHeight: scroll.recent.implicitHeight
    readonly property real bandFrom: Math.min(upper.y, band.y)
    property rect band
    property rect upper
    readonly property rect low: band.width > 0 ? band : Qt.rect(0, upperHeight + Look.e5, width, Math.max(0, height - upperHeight - Look.e5))
    readonly property rect high: upper.width > 0 ? upper : Qt.rect(0, 0, width, upperHeight)
    readonly property alias facts: facts
    readonly property alias helper: helper
    readonly property alias recents: recents
    readonly property alias copy: copy
    readonly property alias rows: scroll.rows
    readonly property alias media: media
    readonly property alias every: every
    readonly property var recentRow: scroll.recent.row
    readonly property alias foot: footer
    readonly property alias menu: menu

    function reset() {
        learn = projects.length === 0 && scenes.length > 0;
        focused = all ? every.grid : projects.length > 0 ? scroll.recent.row : learn ? rows.learn : rows.scene;
        Qt.callLater(() => footer && footer.land());
    }

    function create(starter, media) {
        App.media = media;
        App.compose(placed !== "" ? placed : helper.folderFor(named), starter, named);
    }

    function open(folder) {
        if (!recents.isProject(folder))
            return facts.refuse(folder);
        App.compose(folder, "", "");
    }

    function pickFolder() {
        folders.open();
    }

    function act(row) {
        const entry = row.current;
        if (!entry)
            return;
        if (page.onRecent)
            return entry.all ? showAll(true) : entry.missing ? pickFolder() : open(entry.folder);
        if (row === rows.learn)
            return facts.copyOf(entry);
        facts.make(entry.id);
    }

    function showAll(on) {
        all = on;
        every.find.text = "";
        focused = on ? every.grid : scroll.recent.row;
        Qt.callLater(() => page.focused && page.focused.forceActiveFocus(Qt.TabFocusReason));
    }

    function exercise() {
        facts.exercise();
    }

    title: App.tr("Create")
    padding: 0
    background: null
    property var keyHints: facts.hints
    Component.onCompleted: fixed ? reset() : facts.load()
    onVisibleChanged: Qt.callLater(() => footer && footer.land())

    SceneEditor {
        id: helper
    }

    Recents {
        id: recents
    }

    Chooser {
        id: folders

        mode: Chooser.Folder
        title: App.tr("Choose the folder of a project")
        onAccepted: {
            const folder = App.path(selectedFolder);
            if (page.missing)
                recents.forgetProject(page.current.folder);
            page.open(folder);
        }
    }

    CreateMedia {
        id: media

        page: page
    }

    CreateCopy {
        id: copy
    }

    CreateData {
        id: facts

        page: page
    }

    CreateScroll {
        id: scroll

        page: page
    }

    CreateAll {
        id: every

        page: page
        visible: page.all
        width: page.width
        height: Math.max(0, page.height - footer.height - Look.e4)
    }

    CreateMenu {
        id: menu

        page: page
    }

    CreateFooter {
        id: footer

        page: page
        x: page.low.x
        y: page.low.y + page.low.height - height
        width: page.low.width - (page.low.x < page.high.x ? room.width + Look.e6 : 0)
    }

    Hints {
        id: room

        visible: false
        model: page.keyHints
    }
}
