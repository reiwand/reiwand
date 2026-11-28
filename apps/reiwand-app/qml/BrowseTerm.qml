import QtQuick
import io.github.reiwand.app

Text {
    property bool lit
    property bool small

    textFormat: Text.PlainText
    wrapMode: Text.WordWrap
    font.family: Look.textFont
    font.pixelSize: small ? Look.tFoot : Look.tBody
    color: lit ? Look.frost : Look.mist
}
