import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Column {
    id: empty

    property string icon
    property string title
    property string note
    property string action
    signal activated

    spacing: Look.e3
    Accessible.role: Accessible.Pane
    Accessible.name: title + ". " + note

    Kirigami.Icon {
        anchors.horizontalCenter: parent.horizontalCenter
        width: Look.iconoError
        height: width
        source: empty.icon
        color: Kirigami.Theme.textColor
    }

    Text {
        anchors.horizontalCenter: parent.horizontalCenter
        text: empty.title
        color: Kirigami.Theme.textColor
        font.family: Look.letraTitulo
        font.pixelSize: PreviewLook.tNombre
        font.weight: Font.DemiBold
    }

    Text {
        anchors.horizontalCenter: parent.horizontalCenter
        width: Math.min(implicitWidth, empty.width)
        text: empty.note
        horizontalAlignment: Text.AlignHCenter
        wrapMode: Text.WordWrap
        color: Qt.alpha(Kirigami.Theme.textColor, 0.78)
        font.family: Look.letraTexto
        font.pixelSize: PreviewLook.tSub
    }

    QQC2.Button {
        anchors.horizontalCenter: parent.horizontalCenter
        text: empty.action
        highlighted: true
        onClicked: empty.activated()
    }
}
