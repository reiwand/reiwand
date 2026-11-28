import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.impl
import io.github.reiwand.app

Item {
    id: option

    property string text
    property string icon
    property string note
    property string tone
    property bool off: false
    property bool lit: false
    property string plan
    property bool current: false
    property real size: Look.tBody
    property int high: Look.optionHeight
    readonly property bool hovered: hover.hovered
    readonly property color ink: off ? Look.mistLow : lit ? Tone.of(tone, true, Look.night) : Tone.of(tone, false, Look.frost)

    signal clicked

    implicitHeight: high
    Accessible.role: Accessible.MenuItem
    Accessible.name: note !== "" ? text + ", " + note : text
    Accessible.focused: lit

    HoverHandler {
        id: hover
    }

    TapHandler {
        enabled: !option.off
        onTapped: option.clicked()
    }

    Rectangle {
        anchors.fill: parent
        radius: Look.radiusSmall
        color: option.lit && !option.off ? Look.frost : option.hovered && !option.off ? Look.nightHigh : "transparent"
    }

    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: Look.e3
        anchors.rightMargin: Look.e3
        spacing: Look.e3

        IconImage {
            visible: option.icon !== ""
            name: option.icon
            sourceSize: Qt.size(Look.icon, Look.icon)
            color: option.off ? Look.mistLow : option.lit ? option.ink : Tone.of(option.tone, false, Look.mist)
        }

        SceneBlueprint {
            visible: option.plan !== ""
            implicitWidth: DeskLook.planMenuWidth
            implicitHeight: DeskLook.planMenuHeight
            root: option.plan
            ink: option.lit ? Look.night : option.current ? Look.frost : Look.mist
        }

        Text {
            Layout.fillWidth: true
            text: option.text
            elide: Text.ElideRight
            font.family: Look.textFont
            font.pixelSize: option.size
            font.weight: (option.lit && !option.off) || option.current ? Font.DemiBold : Font.Normal
            color: option.ink
        }

        Text {
            visible: option.note !== ""
            text: option.note
            font.family: Look.textFont
            font.pixelSize: Math.min(Look.tFoot, option.size)
            color: option.off ? Look.mistLow : option.lit ? Look.night : Look.mist
        }
    }
}
