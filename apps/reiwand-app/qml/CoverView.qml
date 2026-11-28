import QtQuick
import io.github.reiwand.app

Cover {
    required property var model
    required property int index
    required property var from
    readonly property string caption: String(from.role(model, from.captionRole) ?? "")
    readonly property bool wants: hovered || lit && from.activeFocus
    readonly property bool scene: from.role(model, "kind") === "scene"
    // scenes, web pages and videos play on the cover while the pointer rests on it
    readonly property bool plays: scene || ["web", "video"].includes(from.role(model, "kind"))
    readonly property bool hidden: from.role(model, "flashing") === true || from.role(model, "mature") === true
    readonly property bool broken: from.role(model, "missing") === true || String(from.role(model, "broken") ?? "") !== ""
    readonly property var layering: Layers.of(String(from.role(model, "uid") ?? ""))
    readonly property string layerNote: Layers.note(layering)

    picture: from.role(model, from.pictureRole) ?? ""
    pictures: from.role(model, from.picturesRole) ?? []
    moving: wants && from.movingOf ? from.movingOf(index) : ""
    liveUid: plays && !hidden && !broken ? String(from.role(model, "uid") ?? "") : ""
    live: !from.view.moving && from.turning !== true
    name: String(from.role(model, "provider") ?? "") !== "" ? Names.of(model) : from.role(model, from.nameRole) ?? ""
    icon: from.iconRole === "" ? "" : from.role(model, from.iconRole) ?? ""
    note: layerNote !== "" ? layerNote : from.role(model, from.noteRole) ?? ""
    noteIcon: layerNote !== "" ? Layers.icon(layering) : from.role(model, from.noteIconRole) ?? ""
    noteTone: layerNote !== "" ? Layers.tone(layering) : from.role(model, from.noteToneRole) ?? ""
    progress: Layers.bar(layering)
    inUse: from.role(model, from.inUseRole) === true
    usedOn: String(from.role(model, from.usedOnRole) ?? "")
    listed: from.listedOf ? from.listedOf(String(from.role(model, "uid") ?? "")) : ""
    off: from.role(model, from.offRole) === true
    lit: index >= 0 && index === from.view.currentIndex
    choosing: from.choosing
    choosable: from.choosable
    chosen: from.chosen.indexOf(index) >= 0
    onTapped: modifiers => from.tap(index, modifiers)
    onDoubleTapped: from.activated(index)
    onAskedMenu: {
        from.tap(index, 0);
        from.asked(index, this);
    }
    onWantsChanged: wants && from.wanted(index)
}
