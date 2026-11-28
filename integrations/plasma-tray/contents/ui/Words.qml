import QtQuick
import "../look"

Text {
    property bool title: false
    property bool soft: false

    color: soft ? Look.mist : Look.frost
    font.family: title ? Look.titleFont : Look.textFont
    font.pixelSize: Look.tBody
    font.weight: title ? Font.Bold : Font.Normal
    textFormat: Text.PlainText
    elide: wrapMode === Text.NoWrap ? Text.ElideRight : Text.ElideNone
}
