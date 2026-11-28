import QtQuick
import io.github.reiwand.app

FocusScope {
    id: part

    required property var page
    readonly property bool several: page.outputs.length > 1
    readonly property var lock: page.lock ?? {}

    function picked(own) {
        return own && !own.desktop && own.playlist === null ? 1 : 0;
    }

    // first: what follows (the desktop, or every screen); a single screen under a choice for every
    // screen can also keep its desktop
    function words(own, first, desktop) {
        return [first].concat(desktop ? [App.tr("Like the desktop")] : [], picked(own) ? [own.name] : [], page.playlists.map(list => list.name));
    }

    function at(own, base) {
        if (!own)
            return 0;
        if (own.desktop)
            return 1;
        return picked(own) ? base : base + Math.max(0, page.playlists.findIndex(list => list.uid === own.playlist));
    }

    function choose(own, index, output, base) {
        const picked = part.picked(own);
        if (index === 0)
            return App.act("FollowDesktopOnLock", [output]);
        if (base === 2 && index === 1)
            return App.act("ShowDesktopOnLock", [output]);
        if (index >= base + picked)
            App.act("SetLockPlaylist", [page.playlists[index - base - picked].uid, output]);
    }

    implicitHeight: rows.height

    ListRows {
        id: rows

        ListRow {
            readonly property var own: part.lock.every ?? null

            objectName: "dynamic"
            text: part.several ? App.tr("Every screen") : App.tr("Lock screen")
            kind: "choice"
            options: part.words(own, App.tr("Like the desktop"), false)
            current: part.at(own, 1)
            explain: part.several ? App.tr("What every screen shows while the session is locked. To show one wallpaper there, choose “Show on every lock screen” in the menu of its cover.") : App.tr("What the screen shows while the session is locked. To show one wallpaper there, choose “Show on the lock screen” in the menu of its cover.")
            onChosen: index => part.choose(own, index, "", 1)
        }

        Repeater {
            model: part.several ? part.page.outputs : []

            ListRow {
                required property string modelData
                readonly property var own: (part.lock.outputs ?? {})[modelData] ?? null
                readonly property int base: part.lock.every ? 2 : 1

                objectName: "dynamic"
                width: parent.width
                text: App.screen(modelData)
                note: modelData
                kind: "choice"
                options: part.words(own, part.lock.every ? App.tr("Like every screen") : App.tr("Like the desktop"), base === 2)
                current: part.at(own, base)
                explain: App.tr("The lock screen of this screen only.")
                onChosen: index => part.choose(own, index, modelData, base)
            }
        }
    }
}
