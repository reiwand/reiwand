import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

Column {
    id: cardItem

    property string title
    property string caption
    default property alias actions: row.data

    width: Math.min(implicitWidth, Look.coverFocus + Look.cover)
    Accessible.role: Accessible.Grouping
    Accessible.name: title

    Text {
        width: Math.min(implicitWidth, cardItem.width)
        text: cardItem.title
        elide: Text.ElideRight
        font.family: Look.titleFont
        font.pixelSize: Look.tFocus
        font.weight: Font.Bold
        font.letterSpacing: Look.tFocus * Look.tight
        color: Look.frost
    }

    Item {
        width: 1
        height: Look.footTop
    }

    Text {
        visible: cardItem.caption !== ""
        width: Math.min(implicitWidth, cardItem.width)
        text: cardItem.caption
        elide: Text.ElideRight
        font.family: Look.textFont
        font.pixelSize: Look.tBody
        color: Look.mist
    }

    Item {
        width: 1
        height: Look.actionsTop
        visible: row.children.length > 0
    }

    RowLayout {
        id: row

        spacing: Look.e3
    }
}
