import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.impl
import io.github.reiwand.app

Rectangle {
    id: place

    property string icon
    property string title
    property string note
    property bool lit: false
    property bool off: false
    property bool last: false
    default property alias body: content.data

    signal picked

    Layout.fillWidth: true
    implicitHeight: column.implicitHeight + 2 * Look.e3
    radius: Look.radius
    color: lit ? Look.nightHigh : Look.night
    border.width: Look.line
    border.color: lit ? Look.frost : Look.nightLine
    opacity: off ? 0.6 : 1
    activeFocusOnTab: !off
    Accessible.role: Accessible.RadioButton
    Accessible.name: title
    Accessible.description: note
    Accessible.checked: lit
    Keys.onSpacePressed: !off && picked()

    TapHandler {
        enabled: !place.off
        onTapped: place.picked()
    }

    FocusHalo {
        corner: place.radius
        visible: place.activeFocus
    }

    ColumnLayout {
        id: column

        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.margins: Look.e3
        spacing: Look.e2

        RowLayout {
            Layout.fillWidth: true
            spacing: Look.e3

            IconImage {
                name: place.icon
                sourceSize: Qt.size(PublishLook.targetIcon, PublishLook.targetIcon)
                color: place.lit ? Look.frost : Look.mist
            }

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 0

                Text {
                    Layout.fillWidth: true
                    text: place.title
                    elide: Text.ElideRight
                    font.family: Look.textFont
                    font.pixelSize: Look.tBody
                    font.weight: Font.DemiBold
                    color: Look.frost
                }

                Text {
                    Layout.fillWidth: true
                    visible: place.note !== ""
                    text: place.note
                    wrapMode: Text.WordWrap
                    font.family: Look.textFont
                    font.pixelSize: Look.tFoot
                    color: Look.mist
                }
            }

            Text {
                visible: place.last
                text: App.tr("Last time")
                font.family: Look.textFont
                font.pixelSize: Look.tFoot
                color: Look.sprout
            }
        }

        ColumnLayout {
            id: content

            Layout.fillWidth: true
            Layout.leftMargin: PublishLook.targetIcon + Look.e3
            visible: place.lit && children.length > 0
            spacing: Look.e2
        }
    }
}
