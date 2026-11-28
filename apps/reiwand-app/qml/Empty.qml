import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Column {
    id: empty

    property string title
    property string text
    property string action
    property string actionIcon
    property bool small: false
    readonly property alias button: button

    signal acted

    width: Look.emptyWidth
    spacing: Look.e3
    Accessible.role: Accessible.Grouping
    Accessible.name: title

    Text {
        width: parent.width
        text: empty.title
        wrapMode: Text.WordWrap
        lineHeight: 1.1
        font.family: Look.titleFont
        font.pixelSize: empty.small ? Look.tRow : Look.tPage
        font.weight: Font.Bold
        font.letterSpacing: empty.small ? -0.015 * Look.tRow : -0.03 * Look.tPage
        color: Look.frost
    }

    Text {
        width: Math.min(parent.width, 62 * font.pixelSize * 0.5)
        visible: empty.text !== ""
        text: empty.text
        wrapMode: Text.WordWrap
        lineHeight: 1.3
        font.family: Look.textFont
        font.pixelSize: empty.small ? Look.tBody : Look.tExplain
        color: Look.mist
    }

    Item {
        width: 1
        height: Look.e3
        visible: empty.action !== ""
    }

    QQC2.Button {
        id: button

        visible: empty.action !== ""
        highlighted: true
        text: empty.action
        icon.name: empty.actionIcon
        onClicked: empty.acted()
    }
}
