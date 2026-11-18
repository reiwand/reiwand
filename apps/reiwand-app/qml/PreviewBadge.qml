import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Rectangle {
    id: badge

    property string text
    property string icon

    implicitWidth: row.implicitWidth + Look.e3
    implicitHeight: Math.round(PreviewLook.tInsignia * 2)
    radius: height / 2
    color: Kirigami.Theme.backgroundColor
    border.width: 1
    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.25)
    Accessible.ignored: true

    Row {
        id: row

        anchors.centerIn: parent
        spacing: Look.e1

        Kirigami.Icon {
            anchors.verticalCenter: parent.verticalCenter
            width: PreviewLook.tInsignia + 2
            height: width
            source: badge.icon
            color: Kirigami.Theme.highlightColor
        }

        Text {
            anchors.verticalCenter: parent.verticalCenter
            text: badge.text
            color: Kirigami.Theme.textColor
            font.family: Look.letraTexto
            font.pixelSize: PreviewLook.tInsignia
            font.weight: Font.Medium
        }
    }
}
