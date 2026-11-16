import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

RowLayout {
    id: row

    property string text

    signal removed

    QQC2.Label {
        Layout.fillWidth: true
        text: row.text
        elide: Text.ElideRight
    }

    QQC2.ToolButton {
        icon.name: "edit-delete"
        text: App.tr("Remove")
        display: QQC2.AbstractButton.IconOnly
        onClicked: row.removed()
    }
}
