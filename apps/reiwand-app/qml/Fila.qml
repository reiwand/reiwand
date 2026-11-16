import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

FocusScope {
    id: fila

    property string text
    property string note
    property string noteTone
    property string value
    property string number
    property string explain
    property string status
    property string statusTone
    property string kind: "texto"
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
    readonly property real base: Math.max(Look.altoFila, etiqueta.implicitHeight + Look.e4)
    readonly property real labelEnd: (base + etiqueta.implicitHeight) / 2 + Look.e2
    readonly property bool below: side.visible && room < Look.explicaAncho / 2
    readonly property real room: {
        let box = fila;
        while (lit && box.parent && !box.clip)
            box = box.parent;
        return box.width - fila.mapToItem(box, width + Look.explicaHueco, 0).x - Look.e6;
    }
    readonly property color ink: lit ? (off ? Look.escarcha : Look.noche) : off ? Look.brumaBaja : Look.escarcha
    readonly property color soft: lit ? (off ? Look.bruma : Look.noche) : off ? Look.brumaBaja : Look.bruma

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
        if (kind === "texto" && words.length > 1) {
            wordAt = Math.max(0, Math.min(words.length - 1, wordAt + by));
        } else if (kind === "elige" && options.length > 0) {
            const next = Math.max(0, Math.min(options.length - 1, current + by));
            if (next !== current) {
                current = next;
                chosen(current);
            }
        } else if (kind === "riel") {
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
        if (kind === "texto" && words.length > 0) {
            worded(wordAt);
        } else if (kind === "llave") {
            checked = !checked;
            toggled();
        } else if (kind === "despliega") {
            open = !open;
            opened();
        } else if (kind === "pliega") {
            opened();
        } else if (kind === "campo" || kind === "captura") {
            valor.begin();
        } else if (kind === "elige") {
            step(current + 1 < options.length ? 1 : -current);
        }
    }

    implicitHeight: below ? Math.max(base, labelEnd + side.implicitHeight + Look.e4) : base
    implicitWidth: Look.filasAncho
    activeFocusOnTab: true
    Accessible.role: kind === "llave" ? Accessible.CheckBox : kind === "riel" ? Accessible.Slider : kind === "elige" ? Accessible.ComboBox : kind === "campo" ? Accessible.EditableText : kind === "texto" ? Accessible.StaticText : Accessible.Button
    Accessible.name: text
    Accessible.description: [note, kind === "elige" ? options[current] ?? "" : value, number, explain].filter(part => part !== "").join(". ")
    Accessible.checkable: kind === "llave"
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
        radius: Look.radio
        color: fila.lit ? (fila.off ? Look.nocheLinea : Look.escarcha) : fila.hovered ? Look.nocheAlta : "transparent"

        Sombra {
            id: lift

            active: fila.lit && !fila.off && lift.hondo
            kind: "fila"
        }
    }

    HoverHandler {
        id: hover
    }

    TapHandler {
        onTapped: {
            fila.forceActiveFocus(Qt.MouseFocusReason);
            if (fila.kind !== "elige" && fila.kind !== "riel")
                fila.act();
        }
    }

    RowLayout {
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: fila.base
        anchors.leftMargin: Look.filaLado
        anchors.rightMargin: Look.filaFin
        spacing: Look.e4

        Column {
            id: etiqueta

            Layout.fillWidth: true
            spacing: 2

            Text {
                width: parent.width
                text: fila.text
                wrapMode: Text.WordWrap
                font.family: Look.letraTitulo
                font.pixelSize: Look.tFila
                font.weight: fila.lit && !fila.off ? Font.Bold : Font.Medium
                font.letterSpacing: -0.01 * Look.tFila
                font.wordSpacing: 0.06 * Look.tFila
                color: fila.ink
            }

            Text {
                width: parent.width
                visible: fila.note !== ""
                text: fila.note
                wrapMode: Text.WordWrap
                font.family: Look.letraTexto
                font.pixelSize: Look.tPie
                font.weight: fila.lit && !fila.off ? Font.Bold : Font.Normal
                color: Tono.of(fila.noteTone, fila.lit && !fila.off, fila.soft)
            }
        }

        FilaValor {
            id: valor

            fila: fila
        }
    }

    FilaExplica {
        id: side

        fila: fila
    }
}
