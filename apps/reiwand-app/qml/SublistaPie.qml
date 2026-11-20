import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Item {
    id: pie

    required property Item sublista

    x: Look.filaLado
    visible: pie.sublista.addText !== ""
    width: pie.sublista.width - Look.filaLado
    implicitHeight: Look.altoSublista

    Rectangle {
        width: Look.linea
        height: parent.height
        color: Look.nocheLinea
    }

    QQC2.Button {
        x: Look.e4
        anchors.verticalCenter: parent.verticalCenter
        flat: true
        text: pie.sublista.addText
        icon.name: "plus"
        Keys.onUpPressed: nextItemInFocusChain(false)?.forceActiveFocus(Qt.TabFocusReason)
        Keys.onDownPressed: nextItemInFocusChain(true)?.forceActiveFocus(Qt.TabFocusReason)
        onClicked: pie.sublista.added()
    }

    QQC2.Button {
        visible: pie.sublista.otherText !== ""
        anchors.right: parent.right
        anchors.rightMargin: Look.filaFin
        anchors.verticalCenter: parent.verticalCenter
        flat: true
        text: pie.sublista.otherText
        icon.name: pie.sublista.otherIcon
        onClicked: pie.sublista.otherAdded()
    }
}
