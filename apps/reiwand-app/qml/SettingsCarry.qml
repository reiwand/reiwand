import QtQuick
import io.github.reiwand.app

FocusScope {
    id: part

    required property var page
    property var exporter: null
    property var importer: null

    function exporting() {
        exporter = exporter ?? Qt.createComponent("CarryExport.qml").createObject(applicationWindow());
        exporter.opener = send;
        exporter.offer();
    }

    function importing() {
        importer = importer ?? Qt.createComponent("CarryImport.qml").createObject(applicationWindow());
        importer.opener = take;
        importer.begin();
    }

    implicitHeight: filas.height

    Filas {
        id: filas

        Fila {
            id: send

            text: App.tr("Export my configuration…")
            note: App.tr("One file to take your repositories, lists and favorites to another computer")
            kind: "pliega"
            explain: App.tr("You choose whether it carries your own wallpapers and your passwords and keys. A file with passwords is locked with a password you choose.")
            onOpened: part.exporting()
        }

        Fila {
            id: take

            text: App.tr("Import a configuration…")
            note: App.tr("Adds what the file brings; nothing you have is replaced")
            kind: "pliega"
            explain: App.tr("You see what the file holds and tick what to bring. Lists with the same name can be joined or kept both.")
            onOpened: part.importing()
        }
    }
}
