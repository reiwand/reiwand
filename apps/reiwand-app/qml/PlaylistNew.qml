import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

SmallPanel {
    id: creator

    required property var page

    title: App.tr("New playlist")
    text: App.tr("Leave the search empty to choose its wallpapers by hand.")
    primary: App.tr("Create")
    primaryIcon: "plus"
    first: name
    onAboutToShow: {
        name.text = "";
        search.text = creator.page.query;
    }
    onAccepted: name.text.trim() !== "" && App.act("Create", [name.text.trim(), search.text.trim()], "Playlists1", made => creator && (creator.page.opened = made[0]))
    foot: [
        Column {
            spacing: Look.e2

            QQC2.TextField {
                id: name

                width: creator.footWidth
                placeholderText: App.tr("Name")
            }

            QQC2.TextField {
                id: search

                width: creator.footWidth
                placeholderText: App.tr("Search, for example fav:yes")
            }
        }
    ]
}
