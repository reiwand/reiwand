import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

RowLayout {
    id: valueBox

    required property Item row
    readonly property bool lit: row.lit && !row.off
    readonly property color quiet: lit ? Look.night : row.off ? (row.lit ? Look.mist : Look.mistLow) : Look.mistLow
    readonly property color dim: Qt.rgba(Look.night.r, Look.night.g, Look.night.b, 0.25)

    function begin() {
        if (row.kind === "field")
            field.forceActiveFocus(Qt.TabFocusReason);
        else
            capture.listen();
    }

    function back(item) {
        item.focus = false;
        row.forceActiveFocus(Qt.TabFocusReason);
    }

    Layout.alignment: Qt.AlignVCenter
    spacing: Look.valueGap

    component Caret: IconImage {
        required property string glyph
        property int by: 0

        name: glyph
        sourceSize: Qt.size(Look.icon, Look.icon)
        color: valueBox.quiet

        TapHandler {
            enabled: parent.by !== 0
            onTapped: {
                valueBox.row.forceActiveFocus(Qt.MouseFocusReason);
                valueBox.row.step(parent.by);
            }
        }
    }

    component Words: Text {
        font.family: Look.textFont
        font.pixelSize: Look.tValue
        font.weight: valueBox.lit ? Font.Bold : Font.Medium
        color: valueBox.row.soft
    }

    Caret {
        visible: valueBox.row.kind === "choice"
        glyph: "caret-left"
        by: -1
    }

    QQC2.Slider {
        visible: valueBox.row.kind === "slider"
        implicitWidth: Look.railWidth
        focusPolicy: Qt.NoFocus
        enabled: !valueBox.row.off
        from: valueBox.row.from
        to: valueBox.row.to
        stepSize: valueBox.row.stepSize
        value: valueBox.row.at
        palette.dark: valueBox.lit ? Look.night : Look.frost
        palette.midlight: valueBox.lit ? valueBox.dim : Look.nightLine
        palette.window: valueBox.lit ? Look.night : Look.frost
        palette.light: valueBox.lit ? Look.night : Look.frost
        palette.mid: valueBox.lit ? Look.night : Look.frost
        Accessible.ignored: true
        onMoved: {
            valueBox.row.at = value;
            valueBox.row.moved(value);
        }
    }

    Words {
        visible: valueBox.row.kind === "slider" || valueBox.row.kind === "expand" ? false : text !== ""
        text: valueBox.row.kind === "choice" ? valueBox.row.options[valueBox.row.current] ?? "" : valueBox.row.value
    }

    ListRowWords {
        row: valueBox.row
        lit: valueBox.lit
    }

    Words {
        visible: valueBox.row.number !== ""
        text: valueBox.row.number
        horizontalAlignment: Text.AlignRight
        Layout.minimumWidth: 3.2 * Look.tValue * 0.55
        color: valueBox.lit ? Look.night : valueBox.row.off ? valueBox.row.soft : Look.frost
    }

    Rectangle {
        visible: valueBox.row.kind === "switch"
        implicitWidth: Look.switchWidth
        implicitHeight: Look.switchHeight
        radius: height / 2
        color: valueBox.lit ? (valueBox.row.checked ? Look.night : valueBox.dim) : valueBox.row.checked ? Look.frost : Look.nightLine

        Rectangle {
            readonly property real gap: (parent.height - height) / 2

            x: valueBox.row.checked ? parent.width - width - gap : gap
            y: gap
            width: Look.switchDot
            height: Look.switchDot
            radius: height / 2
            color: valueBox.lit ? (valueBox.row.checked ? Look.frost : Look.night) : valueBox.row.checked ? Look.night : Look.mist

            Behavior on x {
                enabled: valueBox.visible
                NumberAnimation {
                    duration: Look.pace
                    easing.type: Easing.BezierSpline
                    easing.bezierCurve: Look.paceCurve
                }
            }
        }
    }

    QQC2.TextField {
        id: field

        visible: valueBox.row.kind === "field"
        implicitWidth: Math.min(Look.fieldWidth, Math.round(valueBox.row.width / 2))
        implicitHeight: Look.controlHeight
        activeFocusOnTab: false
        enabled: !valueBox.row.off
        text: valueBox.row.fieldText
        placeholderText: valueBox.row.placeholder
        font.family: Look.textFont
        font.pixelSize: Look.tBody
        color: valueBox.lit ? Look.night : Look.frost
        placeholderTextColor: valueBox.lit ? Look.night : Look.mist
        palette.base: valueBox.lit ? "transparent" : Look.nightHigh
        palette.text: valueBox.lit ? Look.night : Look.frost
        palette.placeholderText: valueBox.lit ? Look.night : Look.mist
        palette.mid: valueBox.lit ? Qt.rgba(Look.night.r, Look.night.g, Look.night.b, 0.35) : Look.nightLine
        palette.highlight: valueBox.lit ? Look.night : Look.moon
        selectedTextColor: valueBox.lit ? Look.frost : Look.night
        Accessible.name: valueBox.row.text

        background: Rectangle {
            radius: Look.radiusSmall
            color: valueBox.lit ? "transparent" : Look.nightHigh
            border.width: 1
            border.color: valueBox.lit ? Qt.rgba(Look.night.r, Look.night.g, Look.night.b, 0.35) : Look.nightLine
        }
        Keys.onEscapePressed: valueBox.back(field)
        Keys.onReturnPressed: valueBox.back(field)
        Keys.onEnterPressed: valueBox.back(field)
        Keys.onUpPressed: valueBox.row.nextItemInFocusChain(false)?.forceActiveFocus(Qt.TabFocusReason)
        Keys.onDownPressed: valueBox.row.nextItemInFocusChain(true)?.forceActiveFocus(Qt.TabFocusReason)
        onEditingFinished: {
            if (text !== valueBox.row.fieldText) {
                valueBox.row.fieldText = text;
                valueBox.row.edited(text);
            }
        }
    }

    KeyCapture {
        id: capture

        visible: valueBox.row.kind === "capture"
        keys: valueBox.row.keys
        lit: valueBox.lit
        onCaptured: combination => {
            valueBox.row.keys = combination;
            valueBox.row.captured(combination);
        }
        onDone: valueBox.back(capture)
    }

    Caret {
        visible: valueBox.row.kind === "choice" || valueBox.row.kind === "page" || valueBox.row.kind === "expand"
        glyph: valueBox.row.kind === "expand" && valueBox.row.open ? "caret-down" : "caret-right"
        by: valueBox.row.kind === "choice" ? 1 : 0
    }
}
