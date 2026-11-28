import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

// Under the canvas, the effects the preview leaves out and why, as the screens of the desktop say
// them: an effect too slow for this graphics card at the size of the canvas is not drawn, and the
// canvas would show the scene without it with no word.
Rectangle {
    id: strip

    required property var page
    property var lines: App.state.previews?.[page.editor.output]?.left_out ?? []
    readonly property string text: lines.map(line => reasons.text(line)).join("\n")

    visible: lines.length > 0
    implicitHeight: row.implicitHeight + Look.e2 * 2
    color: DeskLook.ground
    border.color: Look.coral

    LeftOut {
        id: reasons

        daemon: App
    }

    RowLayout {
        id: row

        anchors.fill: parent
        anchors.margins: Look.e2
        spacing: Look.e2

        Kirigami.Icon {
            Layout.alignment: Qt.AlignTop
            implicitWidth: Look.e4
            implicitHeight: Look.e4
            source: "dialog-warning"
            color: Look.coral
        }

        QQC2.Label {
            objectName: "leftOutText"
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            textFormat: Text.PlainText
            text: strip.text
        }
    }
}
