import QtQuick
import QtQuick.Dialogs

QtObject {
    id: chooser

    enum Mode {
        Open,
        Files,
        Folder,
        Save
    }

    property int mode: Chooser.Open
    property string title
    property var nameFilters: []
    property string defaultSuffix
    property url currentFolder
    property url selectedFile
    property var selectedFiles: []
    property url selectedFolder
    property Item opener
    property var dialog: null
    property Component files: Component {
        FileDialog {
            property Component kind

            title: chooser.title
            fileMode: chooser.mode === Chooser.Files ? FileDialog.OpenFiles : chooser.mode === Chooser.Save ? FileDialog.SaveFile : FileDialog.OpenFile
            nameFilters: chooser.nameFilters
            defaultSuffix: chooser.defaultSuffix
            onAccepted: chooser.done(true)
            onRejected: chooser.done(false)
        }
    }
    property Component folders: Component {
        FolderDialog {
            property Component kind

            title: chooser.title
            onAccepted: chooser.done(true)
            onRejected: chooser.done(false)
        }
    }

    signal accepted
    signal rejected

    function open() {
        const kind = mode === Chooser.Folder ? folders : files;
        if (dialog && dialog.kind !== kind) {
            dialog.destroy();
            dialog = null;
        }
        dialog = dialog ?? kind.createObject(chooser, {
            "kind": kind
        });
        if (currentFolder.toString() !== "")
            dialog.currentFolder = currentFolder;
        opener = dialog.parentWindow?.activeFocusItem ?? null;
        dialog.open();
    }

    function done(chosen) {
        if (chosen && mode === Chooser.Folder)
            selectedFolder = dialog.selectedFolder;
        else if (chosen) {
            selectedFile = dialog.selectedFile;
            selectedFiles = dialog.selectedFiles;
        }
        opener && opener.forceActiveFocus();
        chosen ? accepted() : rejected();
    }
}
