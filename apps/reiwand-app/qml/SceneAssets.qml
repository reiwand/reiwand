import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

ColumnLayout {
    id: panel

    required property var page

    readonly property var frames: JSON.parse(strips.frames || "{}")
    property alias strips: strips
    property alias cutter: cutter
    property alias packing: packing
    property alias menu: menu

    function reach(at) {
        const all = list.model;
        const index = Math.max(0, Math.min(all.length - 1, at));
        if (!all[index])
            return;
        page.editor.select(all[index].pointer);
        list.positionViewAtIndex(index, ListView.Contain);
    }

    Strips {
        id: strips
    }

    SceneMenu {
        id: menu
    }

    Item {
        Layout.fillWidth: true
        Layout.fillHeight: true

        QQC2.ScrollView {
            anchors.fill: parent

            ListView {
                id: list

                readonly property bool focusList: true

                model: panel.page.view.assets ?? []
                clip: true
                activeFocusOnTab: true
                currentIndex: model.findIndex(asset => asset.pointer === panel.page.editor.selected)
                Accessible.role: Accessible.List
                Keys.onShortcutOverride: event => event.accepted = [Qt.Key_Home, Qt.Key_End].includes(event.key)
                Keys.onUpPressed: panel.reach(currentIndex - 1)
                Keys.onDownPressed: panel.reach(currentIndex + 1)
                Keys.onPressed: event => {
                    event.accepted = [Qt.Key_Home, Qt.Key_End].includes(event.key);
                    if (event.accepted)
                        panel.reach(event.key === Qt.Key_Home ? 0 : count - 1);
                }

                delegate: SceneAssetRow {
                    owner: panel
                }
            }
        }

        SceneDropFiles {
            anchors.fill: parent
            page: panel.page
        }
    }

    SceneAssetParts {
        Layout.fillWidth: true
        page: panel.page
    }

    Flow {
        Layout.fillWidth: true

        QQC2.ToolButton {
            width: Math.min(implicitWidth, parent.width)
            text: App.tr("Import…")
            icon.name: "document-import"
            onClicked: files.open()
        }

        QQC2.ToolButton {
            width: Math.min(implicitWidth, parent.width)
            text: App.tr("Import a sprite sheet or a data map…")
            icon.name: "view-grid"
            onClicked: sheet.open()
        }

        QQC2.ToolButton {
            width: Math.min(implicitWidth, parent.width)
            text: App.tr("Import from an assets package…")
            icon.name: "package-x-generic"
            onClicked: shared.open()
        }

        QQC2.ToolButton {
            width: Math.min(implicitWidth, parent.width)
            text: App.tr("Make an assets package…")
            icon.name: "document-export"
            onClicked: offer.open()
        }

        QQC2.ToolButton {
            width: Math.min(implicitWidth, parent.width)
            text: App.tr("Remove unused files")
            icon.name: "edit-clear-all"
            onClicked: panel.page.editor.prune()
        }
    }

    SceneShared {
        id: shared

        page: panel.page
    }

    SceneOffer {
        id: offer

        page: panel.page
    }

    SceneKtx2 {
        id: packing

        page: panel.page
    }

    SceneSpriteCutter {
        id: cutter

        page: panel.page
    }

    SceneImportSheet {
        id: sheet

        page: panel.page
    }

    Chooser {
        id: files

        mode: Chooser.Files
        nameFilters: [App.tr("Pictures, videos, sounds, fonts, color tables and models") + " (*.png *.jpg *.jpeg *.webp *.gif *.bmp *.ktx2 *.hdr *.exr *.webm *.mp4 *.mkv *.ogg *.opus *.flac *.wav *.mp3 *.ttf *.otf *.ttc *.otc *.woff2 *.cube *.glb)"]
        onAccepted: selectedFiles.forEach(file => panel.page.run("import", {
                        "file": App.path(file)
                    }))
    }
}
