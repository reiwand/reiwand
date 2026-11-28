import QtQuick
import QtQuick.Controls.impl
import io.github.reiwand.app

Rectangle {
    id: step

    property string name
    property bool held: false
    property bool over: false
    property bool live: true

    implicitWidth: Look.toolHeight
    implicitHeight: Look.controlHeight
    radius: Look.radiusSmall
    color: !live ? "transparent" : held ? Look.night : over ? Look.nightLine : "transparent"

    IconImage {
        anchors.centerIn: parent
        width: Look.icon
        height: Look.icon
        sourceSize: Qt.size(Look.icon, Look.icon)
        name: step.name
        color: !step.live ? Look.mistLow : step.over ? Look.frost : Look.mist
    }
}
