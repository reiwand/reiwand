import QtQuick
import io.github.reiwand.app

Grid {
    id: mosaic

    property var pictures: []
    property bool off

    visible: pictures.length > 1
    columns: 2
    spacing: Look.line * 2

    Repeater {
        model: mosaic.pictures.length > 1 ? mosaic.pictures.slice(0, 4) : []

        delegate: Photo {
            required property string modelData

            width: (mosaic.width - Look.line * 2) / 2
            height: (mosaic.height - Look.line * 2) / 2
            source: modelData
            sourceSize.width: Look.cover
            opacity: mosaic.off ? Look.coverDimmed : 1
        }
    }
}
