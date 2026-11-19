import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

PanelChico {
    id: dialog

    required property var editor
    readonly property int quality: Math.round(level.value)
    readonly property int sideAcross: resize.checked ? across.value : 0
    readonly property int sideDown: resize.checked ? down.value : 0
    readonly property bool original: plain.checked

    function exercise() {
        if (quality !== 90 || sideAcross !== 0 || sideDown !== 0 || original)
            throw new Error("export: the dialog did not start with every value filled in");
        resize.checked = true;
        if (sideAcross !== 1920 || sideDown !== 1080)
            throw new Error("export: the size did not follow the resize box");
        resize.checked = false;
    }

    title: App.tr("Export the picture")
    text: App.tr("The file type chosen next sets the format. The quality is used by JPEG and AVIF; PNG and lossless WebP ignore it.")
    quiet: App.tr("Cancel")
    principal: App.tr("Choose the file…")
    principalIcon: "document-export"
    onAccepted: files.open()

    ColumnLayout {
        width: dialog.width - 2 * Look.e4

        EditSlider {
            id: level

            text: App.tr("Quality")
            from: 1
            to: 100
            stepSize: 1
            value: 90
            onMoved: value => level.value = value
        }

        QQC2.CheckBox {
            id: resize

            text: App.tr("Resize to a size")
        }

        RowLayout {
            enabled: resize.checked

            QQC2.SpinBox {
                id: across

                from: 16
                to: 16384
                value: 1920
                editable: true
                Accessible.name: App.tr("Width")
            }

            QQC2.Label {
                text: "×"
            }

            QQC2.SpinBox {
                id: down

                from: 16
                to: 16384
                value: 1080
                editable: true
                Accessible.name: App.tr("Height")
            }

            QQC2.Label {
                text: App.tr("px; the picture is cropped to this shape")
                wrapMode: Text.Wrap
                Layout.fillWidth: true
            }
        }

        QQC2.CheckBox {
            id: plain

            text: App.tr("Without my edits, the original with its geometry")
        }
    }

    Chooser {
        id: files

        mode: Chooser.Save
        defaultSuffix: "png"
        nameFilters: [App.tr("PNG pictures") + " (*.png)", App.tr("JPEG pictures") + " (*.jpg *.jpeg)", App.tr("Lossless WebP pictures") + " (*.webp)", App.tr("AVIF pictures") + " (*.avif)"]
        onAccepted: dialog.editor.saveAs(selectedFile.toString(), dialog.quality, dialog.sideAcross, dialog.sideDown, dialog.original)
    }
}
