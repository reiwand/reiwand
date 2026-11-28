import QtQuick
import io.github.reiwand.app

Item {
    id: word

    required property int page
    required property bool apart
    required property bool current
    required property string text
    property bool lit: false
    readonly property bool hovered: hover.hovered

    signal clicked

    implicitWidth: Look.menuIndent + label.implicitWidth
    implicitHeight: apart ? Look.asideHeight : Look.menuHeight
    Accessible.role: Accessible.PageTab
    Accessible.name: text
    Accessible.selected: current

    HoverHandler {
        id: hover

        cursorShape: Qt.PointingHandCursor
    }

    TapHandler {
        onTapped: word.clicked()
    }

    Item {
        anchors.fill: parent

        Rectangle {
            anchors.verticalCenter: parent.verticalCenter
            width: Look.markWidth
            height: Look.markHeight
            radius: 1
            color: Look.moon
            visible: word.current
        }

        Text {
            id: label

            anchors.verticalCenter: parent.verticalCenter
            x: Look.menuIndent
            text: word.text
            color: word.current || word.hovered || word.lit ? Look.frost : Look.mist
            font.family: Look.titleFont
            font.pixelSize: word.apart ? Look.tAside : Look.tMenu
            font.weight: word.current ? Font.Bold : word.apart ? Font.Medium : Font.Normal
            font.letterSpacing: -0.015 * (word.apart ? Look.tAside : Look.tMenu)
        }

        Ring {
            anchors.leftMargin: Look.menuIndent - Look.e2 - Look.ringGap - Look.ring
            anchors.rightMargin: -Look.e2 - Look.ringGap - Look.ring
            visible: word.lit
        }
    }
}
