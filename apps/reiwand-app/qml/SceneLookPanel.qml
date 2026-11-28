import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The See as panel: the preview as the daemon draws it on a device, on battery or with reduced
// motion, the two levels to compare and the effects on that device (the canvas bar's "See as").
Rectangle {
    id: look

    required property var page

    color: Look.night
    clip: true

    QQC2.ScrollView {
        id: scroll

        // The rows sit where they did, with room inside the scroll for their focus rings.
        anchors.fill: parent
        anchors.margins: Look.e3 - Look.ringRoom
        contentWidth: availableWidth
        contentHeight: shown.implicitHeight + 2 * Look.ringRoom

        ScenePreviewPanel {
            id: shown

            x: Look.ringRoom
            y: Look.ringRoom
            width: scroll.availableWidth - 2 * Look.ringRoom
            page: look.page
        }
    }
}
