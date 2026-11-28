import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

Item {
    id: fixedCard

    property string title
    property string caption
    default property alias actions: row.data

    implicitHeight: Math.max(words.implicitHeight, row.implicitHeight) + Look.e4
    Accessible.role: Accessible.Grouping
    Accessible.name: title

    Rectangle {
        width: parent.width
        height: Look.line
        color: Look.nightLine
    }

    Column {
        id: words

        visible: fixedCard.title !== ""
        y: Look.e4 + (parent.height - Look.e4 - height) / 2
        width: parent.width - row.width - Look.e5

        Text {
            width: parent.width
            text: fixedCard.title
            elide: Text.ElideRight
            font.family: Look.titleFont
            font.pixelSize: Look.tFocus
            font.weight: Font.Bold
            font.letterSpacing: Look.tFocus * Look.tight
            color: Look.frost
        }

        Text {
            visible: fixedCard.caption !== ""
            width: parent.width
            text: fixedCard.caption
            elide: Text.ElideRight
            font.family: Look.textFont
            font.pixelSize: Look.tBody
            color: Look.mist
        }
    }

    RowLayout {
        id: row

        x: fixedCard.title !== "" ? parent.width - width : 0
        width: fixedCard.title !== "" ? implicitWidth : fixedCard.width
        y: Look.e4 + (parent.height - Look.e4 - height) / 2
        spacing: Look.e3
    }
}
