import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// Another cover at rest, beside the package's own in How it will look, to see it as it will sit in
// a list: the thumbnail of one of the scenes that come with the app, dimmed as covers not chosen
// are.
ColumnLayout {
    id: beside

    required property var page
    readonly property var other: (JSON.parse(page.editor.examples() || "[]") ?? []).find(item => item.picture && item.title !== (page.view.title ?? "")) ?? null
    readonly property string picture: other?.picture ?? ""

    spacing: Look.e2

    Rectangle {
        Layout.fillWidth: true
        Layout.preferredHeight: width * 9 / 16
        radius: Look.radius
        color: Look.nightHigh
        opacity: 0.6
        clip: true

        Image {
            anchors.fill: parent
            source: beside.picture
            fillMode: Image.PreserveAspectCrop
            asynchronous: true
            sourceSize.width: width
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        text: App.tr("next to another cover, at rest")
        color: Look.mist
        font.pixelSize: Look.tFoot
        wrapMode: Text.Wrap
    }
}
