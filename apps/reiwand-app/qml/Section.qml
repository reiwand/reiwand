import QtQuick
import io.github.reiwand.app

Text {
    topPadding: Look.e5
    leftPadding: Look.rowSide
    font.family: Look.titleFont
    font.pixelSize: Look.tBody
    font.weight: Font.DemiBold
    color: Look.mist
    Accessible.role: Accessible.Heading
    Accessible.name: text
}
