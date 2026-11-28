import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

CreateCard {
    id: cardItem

    required property var page
    readonly property var entry: page.current
    readonly property bool starting: entry !== null && !page.onRecent && !page.atExamples
    readonly property alias primary: primary
    readonly property alias titleField: titleField
    readonly property string ask: page.onAll ? App.tr("See all") : starting && entry.id === "image" ? App.tr("Choose the picture…") : starting && entry.id === "video" ? App.tr("Choose the video…") : starting ? App.tr("Create") : page.missing ? App.tr("Find it…") : page.atExamples ? App.tr("Edit a copy…") : App.tr("Keep editing")

    title: entry ? entry.name : ""
    fields: starting && page.changing
    tone: page.missing ? "coral" : ""
    icon: page.missing ? "warning-circle" : ""
    caption: !entry ? "" : starting ? App.tr("It will be created as “%1” in %2", page.named, page.where) : page.missing ? App.tr("Its folder “%1” is gone. Did you move it or rename it?", entry.folder.split("/").slice(-1).join("")) : page.facts.footText

    function rename(on) {
        page.changing = on;
        if (on)
            titleField.forceActiveFocus(Qt.TabFocusReason);
        else if (page.focused)
            page.focused.forceActiveFocus(Qt.TabFocusReason);
    }

    function land() {
        const menu = page.Window.window?.shell?.menu ?? null;
        const now = page.Window.activeFocusItem;
        let at = now;
        while (at && at !== page && at !== menu)
            at = at.parent;
        if (page.visible && page.focused && at !== menu && !(at === page && now.activeFocusOnTab))
            page.focused.forceActiveFocus(Qt.OtherFocusReason);
    }

    middle: [
        Row {
            spacing: Look.e2

            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: App.tr("Title")
                font.family: Look.textFont
                font.pixelSize: Look.tFoot
                color: Look.mist
            }

            QQC2.TextField {
                id: titleField

                width: Look.toastWidth * 0.45
                text: cardItem.page.named
                Accessible.name: App.tr("Title")
                onTextEdited: cardItem.page.named = text
                onAccepted: cardItem.page.act(cardItem.page.focused)
                Keys.onEscapePressed: cardItem.rename(false)
            }
        },
        Row {
            spacing: Look.e2

            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: App.tr("Folder")
                font.family: Look.textFont
                font.pixelSize: Look.tFoot
                color: Look.mist
            }

            QQC2.TextField {
                width: Look.toastWidth * 0.7
                text: paths.homed(cardItem.page.placed !== "" ? cardItem.page.placed : helper.folderFor(cardItem.page.named))
                Accessible.name: App.tr("Folder")
                onTextEdited: cardItem.page.placed = paths.expanded(text)
                onAccepted: cardItem.page.act(cardItem.page.focused)
                Keys.onEscapePressed: cardItem.rename(false)
            }
        }
    ]

    QQC2.Button {
        visible: cardItem.starting && !cardItem.page.changing
        flat: true
        text: App.tr("Change…")
        onClicked: cardItem.rename(true)
    }

    QQC2.Button {
        id: primary

        visible: cardItem.entry !== null
        highlighted: true
        text: cardItem.ask
        icon.name: cardItem.page.onAll ? "squares-four" : cardItem.starting ? (cardItem.entry.id === "image" || cardItem.entry.id === "video" ? "folder-open" : "plus") : cardItem.page.missing ? "magnifying-glass" : cardItem.page.atExamples ? "copy" : "pencil-simple"
        onClicked: cardItem.page.act(cardItem.page.focused)
    }

    PiecesKey {
        text: App.trc("key", "Enter")
    }

    QQC2.Button {
        visible: cardItem.page.missing
        flat: true
        text: App.tr("Remove from the list")
        onClicked: cardItem.page.facts.forget()
    }

    SceneEditor {
        id: helper
    }

    Recents {
        id: paths
    }
}
