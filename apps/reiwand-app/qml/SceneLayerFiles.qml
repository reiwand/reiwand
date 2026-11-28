import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

QQC2.Menu {
    id: files

    required property var owner
    readonly property var filters: ({
            "image": App.tr("Pictures") + " (*.png *.jpg *.jpeg *.webp *.gif *.bmp *.ktx2 *.hdr *.exr)",
            "video": App.tr("Videos") + " (*.webm *.mp4 *.m4v *.mkv *.mov *.avi *.ogv *.mpg *.mpeg)",
            "sound": App.tr("Sounds") + " (*.ogg *.opus *.flac *.wav *.mp3)",
            "mesh": App.tr("Binary glTF meshes") + " (*.glb)"
        })
    property string kind: "image"
    // [kind, words, icon] of each layer made from a file, also offered in the top bar's Layer menu.
    readonly property var kinds: [["image", App.tr("Picture…"), "image-x-generic"], ["video", App.tr("Video…"), "video-x-generic"], ["sound", App.tr("Sound…"), "audio-x-generic"], ["mesh", App.tr("Mesh from a .glb file…"), "draw-polyline"], ["model", App.tr("3D model from a .glb file…"), "draw-cuboid"]]
    property Chooser dialog: Chooser {
        nameFilters: [files.filters[files.kind]]
        onAccepted: files.add(App.path(selectedFile))
    }

    function choose(chosen) {
        if (chosen === "model") {
            importer.open();
            return;
        }
        kind = chosen;
        dialog.open();
    }

    function add(file) {
        owner.page.run("add_layer", {
            "kind": kind,
            "file": file,
            "into": owner.current?.type === "group" ? owner.current.id : undefined
        });
    }

    title: App.tr("Layer from a file")

    Repeater {
        model: files.kinds

        delegate: QQC2.MenuItem {
            required property var modelData

            text: modelData[1]
            icon.name: modelData[2]
            onTriggered: files.choose(modelData[0])
        }
    }

    SceneModelImport {
        id: importer

        page: files.owner.page
        into: files.owner.current?.type === "group" ? files.owner.current.id : ""
    }
}
