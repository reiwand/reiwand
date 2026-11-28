import QtQuick
import io.github.reiwand.app

Item {
    id: menu

    required property var owner
    required property var tools
    readonly property var page: owner.page
    readonly property alias opened: popup.opened
    readonly property alias popup: popup

    function chosen() {
        const current = owner.current;
        if (!current)
            return [];
        const ids = [current.id].concat(page.picked.filter(id => id !== current.id));
        return ids.map(id => owner.layers.find(layer => layer.id === id)).filter(layer => layer);
    }

    function each(name, layers, args) {
        page.run("steps", layers.map(layer => ({
                        [name]: Object.assign({
                            "id": layer.id
                        }, args)
                    })));
    }

    function tops(layers) {
        return layers.filter(layer => !layers.some(other => other !== layer && layer.pointer.startsWith(other.pointer + "/")));
    }

    function entry(text, icon, act, tone, part) {
        return {
            "text": text,
            "icon": icon,
            "tone": tone ?? "",
            "act": act,
            "part": part ?? "layer"
        };
    }

    function entries() {
        const set = chosen(), one = set.length === 1, line = {
            "separator": true
        };
        if (set.length === 0)
            return empty();
        const hidden = set.every(layer => layer.hidden), locked = set.every(layer => layer.locked);
        const groups = set.filter(layer => layer.type === "group");
        const out = [];
        if (one)
            out.push(entry(App.tr("Rename the layer"), "edit-rename", () => clip.rename(), "", "edit"));
        out.push(entry(App.tr("Duplicate"), "copy", () => each("duplicate_layer", set, {}), "", "edit"));
        out.push(entry(App.tr("Group the layers"), "stack", () => {
            page.run("group", {
                    "ids": set.map(layer => layer.id)
                });
            page.picked = [];
        }));
        if (groups.length > 0)
            out.push(entry(App.tr("Ungroup"), "", () => each("ungroup", groups, {})));
        out.push(entry(hidden ? App.tr("Show") : App.tr("Hide"), hidden ? "view-visible" : "view-hidden", () => each("hide", set, {
                    "hidden": !hidden
                })));
        out.push(entry(locked ? App.tr("Unlock") : App.tr("Lock"), locked ? "object-locked" : "object-locked", () => each("lock", set, {
                    "locked": !locked
                })));
        if (one && owner.index < owner.siblings.length - 1)
            out.push(entry(App.tr("Raise"), "arrow-up", () => owner.move(owner.index + 1, owner.group)));
        if (one && owner.index > 0)
            out.push(entry(App.tr("Lower"), "arrow-down", () => owner.move(owner.index - 1, owner.group)));
        out.push(line, entry(App.tr("Copy the layers"), "edit-copy", () => clip.copy(), "", "edit"));
        if (SceneClip.pasteable)
            out.push(entry(App.tr("Paste the layers"), "edit-paste", () => clip.paste(), "", "edit"));
        out.push(line, entry(App.tr("Remove"), "edit-delete", () => remove(), "coral", "edit"));
        return out;
    }

    function remove() {
        const set = chosen();
        if (set.length === 1)
            tools.remove();
        else if (set.length > 1)
            each("remove_layer", tops(set), {});
        page.picked = [];
    }

    function empty() {
        const out = [entry(App.tr("Add a layer"), "list-add", () => tools.add())];
        if (SceneClip.pasteable)
            out.unshift(entry(App.tr("Paste the layers"), "edit-paste", () => clip.paste(), "", "edit"));
        return out;
    }

    function onLayer(layer, item, x, y) {
        if (layer.pointer !== page.editor.selected && !page.picked.includes(layer.id))
            owner.pick(layer, false);
        popup.show(item, x, y, entries());
    }

    function onEmpty(item, x, y) {
        page.picked = [];
        page.editor.select("/scene/canvas");
        popup.show(item, x, y, empty());
    }

    SceneMenu {
        id: popup
    }

    SceneLayerClip {
        id: clip

        owner: menu.owner
    }
}
