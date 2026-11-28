import QtQuick
import QtQuick.Controls.impl
import QtQuick.Templates as T
import io.github.reiwand.app

T.Button {
    id: control

    readonly property bool primary: highlighted
    readonly property bool word: flat && !highlighted
    readonly property color face: palette.highlight
    readonly property color deep: Qt.colorEqual(face, Look.moon) ? Look.moonDeep : control.mix(face, Look.night, 0.2)
    readonly property color ink: !enabled ? (primary ? Look.mist : Look.mistLow) : primary ? Look.night : Look.frost

    property bool settling: false
    property bool reshown: false
    property bool placed: false

    function regrip() {
        if (!placed)
            return;
        if (!visible) {
            reshown = true;
            return;
        }
        settling = true;
        calm.restart();
    }

    function mix(a, b, amount) {
        return Qt.rgba(a.r + (b.r - a.r) * amount, a.g + (b.g - a.g) * amount, a.b + (b.b - a.b) * amount, 1);
    }

    onTextChanged: regrip()
    onVisibleChanged: {
        if (visible && reshown) {
            reshown = false;
            regrip();
        }
    }
    Component.onCompleted: placed = true
    Keys.onPressed: event => {
        if (settling && [Qt.Key_Return, Qt.Key_Enter, Qt.Key_Space, Qt.Key_Select].includes(event.key))
            event.accepted = true;
    }

    Timer {
        id: calm

        interval: 500
        onTriggered: control.settling = false
    }

    z: visualFocus ? 1 : 0
    implicitWidth: Math.max(implicitBackgroundWidth + leftInset + rightInset, implicitContentWidth + leftPadding + rightPadding)
    implicitHeight: Math.max(implicitBackgroundHeight + topInset + bottomInset, implicitContentHeight + topPadding + bottomPadding)
    leftPadding: word ? 2 : Look.e4
    rightPadding: word ? 2 : primary ? 20 : Look.e4
    topPadding: 0
    bottomPadding: 0
    spacing: 9
    font.family: primary ? Look.titleFont : Look.textFont
    font.pixelSize: primary ? Look.tButton : Look.tBody
    font.weight: primary ? Font.Bold : word ? Font.DemiBold : Font.Medium
    font.underline: word && enabled && hovered && !visualFocus
    icon.width: 18
    icon.height: 18
    icon.color: word ? (enabled ? Look.mist : Look.mistLow) : ink
    transform: Translate {
        y: control.down && !control.word ? 1 : 0
    }

    contentItem: IconLabel {
        spacing: control.spacing
        mirrored: control.mirrored
        display: control.display
        icon: control.icon
        text: control.text
        font: control.font
        color: control.ink
    }

    background: Rectangle {
        implicitHeight: Look.controlHeight
        radius: Look.radius
        border.width: control.primary || control.word ? 0 : Look.line
        border.color: control.checked ? Look.mist : Look.nightLine
        color: {
            if (control.word)
                return "transparent";
            if (control.primary)
                return !control.enabled ? Look.nightLine : control.down ? control.deep : control.hovered ? control.mix(control.face, Look.frost, 0.12) : control.face;
            if (!control.enabled)
                return "transparent";
            return control.down ? Look.night : control.hovered || control.checked ? Look.nightLine : Look.nightHigh;
        }

        MouseArea {
            anchors.fill: parent
            enabled: control.settling
        }

        FocusRing {
            corner: Look.radius
            visible: control.visualFocus
        }
    }
}
