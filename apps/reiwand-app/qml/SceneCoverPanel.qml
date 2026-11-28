import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// How it will look: the cover of the package as a library shows it (its thumbnail, title and
// kind), next to another cover at rest when there is room, "Update the thumbnail" from the canvas,
// "Record 6 s" for the animated preview, and the package's thumbnail, animated preview and accent
// fields.
Rectangle {
    id: cover

    required property var page
    property int shots: 0
    readonly property var manifest: {
        cover.page.view;
        return JSON.parse(page.editor.value("/manifest") || "{}") ?? {};
    }
    readonly property string thumbnail: Publishing.thumbnail(manifest)
    readonly property var rows: {
        cover.page.view;
        cover.page.fields;
        return JSON.parse(page.editor.inspect("/manifest") || "[]").filter(row => ["previews", "preview", "accent"].includes(String(row.pointer).split("/")[2]));
    }

    color: Look.night
    clip: true

    Connections {
        target: cover.page.editor

        function onBusyChanged() {
            if (!cover.page.editor.busy)
                cover.shots++;
        }
    }

    QQC2.ScrollView {
        id: scroll

        anchors.fill: parent
        anchors.margins: Look.e3
        contentWidth: availableWidth

        ScrollColumn {
            view: scroll

            RowLayout {
                Layout.fillWidth: true
                spacing: Look.e3

                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredWidth: 1
                    Layout.alignment: Qt.AlignTop
                    // A short group keeps the whole picture, and its words, in sight.
                    Layout.preferredHeight: Math.max(Math.min(width * 9 / 16, scroll.availableHeight), none.implicitHeight + Look.e4)
                    radius: Look.radius
                    color: Look.nightHigh
                    clip: true

                    Image {
                        anchors.fill: parent
                        visible: cover.thumbnail !== ""
                        source: cover.thumbnail !== "" ? "file://" + cover.page.editor.folder + "/" + cover.thumbnail + "?" + cover.shots : ""
                        fillMode: Image.PreserveAspectCrop
                        asynchronous: true
                        cache: false
                        sourceSize.width: width
                    }

                    QQC2.Label {
                        id: none

                        anchors.centerIn: parent
                        width: parent.width - Look.e4 * 2
                        visible: cover.thumbnail === ""
                        horizontalAlignment: Text.AlignHCenter
                        wrapMode: Text.Wrap
                        color: Look.mist
                        text: App.tr("It has no thumbnail: the library draws one by itself, but a server does not take it without one.")
                    }
                }

                SceneCoverBeside {
                    Layout.fillWidth: true
                    Layout.preferredWidth: 1
                    Layout.alignment: Qt.AlignTop
                    visible: scroll.availableWidth >= Look.sideLeft * 2 && picture !== ""
                    page: cover.page
                }
            }

            QQC2.Label {
                Layout.fillWidth: true
                text: cover.page.view.title ?? ""
                font.pixelSize: Look.tSection
                font.weight: Font.Bold
                elide: Text.ElideRight
            }

            QQC2.Label {
                Layout.fillWidth: true
                text: SceneWords.kind(cover.page.view.kind ?? "scene")
                color: Look.mist
                font.pixelSize: Look.tFoot
            }

            PackageCapture {
                host: ({
                        "field": {
                            "widget": ""
                        },
                        "page": cover.page
                    })
                display: QQC2.AbstractButton.TextBesideIcon
                text: App.tr("Update the thumbnail")
            }

            // Six seconds from the second the preview is on, as a WebP that keeps every color.
            QQC2.ToolButton {
                readonly property string kind: cover.page.view.kind ?? "scene"

                visible: kind === "scene" || kind === "video"
                enabled: !cover.page.editor.busy
                icon.name: "media-record"
                text: App.tr("Record 6 s")
                display: QQC2.AbstractButton.TextBesideIcon
                QQC2.ToolTip.text: App.tr("Record 6 seconds from the second the preview is on and keep them as the animated preview")
                QQC2.ToolTip.visible: hovered
                onClicked: cover.page.run("shot", {
                    "recording": {
                        "from": Math.max(0, Number(cover.page.editor.time ?? 0)),
                        "seconds": 6,
                        "fps": 10,
                        "format": "webp"
                    }
                })
            }

            SceneFieldList {
                Layout.fillWidth: true
                Layout.topMargin: Look.e2
                page: cover.page
                rows: cover.rows
            }
        }
    }
}
