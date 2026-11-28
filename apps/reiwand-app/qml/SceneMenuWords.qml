import QtQuick
import io.github.reiwand.app

// What each menu of the editor's top bar holds. Every entry does what a panel, a bar or a key of
// the editor already does; the menus only gather them.
QtObject {
    id: words

    readonly property var names: ["file", "edit", "layer", "view", "window"]

    // A package that is not a scene has no layers and no canvas of its own to zoom.
    function namesOf(page) {
        return page?.layers ? names : ["file", "edit", "view", "window"];
    }

    function entry(text, icon, act, note, off) {
        return {
            "text": text,
            "icon": icon,
            "act": act,
            "note": note ?? "",
            "off": off ?? false
        };
    }

    function heading(text) {
        return {
            "heading": text
        };
    }

    function title(name) {
        return {
            "file": App.tr("&File"),
            "edit": App.tr("&Edit"),
            "layer": App.tr("&Layer"),
            "view": App.tr("&View"),
            "window": App.tr("&Window")
        }[name];
    }

    function layered(page, part) {
        if (!page.layers)
            return [];
        const menu = page.layers.menu;
        return menu.entries().filter(item => !item.separator && item.part === part);
    }

    // Back to the page the editor was opened from, as the top bar's first button.
    function back(page) {
        const host = page.Window.window;
        if (!host?.backWord)
            return [];
        return [
            {
                "separator": true
            },
            entry(App.tr("Back to %1", host.backWord), "caret-left", () => host.leave())];
    }

    function file(page) {
        const assets = page.docks.assets;
        if (!assets)
            return [entry(App.tr("Save now"), "document-save", () => page.editor.save(), "Ctrl+S", !page.editor.dirty || page.editor.busy)].concat(back(page));
        return [entry(App.tr("Save now"), "document-save", () => page.editor.save(), "Ctrl+S", !page.editor.dirty || page.editor.busy),
            {
                "separator": true
            },
            entry(App.tr("Import…"), "document-import", () => assets.bring("files")), entry(App.tr("Import a sprite sheet or a data map…"), "view-grid", () => assets.bring("sheet")), entry(App.tr("Import from an assets package…"), "package-x-generic", () => assets.bring("shared")), entry(App.tr("Make an assets package…"), "document-export", () => assets.bring("offer")),
            {
                "separator": true
            },
            entry(App.tr("Capture…"), "camera", () => page.docks.stage.canvas.bar.capture.clicked()),
            {
                "separator": true
            },
            entry(App.tr("Open the scene's folder"), "document-open-folder", () => Qt.openUrlExternally("file://" + page.editor.folder))].concat(back(page));
    }

    function edit(page) {
        const history = page.view.history ?? ({});
        const done = (history.done ?? []).length > 0, undone = (history.undone ?? []).length > 0;
        const out = [entry(history.undo ? App.tr("Undo: %1", history.undo) : App.tr("Undo"), "arrow-counter-clockwise", () => page.run("undo"), "Ctrl+Z", !done), entry(history.redo ? App.tr("Redo: %1", history.redo) : App.tr("Redo"), "arrow-clockwise", () => page.run("redo"), "Ctrl+Shift+Z", !undone)];
        const own = layered(page, "edit");
        return own.length > 0 ? out.concat([
            {
                "separator": true
            }
        ], own) : out;
    }

    function layer(page) {
        const held = (page.view.layers ?? []).find(item => item.pointer === page.editor.selected) ?? null;
        const own = layered(page, "layer").filter(item => item.text !== App.tr("Add a layer"));
        const files = page.layers.tools.files;
        const templated = held && ["group", "image", "text", "shape", "solid", "spectrum", "mesh"].includes(held.type);
        const made = [heading(App.tr("Layer from a file"))].concat(files.kinds.map(kind => entry(kind[1], kind[2], () => files.choose(kind[0]))), [
            {
                "separator": true
            },
            entry(App.tr("Make a template"), "copy", () => {
                page.run("template", {
                    "make": {
                        "layers": [held.id].concat(page.picked.filter(id => id !== held.id))
                    }
                });
                page.picked = [];
            }, "", !templated)]);
        return [entry(App.tr("Add a layer"), "list-add", () => page.layers.tools.add()), entry(App.tr("Add an effect"), "sparkle", () => page.layout.reveal("effects") || page.layout.reopen("effects"), "", !held), entry(held?.solo ? App.tr("Show every layer") : App.tr("Show alone"), "favorite", () => page.run("solo", held.solo ? {} : {
                    "id": held.id
                }), "", !held),
            {
                "separator": true
            }
        ].concat(made, own.length > 0 ? [
            {
                "separator": true
            }
        ] : [], own);
    }

    function halved(page) {
        return (JSON.parse(page.editor.split || "{}").on ?? false) === true;
    }

    function view(page) {
        const tones = entry(page.status.tones.saved.light ? App.tr("Dark editor") : App.tr("Light editor"), page.status.tones.saved.light ? "moon" : "sun", () => page.status.tones.toggle());
        if (!page.docks.stage)
            return [tones];
        const canvas = page.docks.stage.canvas, bar = canvas.bar;
        return [entry(App.tr("Zoom in"), "magnifying-glass-plus", () => canvas.zoom = Math.min(16, canvas.zoom * 1.25)), entry(App.tr("Zoom out"), "magnifying-glass-minus", () => canvas.zoom = Math.max(0.1, canvas.zoom / 1.25)), entry(App.tr("Fit"), "frame-corners", () => {
                canvas.zoom = 1;
                canvas.pan = Qt.point(0, 0);
            }),
            {
                "separator": true
            },
            entry(canvas.testing ? App.tr("Stop testing the pointer") : App.tr("Test the pointer"), "cursor-click", () => {
                canvas.testing = !canvas.testing;
                if (!canvas.testing)
                    page.editor.touch(-1, -1, 0);
            }), entry(App.tr("Guide…"), "frame-corners", () => bar.guides.popup.open()), entry(App.tr("Several screens…"), "monitor", () => bar.spans.clicked()), entry(App.tr("See as…"), "gauge", () => bar.seeAs.clicked()), entry(App.tr("Before and after"), "view-split-left-right", () => page.editor.halve(!halved(page)), "", (page.view.canvas?.bloom ?? null) === null),
            {
                "separator": true
            },
            tones];
    }

    function window(page) {
        const area = page.docks, layout = page.layout;
        // The panel in use: the one with the keyboard, or the one that had it before the menus did.
        const unit = area.focusUnit || (layout.groupOf(area.lastUnit) ? area.lastUnit : "");
        const open = new Set([].concat(...layout.groups.map(group => group.panels)));
        const one = unit => open.has(unit) ? entry(area.title(unit), "checkmark", () => layout.close(unit), "", !layout.canClose(unit)) : entry(area.title(unit), "", () => layout.reopen(unit));
        const shown = layout.units.filter(unit => !layout.absent.includes(unit));
        // The panels under the kind of work they belong to; a panel of none goes last.
        const kinds = [[App.tr("Scene"), ["layers", "assets", "properties", "effects"]], [App.tr("Animation"), ["timeline", "curves"]], [App.tr("Particles and cost"), ["particles", "cost"]], [App.tr("Scripts"), ["scripts", "code", "reference", "console"]], [App.tr("Simulation"), ["viewas", "inputs", "try", "screens"]], [App.tr("Wallpaper"), ["options", "sheet", "texts", "cover", "review", "problems"]]];
        const placed = new Set([].concat(...kinds.map(kind => kind[1])));
        const sorted = kinds.concat([[App.tr("Other panels"), shown.filter(unit => !placed.has(unit))]]).map(kind => [kind[0], kind[1].filter(unit => shown.includes(unit))]).filter(kind => kind[1].length > 0);
        const panels = [].concat(...sorted.map(kind => [heading(kind[0])].concat(kind[1].map(one))));
        const plans = layout.layouts.map(item => Object.assign(entry(item.name, "", () => layout.choose(item.chosen), item.key), {
                "plan": item.root,
                "current": item === layout.current
            }));
        const own = layout.layouts.some(item => item.own >= 0);
        return panels.concat([
            {
                "separator": true
            },
            heading(App.tr("Layouts"))], plans, [
            {
                "separator": true
            },
            heading(App.tr("The panel in use")), entry(App.tr("Next panel"), "arrow-right", () => area.keys?.groupAt(1), "F6", !area.keys), entry(App.tr("Detach into a window"), "arrow-square-out", () => layout.view.detach(unit), "", !unit), entry(App.tr("Detach the whole group"), "arrows-out", () => layout.view.detachAll(layout.groupOf(unit).panels), "", !unit || (layout.groupOf(unit)?.panels ?? ["canvas"]).includes("canvas") || layout.groupOf(unit).panels.length < 2), entry(layout.view.maximised ? App.tr("Back to the workbench") : App.tr("Maximize"), "frame-corners", () => layout.view.maximise(layout.view.maximised || unit), "F11", !unit && !layout.view.maximised),
            {
                "separator": true
            },
            heading(App.tr("Your layouts")), entry(App.tr("Save as new layout…"), "list-add", () => page.menus.naming.ask(-1)), entry(App.tr("Rename or remove your layouts…"), "edit-rename", () => page.menus.owning.open(), "", !own), entry(App.tr("Reset this layout"), "view-refresh", () => layout.reset())]);
    }

    function entries(page, name) {
        switch (name) {
        case "file":
            return file(page);
        case "edit":
            return edit(page);
        case "layer":
            return layer(page);
        case "view":
            return view(page);
        default:
            return window(page);
        }
    }
}
