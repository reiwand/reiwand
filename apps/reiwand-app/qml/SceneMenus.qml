import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The menus as words of the editor's top bar: File, Edit, Layer, View and Window, each opened with
// a click, Alt and its underlined letter, or Alt alone and the arrows; in a window narrower than
// DeskLook.folds they fold into one "Menu".
Row {
    id: menus

    property var page: null
    readonly property SceneMenuWords words: SceneMenuWords {}
    readonly property var names: words.namesOf(page)
    readonly property bool folded: ((Window.window ?? page?.Window.window)?.width ?? 0) < DeskLook.folds
    readonly property alias popup: popup
    readonly property alias naming: naming
    readonly property alias owning: owning
    property bool cued: false
    property int shown: -1
    property Item back: null

    function letter(name) {
        const title = words.title(name);
        const at = title.indexOf("&");
        return at >= 0 ? title.charAt(at + 1) : "";
    }

    function plain(name) {
        return words.title(name).replace("&", "");
    }

    function marked(name) {
        const title = words.title(name);
        const at = title.indexOf("&");
        return at < 0 || !cued ? plain(name) : title.slice(0, at) + "<u>" + title.charAt(at + 1) + "</u>" + title.slice(at + 2);
    }

    function button(index) {
        return folded ? fold : row.itemAt(index);
    }

    function open(index) {
        const from = button(index);
        if (!from)
            return;
        shown = index;
        popup.show(from, 0, from.height + Look.e1, words.entries(menus.page, menus.names[index]));
    }

    function step(by) {
        const next = (shown + by + menus.names.length) % menus.names.length;
        popup.pending = null;
        popup.close();
        Qt.callLater(open, next);
    }

    function enter() {
        back = menus.page?.Window.activeFocusItem ?? null;
        (folded ? fold : row.itemAt(0))?.forceActiveFocus(Qt.ShortcutFocusReason);
    }

    function leave() {
        cued = false;
        back?.forceActiveFocus(Qt.ShortcutFocusReason);
    }

    spacing: 0

    Repeater {
        id: row

        model: menus.folded ? [] : menus.names

        delegate: QQC2.ToolButton {
            id: word

            required property string modelData
            required property int index
            readonly property bool open: popup.opened && menus.shown === index

            implicitHeight: DeskLook.word
            leftPadding: DeskLook.wordSide
            rightPadding: DeskLook.wordSide
            text: menus.plain(modelData)
            font.family: Look.titleFont
            font.pixelSize: DeskLook.tWord
            font.weight: Font.Medium
            highlighted: false
            down: open
            Accessible.role: Accessible.ButtonMenu
            onClicked: menus.open(index)
            Keys.onLeftPressed: row.itemAt((index + row.count - 1) % row.count).forceActiveFocus(Qt.TabFocusReason)
            Keys.onRightPressed: row.itemAt((index + 1) % row.count).forceActiveFocus(Qt.TabFocusReason)
            Keys.onDownPressed: menus.open(index)
            Keys.onEscapePressed: menus.leave()
            onActiveFocusChanged: menus.cued = activeFocus || popup.opened

            contentItem: Text {
                text: menus.marked(word.modelData)
                textFormat: Text.StyledText
                font: word.font
                color: word.open || word.hovered || word.visualFocus ? Look.frost : Look.mist
                verticalAlignment: Text.AlignVCenter
            }

            Shortcut {
                sequence: "Alt+" + menus.letter(word.modelData)
                enabled: menus.page?.visible === true && menus.letter(word.modelData) !== ""
                onActivated: menus.open(word.index)
            }
        }
    }

    QQC2.ToolButton {
        id: fold

        visible: menus.folded
        implicitHeight: DeskLook.word
        icon.name: "rows"
        text: App.tr("Menu")
        font.pixelSize: DeskLook.tWord
        Accessible.role: Accessible.ButtonMenu
        onClicked: popup.show(fold, 0, fold.height + Look.e1, menus.names.map((name, at) => menus.words.entry(menus.plain(name), "", () => Qt.callLater(menus.open, at))))
        Keys.onDownPressed: clicked()
        Keys.onEscapePressed: menus.leave()

        Instantiator {
            model: menus.folded ? menus.names : []

            delegate: Shortcut {
                required property string modelData
                required property int index

                sequence: "Alt+" + menus.letter(modelData)
                enabled: menus.page?.visible === true && menus.letter(modelData) !== ""
                onActivated: menus.open(index)
            }
        }
    }

    SceneLayoutName {
        id: naming

        layout: menus.page?.layout ?? null
    }

    SceneLayoutOwn {
        id: owning

        layout: menus.page?.layout ?? null
        naming: naming
    }

    SceneMenu {
        id: popup

        Shortcut {
            sequence: "Left"
            enabled: popup.opened && menus.shown >= 0
            onActivated: menus.step(-1)
        }

        Shortcut {
            sequence: "Right"
            enabled: popup.opened && menus.shown >= 0
            onActivated: menus.step(1)
        }
    }

    Connections {
        target: popup

        function onClosed() {
            menus.cued = false;
        }
    }
}
