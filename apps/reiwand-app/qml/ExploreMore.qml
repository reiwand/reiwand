import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

QQC2.Button {
    id: more

    readonly property alias menu: menu
    property var publisher: null

    function pick(index) {
        if (index === 0)
            packages.open();
        else if (index === 1)
            applicationWindow().work("RepoStaticPage.qml", {});
    }

    text: App.tr("More")
    icon.name: "dots-three"
    Accessible.name: App.tr("More: tools to publish wallpapers")
    onClicked: menu.open()

    OptionMenu {
        id: menu

        opener: more
        options: [{
                "text": App.tr("Upload a package…"),
                "icon": "upload-simple"
            }, {
                "text": App.tr("Static repository…"),
                "icon": "folder-simple"
            }]
        onChosen: index => more.pick(index)
    }

    Chooser {
        id: packages

        nameFilters: [App.tr("Packages (*.wpak)")]
        onAccepted: more.publisher && more.publisher.openWith(App.path(selectedFile))
    }
}
