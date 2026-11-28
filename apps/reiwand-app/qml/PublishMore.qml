import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

ColumnLayout {
    id: more

    property string text: App.tr("More")
    property bool open: false
    default property alias body: inner.data

    Layout.fillWidth: true
    spacing: Look.e3

    QQC2.AbstractButton {
        id: fold

        implicitWidth: summary.implicitWidth
        implicitHeight: summary.implicitHeight
        Accessible.role: Accessible.Button
        Accessible.name: more.text
        Accessible.checkable: true
        Accessible.checked: more.open
        onClicked: more.open = !more.open

        Row {
            id: summary

            spacing: Look.listGap

            IconImage {
                anchors.verticalCenter: parent.verticalCenter
                name: "caret-right"
                sourceSize: Qt.size(Look.noteIcon, Look.noteIcon)
                color: Look.mist
                rotation: more.open ? 90 : 0
            }

            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: more.text
                font.family: Look.textFont
                font.pixelSize: Look.tFoot
                font.weight: Font.DemiBold
                color: Look.mist
            }
        }

        FocusHalo {
            corner: Look.radiusSmall
            visible: fold.visualFocus
        }
    }

    RowLayout {
        Layout.fillWidth: true
        visible: more.open && inner.children.length > 0
        spacing: Look.e4

        Rectangle {
            Layout.fillHeight: true
            implicitWidth: Look.line
            color: Look.nightLine
        }

        ColumnLayout {
            id: inner

            Layout.fillWidth: true
            spacing: Look.e3
        }
    }
}
