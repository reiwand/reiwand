import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Rectangle {
    id: cap

    property string key

    implicitWidth: Math.max(Math.round(PreviewLook.tTecla * 2.1), word.implicitWidth + Look.e3)
    implicitHeight: Math.round(PreviewLook.tTecla * 1.9)
    radius: Look.teclaRadio
    color: Kirigami.Theme.alternateBackgroundColor
    border.width: 1
    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.3)
    Accessible.ignored: true

    Text {
        id: word

        anchors.centerIn: parent
        text: cap.key
        color: Kirigami.Theme.textColor
        font.family: Look.letraTexto
        font.pixelSize: PreviewLook.tTecla
        font.weight: Font.Medium
    }
}
