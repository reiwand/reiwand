import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

Rectangle {
    id: inUse

    property string screen
    property string list
    property bool pointed
    readonly property bool named: list !== ""
    readonly property bool hinted: pointed && visible && !named
    readonly property alias hint: hint
    readonly property string words: named ? list : screen !== "" ? App.tr("On %1", App.screens(screen.split(", "))) : App.tr("In use")

    implicitWidth: named ? row.implicitWidth + Look.e2 + Look.coverMark : implicitHeight
    implicitHeight: Look.inUseHeight
    radius: named ? Look.radiusSmall : height / 2
    color: Look.night
    Accessible.role: Accessible.StaticText
    Accessible.name: words

    QQC2.ToolTip {
        id: hint

        parent: inUse
        visible: inUse.hinted
        text: inUse.words
    }

    Row {
        id: row

        x: inUse.named ? Look.e2 : Math.round((inUse.width - width) / 2)
        anchors.verticalCenter: parent.verticalCenter
        spacing: Look.inUseGap

        IconImage {
            anchors.verticalCenter: parent.verticalCenter
            width: Look.icon
            height: Look.icon
            sourceSize.width: width
            sourceSize.height: height
            name: inUse.named ? "playlist" : "check-circle-fill"
            color: Look.moon
        }

        Text {
            visible: inUse.named
            anchors.verticalCenter: parent.verticalCenter
            text: inUse.list
            font.family: Look.textFont
            font.pixelSize: Look.tKey
            font.weight: Font.DemiBold
            color: Look.frost
        }
    }
}
