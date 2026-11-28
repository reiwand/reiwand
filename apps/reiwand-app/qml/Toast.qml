import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

FocusScope {
    id: toast

    property string text
    property string tone
    property string icon: Tone.icon(tone)
    property string action
    readonly property bool held: hover.hovered || activeFocus
    readonly property real room: Look.toastWidth - Look.e4 - 10 - Look.toastIcon - Look.closeHeight - 3 * Look.e3 - Look.e2
    readonly property bool stacked: action !== "" && metrics.advanceWidth > room - inline.implicitWidth - Look.e3

    signal acted
    signal closed

    implicitWidth: Math.min(Look.toastWidth, line.implicitWidth + Look.e4 + 10)
    implicitHeight: line.implicitHeight + 2 * Look.e3
    Accessible.role: Accessible.AlertMessage
    Accessible.name: text

    HoverHandler {
        id: hover
    }

    Rectangle {
        anchors.fill: parent
        radius: Look.radius
        color: Look.nightHigh
        border.width: Look.line
        border.color: Look.nightLine

        Shadow {
            kind: "panel"
        }
    }

    TextMetrics {
        id: metrics

        font: words.font
        text: toast.text
    }

    ColumnLayout {
        id: line

        anchors.fill: parent
        anchors.leftMargin: Look.e4
        anchors.rightMargin: 10
        spacing: Look.e2

        RowLayout {
            Layout.fillWidth: true
            spacing: Look.e3

            IconImage {
                name: toast.icon
                sourceSize: Qt.size(Look.toastIcon, Look.toastIcon)
                color: Tone.of(toast.tone, false, Look.mist)
            }

            Text {
                id: words

                Layout.fillWidth: true
                Layout.rightMargin: Look.e2
                text: toast.text
                wrapMode: Text.WordWrap
                lineHeight: 1.35
                font.family: Look.textFont
                font.pixelSize: Look.tBody
                color: Look.frost
            }

            QQC2.Button {
                id: inline

                visible: toast.action !== "" && !toast.stacked
                flat: true
                text: toast.action
                onClicked: toast.acted()
            }

            QQC2.AbstractButton {
                id: close

                objectName: "notice-close"
                implicitWidth: Look.closeHeight
                implicitHeight: Look.closeHeight
                Accessible.role: Accessible.Button
                Accessible.name: App.tr("Close")
                onClicked: toast.closed()

                Rectangle {
                    anchors.fill: parent
                    radius: Look.radiusSmall
                    color: close.hovered ? Look.nightLine : "transparent"

                    FocusHalo {
                        corner: Look.radiusSmall
                        visible: close.visualFocus
                    }
                }

                IconImage {
                    anchors.centerIn: parent
                    name: "x"
                    sourceSize: Qt.size(Look.icon, Look.icon)
                    color: close.hovered ? Look.frost : Look.mist
                }
            }
        }

        QQC2.Button {
            Layout.alignment: Qt.AlignRight
            visible: toast.action !== "" && toast.stacked
            flat: true
            text: toast.action
            onClicked: toast.acted()
        }
    }
}
