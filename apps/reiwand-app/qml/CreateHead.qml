import QtQuick
import io.github.reiwand.app

Row {
    property string text
    property string small

    spacing: Look.e2
    Accessible.ignored: true

    Text {
        id: main

        text: parent.text
        font.family: Look.titleFont
        font.pixelSize: Look.tName
        font.weight: Font.DemiBold
        color: Look.frost
    }

    Text {
        anchors.baseline: main.baseline
        text: parent.small
        font.family: Look.textFont
        font.pixelSize: Look.tFoot
        color: Look.mist
    }
}
