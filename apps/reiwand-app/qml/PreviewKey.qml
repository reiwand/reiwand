import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Rectangle {
    id: cap

    property string key

    implicitWidth: Math.max(Math.round(PreviewLook.tKey * 2.1), word.implicitWidth + Look.e3)
    implicitHeight: Math.round(PreviewLook.tKey * 1.9)
    radius: Look.keyRadius
    color: Kirigami.Theme.alternateBackgroundColor
    border.width: 1
    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.3)
    Accessible.ignored: true

    Text {
        id: word

        anchors.centerIn: parent
        text: cap.key === "Intro" ? App.trc("key", "Enter") : cap.key
        color: Kirigami.Theme.textColor
        font.family: Look.textFont
        font.pixelSize: PreviewLook.tKey
        font.weight: Font.Medium
    }
}
