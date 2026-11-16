import QtQuick
import io.github.reiwand.app

Grid {
    id: mosaico

    property var pictures: []
    property bool off

    visible: pictures.length > 1
    columns: 2
    spacing: Look.linea * 2

    Repeater {
        model: mosaico.pictures.length > 1 ? mosaico.pictures.slice(0, 4) : []

        delegate: Foto {
            required property string modelData

            width: (mosaico.width - Look.linea * 2) / 2
            height: (mosaico.height - Look.linea * 2) / 2
            source: modelData
            sourceSize.width: Look.tapa
            opacity: mosaico.off ? Look.tapaApagada : 1
        }
    }
}
