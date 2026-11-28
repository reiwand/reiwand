import QtQuick
import io.github.reiwand.app

// A panel that is made the first time its unit shows and then kept, so the editor opens without making
// the panels no layout in use shows.
SceneUnit {
    id: unit

    property bool seen: false
    default property alias content: load.sourceComponent

    onVisibleChanged: seen = seen || visible

    Loader {
        id: load

        anchors.fill: parent
        active: unit.seen
    }
}
