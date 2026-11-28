import QtQuick
import io.github.reiwand.app

// the title and text of a SmallPanel; unpadded panels (menus) inset them like the options under them
Column {
    id: words

    required property var owner

    visible: owner.title !== "" || owner.text !== ""
    spacing: Look.e3

    Text {
        visible: words.owner.title !== ""
        width: parent.width
        leftPadding: words.owner.inset
        rightPadding: words.owner.inset
        topPadding: words.owner.padded ? 0 : Look.e3
        text: words.owner.title
        wrapMode: Text.WordWrap
        font.family: Look.titleFont
        font.pixelSize: Look.tRow
        font.weight: Font.Bold
        color: Look.frost
    }

    Text {
        visible: words.owner.text !== ""
        width: parent.width
        leftPadding: words.owner.inset
        rightPadding: words.owner.inset
        text: words.owner.text
        wrapMode: Text.WordWrap
        lineHeight: 1.3
        font.family: Look.textFont
        font.pixelSize: Look.tBody
        color: Look.mist
    }
}
