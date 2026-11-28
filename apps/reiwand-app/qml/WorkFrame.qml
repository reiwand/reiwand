import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Item {
    id: frame

    required property var page
    required property string backWord
    readonly property alias seat: seat
    readonly property alias backButton: back
    readonly property bool own: !!(page && page.topBar)
    readonly property bool dense: own && page.dense === true
    readonly property real high: dense ? DeskLook.top : Look.topHeight
    readonly property point noticeAt: Qt.point(width - (page?.rightWide ?? 0) - Look.e4, high + (dense ? DeskLook.strip + Look.line : 0) + Look.canvasBarHeight + Look.e4)

    signal leave

    Rectangle {
        id: bar

        width: frame.width
        height: frame.high
        color: Look.night

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: frame.dense ? Look.e1 + 2 : Look.e4
            anchors.rightMargin: frame.dense ? Look.e2 + 2 : Look.e4
            spacing: frame.dense ? Look.e2 : Look.e4

            QQC2.Button {
                id: back

                flat: true
                font.pixelSize: frame.dense ? DeskLook.tWord : Look.tBody
                icon.name: "caret-left"
                text: frame.backWord
                Accessible.name: App.tr("Back to %1", frame.backWord)
                onClicked: frame.leave()

                contentItem: IconLabel {
                    spacing: back.spacing
                    display: back.display
                    icon: back.icon
                    text: back.text
                    font: back.font
                    color: frame.dense && !back.hovered && !back.visualFocus ? Look.mist : Look.frost
                }

                Binding on implicitHeight {
                    when: frame.dense
                    value: DeskLook.button
                }
            }

            Text {
                Layout.maximumWidth: frame.width / 3
                text: frame.page?.title ?? ""
                elide: Text.ElideRight
                color: Look.frost
                font.family: Look.titleFont
                font.pixelSize: frame.dense ? DeskLook.tTitle : Look.tTop
                font.weight: Font.Bold
                Accessible.role: Accessible.Heading
                Accessible.name: text
            }

            Loader {
                Layout.fillWidth: true
                Layout.fillHeight: true
                // The page's own bar takes the room that is left and gives up words in it.
                Layout.preferredWidth: 0
                Layout.minimumWidth: 0
                active: frame.own
                visible: active
                sourceComponent: frame.own ? frame.page.topBar : null
            }

            Kirigami.ActionToolBar {
                Layout.fillWidth: true
                visible: !frame.own
                alignment: Qt.AlignRight
                actions: frame.page?.actions ?? []
            }
        }

        Rectangle {
            anchors.bottom: parent.bottom
            width: parent.width
            height: Look.line
            color: Look.nightLine
        }
    }

    Item {
        id: seat

        y: frame.high
        width: frame.width
        height: frame.height - y
    }
}
