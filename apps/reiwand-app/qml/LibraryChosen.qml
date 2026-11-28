import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Selection {
    id: line

    required property var page
    readonly property var tools: page.tools
    readonly property var items: page.library
    readonly property var into: page.into
    readonly property bool acts: count > 0 && !into

    function add() {
        const uid = into.uid;
        App.act("AddItems", [uid, JSON.parse(items.selection())], "Playlists1", () => line && applicationWindow().showPage(1, {
                "opened": uid
            }));
    }

    count: page.chosen.length

    Text {
        visible: line.count === 0
        text: App.tr("Click the covers to choose them")
        color: Look.mist
        font.family: Look.textFont
        font.pixelSize: Look.tName
    }

    QQC2.Button {
        visible: line.count > 0 && !!line.into
        highlighted: true
        icon.name: "plus"
        text: App.tr("Add to “%1”", line.into?.name ?? "")
        onClicked: line.add()
    }

    QQC2.Button {
        id: rotate

        visible: line.acts
        highlighted: true
        icon.name: "clock-clockwise"
        text: App.tr("Rotate these every…")
        onClicked: line.tools.chosen("rotator", rotate)
    }

    QQC2.Button {
        id: list

        visible: line.acts
        flat: true
        text: App.tr("Playlist")
        onClicked: line.tools.chosen("playlist", list)
    }

    QQC2.Button {
        id: tags

        visible: line.acts
        flat: true
        text: App.tr("Tags")
        onClicked: line.tools.chosen("tagger", tags)
    }

    QQC2.Button {
        id: more

        visible: line.acts
        flat: true
        text: App.tr("More")
        icon.name: "caret-down"
        LayoutMirroring.enabled: true
        onClicked: menu.open()
    }

    QQC2.Button {
        flat: true
        text: App.tr("Stop choosing")
        onClicked: line.page.stop()
    }

    PiecesKey {
        text: App.trc("key", "Esc")
    }

    OptionMenu {
        id: menu

        opener: more
        title: App.tr("More")
        options: [{
                "text": line.items.favorite ? App.tr("Unfavorite") : App.tr("Favorite"),
                "icon": line.items.favorite ? "heart" : "heart-fill"
            }, {
                "text": App.tr("Rate…"),
                "icon": "star"
            }, {
                "text": App.tr("Allow these wallpapers…"),
                "icon": "warning",
                "off": !line.items.flashing
            }, {
                "separator": true
            }, {
                "text": App.tr("Remove…"),
                "icon": "trash",
                "tone": "coral",
                "off": !line.items.removable
            }]
        onChosen: index => {
            if (index === 0)
                line.tools.favorite(!line.items.favorite);
            else if (index === 1)
                line.tools.chosen("rater", more);
            else if (index === 2)
                line.tools.chosen("allower", more);
            else if (index === 4)
                line.tools.chosen("remover", more);
        }
    }
}
