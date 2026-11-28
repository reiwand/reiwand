import QtQuick
import QtQuick.Controls.impl
import io.github.reiwand.app

Rectangle {
    id: chooseMark

    property bool chosen

    implicitWidth: Look.chooseSide
    implicitHeight: Look.chooseSide
    radius: width / 2
    border.width: Look.ring
    border.color: Look.frost
    color: chosen ? Look.frost : Qt.rgba(Look.night.r, Look.night.g, Look.night.b, Look.chooseVeil)
    Accessible.role: Accessible.CheckBox
    Accessible.checkable: true
    Accessible.checked: chosen
    Accessible.name: App.tr("Chosen")

    IconImage {
        visible: chooseMark.chosen
        anchors.centerIn: parent
        width: Look.icon
        height: Look.icon
        sourceSize.width: width
        sourceSize.height: height
        name: "check-bold"
        color: Look.night
    }
}
