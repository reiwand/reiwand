import QtQuick
import io.github.reiwand.app

Row {
    spacing: Look.e2
    Accessible.role: Accessible.StaticText
    Accessible.name: "Reiwand"

    Rectangle {
        anchors.verticalCenter: parent.verticalCenter
        width: Look.logoLuna
        height: width
        radius: width / 2
        color: Look.escarcha
    }

    Text {
        text: "Reiwand"
        color: Look.escarcha
        font.family: Look.letraTitulo
        font.pixelSize: Look.logoLetra
        font.weight: Font.DemiBold
    }
}
