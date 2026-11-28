import QtQuick
import io.github.reiwand.app

Sublist {
    required property var editor

    movable: editor.manual
    removeText: editor.manual ? App.tr("Remove") : ""
    addText: editor.manual ? App.tr("Add wallpapers…") : ""
    otherText: editor.manual ? App.tr("Add a folder…") : ""
    otherIcon: "folder-add"
    model: editor.items.map(item => ({
                "name": item.title ?? item.uid,
                "picture": item.thumb ? "file://" + item.thumb : "",
                "where": editor.shownNow.includes(item.uid) ? App.tr("on screen now") : ""
            }))
    onRemoved: index => editor.edit("RemoveEntries", [[index], [editor.items[index].uid]])
    onReordered: (from, to) => editor.edit("MoveItem", [from, to, editor.items[from].uid])
    onAdded: applicationWindow().showPage(0, {
        "all": true,
        "choosing": true,
        "into": {
            "uid": editor.uid,
            "name": editor.playlist.name ?? ""
        }
    })
}
