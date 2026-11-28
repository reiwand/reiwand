import QtQuick
import QtQuick.Layouts
import "../look"

RowLayout {
    property bool paused: false

    spacing: Look.e1 + 2

    Glyph {
        name: parent.paused ? "pause-fill" : "play-fill"
        width: Look.noteIcon
        height: width
        tone: parent.paused ? Look.amber : Look.mist
    }

    Words {
        text: parent.paused ? i18n("Paused") : i18n("Playing")
        soft: true
        font.pixelSize: Look.tFoot
    }
}
