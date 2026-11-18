import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

ColumnLayout {
    id: exporting

    required property var maps
    required property var depth
    required property bool colour

    QQC2.Button {
        text: App.tr("Export the adjusted map…")
        icon.name: "document-export"
        enabled: exporting.maps.map !== "" && !exporting.maps.busy
        onClicked: sending.open()
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: exporting.maps.note.startsWith("/")
        text: App.tr("Saved as %1", exporting.maps.note)
        wrapMode: Text.Wrap
    }

    Chooser {
        id: sending

        mode: Chooser.Save
        defaultSuffix: "png"
        nameFilters: [App.tr("PNG pictures") + " (*.png)"]
        onAccepted: exporting.maps.exportMap(selectedFile.toString(), JSON.stringify(exporting.depth), true, exporting.colour)
    }
}
