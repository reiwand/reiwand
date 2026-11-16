import QtQuick
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

ColumnLayout {
    id: head

    required property var page

    Opcion {
        Layout.fillWidth: true
        lit: head.page.editor.selected === "/manifest"
        icon: "info"
        text: App.tr("Wallpaper details…")
        onClicked: head.page.editor.select("/manifest")
    }

    Text {
        Layout.fillWidth: true
        Layout.leftMargin: Look.e3
        wrapMode: Text.WordWrap
        text: App.tr("Title, author, license and tags")
        color: Look.brumaBaja
        font.pixelSize: Look.tPie
    }

    Kirigami.Heading {
        Layout.fillWidth: true
        level: 4
        text: App.tr("Options you offer to whoever uses it")
    }
}
