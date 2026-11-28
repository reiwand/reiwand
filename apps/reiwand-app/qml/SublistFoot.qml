import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Item {
    id: footText

    required property Item sublist

    x: Look.rowSide
    visible: footText.sublist.addText !== ""
    width: footText.sublist.width - Look.rowSide
    implicitHeight: Look.sublistHeight

    Rectangle {
        width: Look.line
        height: parent.height
        color: Look.nightLine
    }

    QQC2.Button {
        x: Look.e4
        anchors.verticalCenter: parent.verticalCenter
        flat: true
        text: footText.sublist.addText
        icon.name: "plus"
        Keys.onUpPressed: nextItemInFocusChain(false)?.forceActiveFocus(Qt.TabFocusReason)
        Keys.onDownPressed: nextItemInFocusChain(true)?.forceActiveFocus(Qt.TabFocusReason)
        onClicked: footText.sublist.added()
    }

    QQC2.Button {
        visible: footText.sublist.otherText !== ""
        anchors.right: parent.right
        anchors.rightMargin: Look.rowEnd
        anchors.verticalCenter: parent.verticalCenter
        flat: true
        text: footText.sublist.otherText
        icon.name: footText.sublist.otherIcon
        onClicked: footText.sublist.otherAdded()
    }
}
