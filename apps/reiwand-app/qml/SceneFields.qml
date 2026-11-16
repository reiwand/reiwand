import QtQuick
import QtQuick.Layouts
import io.github.reiwand.app

Rectangle {
    id: fields

    required property var page

    color: Look.nocheAlta
    clip: true

    ColumnLayout {
        anchors.fill: parent
        anchors.leftMargin: Look.e4
        anchors.rightMargin: Look.e3
        anchors.topMargin: Look.e3
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
