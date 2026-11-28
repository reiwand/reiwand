import QtQuick
import io.github.reiwand.app

Column {
    id: now

    property int sources: 0
    property int photos: 0

    spacing: Look.e2

    Text {
        anchors.right: parent.right
        text: App.tr("Sources")
        font.family: Look.titleFont
        font.pixelSize: Look.tCover
        font.weight: Font.Bold
        font.letterSpacing: -0.03 * Look.tCover
        color: Look.frost
    }

    Text {
        anchors.right: parent.right
        text: now.sources === 0 ? App.tr("Photos that renew themselves · none yet") : now.sources === 1 ? App.tr("Photos that renew themselves · 1 source · %1 photos", now.photos) : App.tr("Photos that renew themselves · %1 sources · %2 photos", now.sources, now.photos)
        font.family: Look.textFont
        font.pixelSize: Look.tBody
        color: Look.mist
    }

    Text {
        anchors.right: parent.right
        visible: now.sources === 0
        width: Look.emptyWidth
        horizontalAlignment: Text.AlignRight
        wrapMode: Text.WordWrap
        text: App.tr("Pick where the photos come from. They arrive in your library and you can rotate them with one button.")
        font.family: Look.textFont
        font.pixelSize: Look.tBody
        color: Look.mist
    }
}
