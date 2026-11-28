import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

Item {
    id: cardItem

    property string title
    property string caption
    property string tone
    property string icon
    property bool fields
    property alias middle: mid.data
    default property alias actions: row.data
    property bool stacked: false

    function decide() {
        stacked = fields && heading.implicitWidth + mid.implicitWidth + row.implicitWidth + 2 * Look.e5 > width;
    }

    implicitHeight: stacked ? Look.e4 + words.implicitHeight + Look.e3 + mid.implicitHeight + Look.e3 : Math.max(words.implicitHeight, row.implicitHeight, fields ? mid.implicitHeight : 0) + Look.e4
    onWidthChanged: Qt.callLater(decide)
    onFieldsChanged: decide()
    Accessible.role: Accessible.Grouping
    Accessible.name: title

    Rectangle {
        width: parent.width
        height: Look.line
        color: Look.nightLine
    }

    Column {
        id: words

        y: cardItem.stacked ? Look.e4 : Look.e4 + (parent.height - Look.e4 - height) / 2
        width: cardItem.fields ? heading.implicitWidth : parent.width - row.width - Look.e5

        Text {
            id: heading

            width: parent.width
            text: cardItem.title
            elide: Text.ElideRight
            font.family: Look.titleFont
            font.pixelSize: Look.tFocus
            font.weight: Font.Bold
            font.letterSpacing: Look.tFocus * Look.tight
            color: Look.frost
        }

        Note {
            visible: cardItem.caption !== "" && !cardItem.fields
            text: cardItem.caption
            tone: cardItem.tone
            icon: cardItem.icon
        }
    }

    Row {
        id: mid

        visible: cardItem.fields
        onImplicitWidthChanged: Qt.callLater(cardItem.decide)
        x: cardItem.stacked ? 0 : heading.implicitWidth + Look.e5
        y: cardItem.stacked ? words.y + words.height + Look.e3 : Look.e4 + (parent.height - Look.e4 - height) / 2
        spacing: Look.e4
    }

    RowLayout {
        id: row

        x: parent.width - width
        y: cardItem.stacked ? words.y + (words.height - height) / 2 : Look.e4 + (parent.height - Look.e4 - height) / 2
        spacing: Look.e3
    }
}
