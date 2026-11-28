import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// Over a canvas paused at second 0 whose layers are all particles that start empty: the black preview is
// the start of the scene, not a failure. Goes away with Play or once the playhead moves.
Rectangle {
    id: hint

    required property var page

    visible: (page.view.unborn ?? false) && !page.editor.playing && page.clock === 0
    width: Math.min(parent.width - Look.e4 * 2, row.implicitWidth + Look.e3 * 2)
    height: row.implicitHeight + Look.e2 * 2
    radius: Look.e2
    color: Look.night
    border.color: Look.nightLine

    RowLayout {
        id: row

        anchors.fill: parent
        anchors.margins: Look.e2
        anchors.leftMargin: Look.e3
        anchors.rightMargin: Look.e3
        spacing: Look.e3

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: App.tr("Paused at the start: nothing has appeared yet. Play to see it.")
        }

        QQC2.Button {
            icon.name: "media-playback-start"
            text: App.tr("Play")
            onClicked: {
                hint.page.clock = 0;
                hint.page.editor.seek(0, true);
            }
        }
    }
}
