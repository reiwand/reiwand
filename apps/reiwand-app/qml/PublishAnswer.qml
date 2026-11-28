import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

RowLayout {
    id: answer

    property string text
    property var value: undefined

    signal said(bool yes)

    spacing: Look.e2

    Text {
        Layout.fillWidth: true
        text: answer.text
        wrapMode: Text.WordWrap
        font.family: Look.textFont
        font.pixelSize: Look.tBody
        color: Look.frost
    }

    QQC2.Button {
        text: App.tr("Yes")
        checkable: true
        checked: answer.value === true
        onClicked: answer.said(true)
    }

    QQC2.Button {
        text: App.tr("No")
        checkable: true
        checked: answer.value === false
        onClicked: answer.said(false)
    }
}
