import QtQuick
import io.github.reiwand.app

Row {
    spacing: Look.e2
    Accessible.role: Accessible.StaticText
    Accessible.name: "Reiwand"

    Rectangle {
        anchors.verticalCenter: parent.verticalCenter
        width: Look.logoMoon
        height: width
        radius: width / 2
        color: Look.frost
    }

    Text {
        text: "Reiwand"
        color: Look.frost
        font.family: Look.titleFont
        font.pixelSize: Look.logoLetter
        font.weight: Font.DemiBold
    }
}
