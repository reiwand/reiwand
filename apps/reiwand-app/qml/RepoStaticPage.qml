import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Kirigami.ScrollablePage {
    id: page

    readonly property alias stall: stall

    title: App.tr("Static repository")
    padding: 0
    actions: [
        Kirigami.Action {
            text: App.tr("Back to Explore")
            icon.name: "arrow-left"
            onTriggered: applicationWindow().leave()
        }
    ]

    RepoStatic {
        id: stall
    }
}
