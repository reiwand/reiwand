import QtQuick
import io.github.reiwand.app

Row {
    id: thumbs

    property string source
    property bool stale: false

    spacing: Look.e2
    Component.onCompleted: {
        model.search("source:" + source + " missing:no", "", false, "");
        model.visible(0, Look.sourcePhotos - 1);
    }

    ItemModel {
        id: model

        size: 256
        onReloaded: model.visible(0, Look.sourcePhotos - 1)
    }

    Repeater {
        model: model

        delegate: Item {
            required property int index
            required property string thumb

            visible: index < Look.sourcePhotos
            width: visible ? height * 16 / 9 : 0
            height: thumbs.height

            Rectangle {
                anchors.fill: parent
                radius: Look.radiusSmall
                color: Look.nightHigh
                clip: true

                Image {
                    anchors.fill: parent
                    source: parent.parent.thumb
                    asynchronous: true
                    fillMode: Image.PreserveAspectCrop
                }

                Rectangle {
                    anchors.fill: parent
                    color: Look.night
                    opacity: thumbs.stale ? Look.sourceVeil * 2 : Look.sourceVeil
                }
            }
        }
    }
}
