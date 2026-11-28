import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// A number in a narrow field: at rest it shows from its first digit, never the tail of a long
// number scrolled into view; when it does not fit, the whole value is in the tooltip.
QQC2.TextField {
    id: box

    required property var number
    readonly property bool known: typeof number === "number" && isFinite(number)
    property bool typed: false

    signal chosen(string typed)

    function shown() {
        return typeof number === "number" && isFinite(number) ? String(Number(number.toFixed(4))) : "";
    }

    function rest() {
        if (!activeFocus) {
            text = shown();
            cursorPosition = 0;
        }
    }

    text: shown()
    QQC2.ToolTip.text: shown()
    QQC2.ToolTip.visible: hovered && !activeFocus && known && contentWidth > width - leftPadding - rightPadding
    onNumberChanged: rest()
    onActiveFocusChanged: rest()
    onTextEdited: typed = true
    onEditingFinished: {
        if (typed)
            chosen(text);
        typed = false;
        rest();
    }
    Component.onCompleted: cursorPosition = 0
}
