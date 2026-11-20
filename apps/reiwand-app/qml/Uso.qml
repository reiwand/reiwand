import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

Rectangle {
    id: uso

    property string screen
    property string list
    property bool pointed
    readonly property bool named: list !== ""
    readonly property bool hinted: pointed && visible && !named
    readonly property alias hint: hint
    readonly property string words: named ? list : screen !== "" ? App.tr("On %1", App.screens(screen.split(", "))) : App.tr("In use")

    implicitWidth: named ? row.implicitWidth + Look.e2 + Look.marcaTapa : implicitHeight
    implicitHeight: Look.usoAlto
    radius: named ? Look.radioChico : height / 2
    color: Look.noche
    Accessible.role: Accessible.StaticText
    Accessible.name: words

    QQC2.ToolTip {
        id: hint

        parent: uso
        visible: uso.hinted
        text: uso.words
    }

    Row {
        id: row

        x: uso.named ? Look.e2 : Math.round((uso.width - width) / 2)
        anchors.verticalCenter: parent.verticalCenter
        spacing: Look.usoHueco

        IconImage {
            anchors.verticalCenter: parent.verticalCenter
            width: Look.icono
            height: Look.icono
            sourceSize.width: width
            sourceSize.height: height
            name: uso.named ? "playlist" : "check-circle-fill"
            color: Look.luna
        }

        Text {
            visible: uso.named
            anchors.verticalCenter: parent.verticalCenter
            text: uso.list
            font.family: Look.letraTexto
            font.pixelSize: Look.tTecla
            font.weight: Font.DemiBold
            color: Look.escarcha
        }
    }
}
