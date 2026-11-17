import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

QQC2.Button {
    required property var tools
    required property bool tight

    flat: true
    icon.name: "download-simple"
    text: App.tr("Import wallpaper…")
    display: tight ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
    onClicked: tools.dialogs.importer.pick()
    QQC2.ToolTip.text: text
    QQC2.ToolTip.visible: tight && hovered
}
