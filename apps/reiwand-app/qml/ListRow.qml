import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

FocusScope {
    id: row

    property string text
    property string note
    property string noteTone
    property string value
    property string number
    property string explain
    property string status
    property string statusTone
    property string kind: "text"
    property bool off: false
    property bool checked: false
    property bool open: false
    property var options: []
    property int current: 0
    property real from: 0
    property real to: 1
    property real stepSize: 0.1
    property real at: 0
    property string fieldText
    property string placeholder
    property var keys: []
    property var words: []
    property int wordAt: 0
    readonly property bool lit: activeFocus
    readonly property bool hovered: hover.hovered
    readonly property real base: Math.max(Look.rowHeight, label.implicitHeight + Look.e4)
    readonly property real labelEnd: (base + label.implicitHeight) / 2 + Look.e2
    readonly property bool below: side.visible && room < Look.explainWidth / 2
    readonly property real room: {
        let box = row;
        while (lit && box.parent && !box.clip)
            box = box.parent;
        return box.width - row.mapToItem(box, width + Look.explainGap, 0).x - Look.e6;
    }
    readonly property color ink: lit ? (off ? Look.frost : Look.night) : off ? Look.mistLow : Look.frost
    readonly property color soft: lit ? (off ? Look.mist : Look.night) : off ? Look.mistLow : Look.mist

    signal toggled
    signal chosen(int index)
    signal moved(real value)
    signal edited(string text)
    signal captured(var keys)
    signal opened
    signal worded(int index)

    function step(by) {
        if (off)
            return;
        if (kind === "text" && words.length > 1) {
            wordAt = Math.max(0, Math.min(words.length - 1, wordAt + by));
        } else if (kind === "choice" && options.length > 0) {
            const next = Math.max(0, Math.min(options.length - 1, current + by));
            if (next !== current) {
                current = next;
                chosen(current);
            }
        } else if (kind === "slider") {
            const next = Math.max(from, Math.min(to, at + by * stepSize));
            if (next !== at) {
                at = next;
                moved(at);
            }
        }
    }

    function act() {
        if (off)
            return;
        if (kind === "text" && words.length > 0) {
            worded(wordAt);
        } else if (kind === "switch") {
            checked = !checked;
            toggled();
        } else if (kind === "expand") {
            open = !open;
            opened();
        } else if (kind === "page") {
            opened();
        } else if (kind === "field" || kind === "capture") {
            valueBox.begin();
        } else if (kind === "choice") {
            step(current + 1 < options.length ? 1 : -current);
        }
    }

    implicitHeight: below ? Math.max(base, labelEnd + side.implicitHeight + Look.e4) : base
    implicitWidth: Look.rowsWidth
    activeFocusOnTab: true
    Accessible.role: kind === "switch" ? Accessible.CheckBox : kind === "slider" ? Accessible.Slider : kind === "choice" ? Accessible.ComboBox : kind === "field" ? Accessible.EditableText : kind === "text" ? Accessible.StaticText : Accessible.Button
    Accessible.name: text
    Accessible.description: [note, kind === "choice" ? options[current] ?? "" : value, number, explain].filter(part => part !== "").join(". ")
    Accessible.checkable: kind === "switch"
    Accessible.checked: checked
    Accessible.focusable: true
    Keys.onUpPressed: nextItemInFocusChain(false)?.forceActiveFocus(Qt.TabFocusReason)
    Keys.onDownPressed: nextItemInFocusChain(true)?.forceActiveFocus(Qt.TabFocusReason)
    Keys.onLeftPressed: step(-1)
    Keys.onRightPressed: step(1)
    Keys.onSpacePressed: act()
    Keys.onReturnPressed: act()
    Keys.onEnterPressed: act()

    Rectangle {
        anchors.fill: parent
        radius: Look.radius
        color: row.lit ? (row.off ? Look.nightLine : Look.frost) : row.hovered ? Look.nightHigh : "transparent"

        Shadow {
            id: lift

            active: row.lit && !row.off && lift.deep
            kind: "row"
        }
    }

    HoverHandler {
        id: hover
    }

    TapHandler {
        onTapped: {
            row.forceActiveFocus(Qt.MouseFocusReason);
            if (row.kind !== "choice" && row.kind !== "slider")
                row.act();
        }
    }

    RowLayout {
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: row.base
        anchors.leftMargin: Look.rowSide
        anchors.rightMargin: Look.rowEnd
        spacing: Look.e4

        Column {
            id: label

            Layout.fillWidth: true
            spacing: 2

            Text {
                width: parent.width
                text: row.text
                wrapMode: Text.WordWrap
                font.family: Look.titleFont
                font.pixelSize: Look.tRow
                font.weight: row.lit && !row.off ? Font.Bold : Font.Medium
                font.letterSpacing: -0.01 * Look.tRow
                font.wordSpacing: 0.06 * Look.tRow
                color: row.ink
            }

            Text {
                width: parent.width
                visible: row.note !== ""
                text: row.note
                wrapMode: Text.WordWrap
                font.family: Look.textFont
                font.pixelSize: Look.tFoot
                font.weight: row.lit && !row.off ? Font.Bold : Font.Normal
                color: Tone.of(row.noteTone, row.lit && !row.off, row.soft)
            }
        }

        ListRowValue {
            id: valueBox

            row: row
        }
    }

    ListRowExplain {
        id: side

        row: row
    }
}
