import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Column {
    id: caption

    property string title
    property string detail

    spacing: Look.e1
    Accessible.role: Accessible.StaticText
    Accessible.name: detail === "" ? title : title + ", " + detail

    Text {
        anchors.horizontalCenter: parent.horizontalCenter
        width: Math.min(implicitWidth, caption.width)
        text: caption.title
        elide: Text.ElideRight
        color: Kirigami.Theme.textColor
        font.family: Look.titleFont
        font.pixelSize: PreviewLook.tName
        font.weight: Font.DemiBold
    }

    Text {
        anchors.horizontalCenter: parent.horizontalCenter
        text: caption.detail
        color: Qt.alpha(Kirigami.Theme.textColor, 0.78)
        font.family: Look.textFont
        font.pixelSize: PreviewLook.tSub
    }
}
