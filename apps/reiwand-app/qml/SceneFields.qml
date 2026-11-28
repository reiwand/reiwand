import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

Rectangle {
    id: fields

    required property var page

    color: Look.night
    clip: true

    ColumnLayout {
        anchors.fill: parent
        anchors.leftMargin: Look.e3
        anchors.rightMargin: Look.e2
        anchors.topMargin: Look.e2
        anchors.bottomMargin: Look.e2
        spacing: Look.e2

        SceneInspector {
            Layout.fillWidth: true
            Layout.maximumWidth: parent.width
            Layout.fillHeight: true
            page: fields.page
        }
    }

    SceneTransitionPanel {
        anchors.fill: parent
        page: fields.page
    }
}
