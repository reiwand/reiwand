import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

Column {
    id: detailsFold

    property string text: App.tr("Details")
    property string details
    property bool open: false

    spacing: Look.e2

    QQC2.AbstractButton {
        id: fold

        implicitWidth: summary.implicitWidth
        implicitHeight: summary.implicitHeight
        Accessible.role: Accessible.Button
        Accessible.name: detailsFold.text
        Accessible.checkable: true
        Accessible.checked: detailsFold.open
        onClicked: detailsFold.open = !detailsFold.open

        Row {
            id: summary

            spacing: Look.listGap

            IconImage {
                anchors.verticalCenter: parent.verticalCenter
                name: "caret-right"
                sourceSize: Qt.size(Look.noteIcon, Look.noteIcon)
                color: Look.mist
                rotation: detailsFold.open ? 90 : 0

                Behavior on rotation {
                    enabled: detailsFold.visible
                    NumberAnimation {
                        duration: Look.pace
                        easing.type: Easing.BezierSpline
                        easing.bezierCurve: Look.paceCurve
                    }
                }
            }

            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: detailsFold.text
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

    Rectangle {
        visible: detailsFold.open
        width: parent.width
        implicitHeight: technical.implicitHeight + 2 * 10
        radius: Look.radiusSmall
        color: Look.nightHigh

        Area {
            id: technical

            anchors.fill: parent
            readOnly: true
            selectByMouse: true
            activeFocusOnTab: false
            leftPadding: Look.e3
            rightPadding: Look.e3
            topPadding: 10
            bottomPadding: 10
            palette.mid: Look.nightLine
            palette.highlight: Look.moon
            text: detailsFold.details
            wrapMode: Text.WrapAnywhere
            font.family: Look.fixedFont
            font.pixelSize: Look.tKey
            color: Look.mist
            Accessible.name: detailsFold.text
        }
    }
}
