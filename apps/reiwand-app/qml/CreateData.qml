import QtQuick
import io.github.reiwand.app

QtObject {
    id: facts

    required property var page
    readonly property var story: CreateStarters {
    }
    readonly property var mine: page.projects.map(entry => card(entry))
    readonly property var recentModel: {
        const head = mine.filter(entry => entry.recent !== false).slice(0, page.fits);
        return mine.length === 0 ? head : head.concat([{
                    "all": true,
                    "icon": "squares-four",
                    "name": App.tr("All my projects"),
                    "note": String(mine.length)
                }]);
    }
    readonly property string needle: page.query.trim().toLowerCase()
    readonly property var allModel: needle === "" ? mine : mine.filter(entry => entry.name.toLowerCase().includes(needle))
    readonly property var sceneModel: story.scene
    readonly property var singleModel: story.single
    readonly property var moreModel: story.more
    readonly property var learnModel: page.scenes.map(item => ({
                "id": item.name,
                "name": item.title,
                "kind": item.kind,
                "license": item.license ?? "",
                "picture": item.picture ?? ""
            }))
    readonly property string footText: !page.current || page.onAll ? "" : page.onRecent ? [SceneWords.kind(page.current.kind), page.current.version ? App.tr("version %1", page.current.version) : "", App.tr("edited %1", ago(page.current.edited))].filter(part => part !== "").join(" · ") : page.atExamples ? [SceneWords.kind(page.current.kind), App.tr("built into the app"), page.current.license].filter(part => part !== "").join(" · ") : ""

    function card(entry) {
        return {
                "folder": entry.folder,
                "kind": entry.kind,
                "version": entry.version ?? "",
                "edited": entry.edited,
                "missing": entry.missing,
                "name": entry.title,
                "picture": entry.poster ? App.url(entry.poster) : "",
                "icon": "stack",
                "off": entry.missing,
                "note": entry.missing ? App.tr("Its folder is gone") : App.tr("Edited %1", ago(entry.edited)),
                "noteIcon": entry.missing ? "warning-circle" : "",
                "noteTone": entry.missing ? "coral" : "",
                "recent": entry.recent !== false
            };
    }

    // In "All my projects" the projects open and have actions only from their grid; the search
    // field and Back lead down to it.
    readonly property var hints: page.all && !page.every.grid.activeFocus ? [{
            "keys": ["down"],
            "text": App.tr("See your projects")
        }, {
            "keys": ["Esc"],
            "text": App.tr("Back")
        }] : page.all ? [{
            "keys": ["left", "right", "up", "down"],
            "text": App.tr("Move around")
        }, {
            "keys": ["Intro"],
            "text": App.tr("Open")
        }, {
            "keys": ["Esc"],
            "text": App.tr("Back")
        }, {
            "keys": [App.trc("key", "Menu")],
            "text": App.tr("Project actions")
        }] : [{
            "keys": ["left", "right"],
            "text": App.tr("Move around")
        }, {
            "keys": ["Tab"],
            "text": App.tr("Change row")
        }].concat(page.onRecent ? [{
                "keys": [App.trc("key", "Menu")],
                "text": App.tr("Project actions")
            }] : [])

    function exercise() {
        if (page.scenes.length === 0)
            throw new Error("the create page offers no built-in example to copy");
        for (const part of [["page", "web"], ["clip", "video"], ["still", "image"], ["preset", "preset"]])
            if (App.backend.claim("compose." + part[0]))
                App.compose(page.helper.folderFor(part[0]), part[1], "");
        App.compose(page.helper.folderFor("check"), "empty", "");
    }

    function make(starter) {
        if (starter === "image" || starter === "video")
            return page.media.start(starter);
        page.create(starter, "");
    }

    function copyOf(entry) {
        const made = JSON.parse(page.helper.examplePackage(entry.id));
        if (made.error !== undefined)
            return applicationWindow().showPassiveNotification({
                    "text": made.error,
                    "tone": "coral"
                }, 12000);
        page.copy.start(made.path, page.rows.learn.view.currentItem || page.foot.primary, entry.picture);
    }

    function forget() {
        page.recents.forgetProject(page.current.folder);
        reload();
    }

    function reload() {
        page.projects = JSON.parse(page.recents.projects() || "[]");
        page.reset();
    }

    function hide(folder) {
        page.recents.hideProject(folder);
        reload();
    }

    function drop(folder, name) {
        const before = page.recents.snapshot();
        page.recents.forgetProject(folder);
        reload();
        undoable(App.tr("“%1” left the list. Its files stay where they are.", name), before);
    }

    function clear() {
        const before = page.recents.snapshot();
        page.recents.clearRecents();
        reload();
        undoable(App.tr("The recent projects were cleared. Their files stay where they are."), before);
    }

    function undoable(text, before) {
        applicationWindow().showPassiveNotification({
            "text": text
        }, 8000, App.tr("Undo"), () => facts && (facts.page.recents.restore(before), facts.reload()));
    }

    function load() {
        page.scenes = JSON.parse(page.helper.examples() || "[]");
        page.reset();
        page.loaded = true;
    }

    function ago(seconds) {
        const minutes = Math.round((Date.now() / 1000 - seconds) / 60);
        if (minutes < 2)
            return App.tr("just now");
        if (minutes < 60)
            return App.tr("%1 minutes ago", minutes);
        const hours = Math.round(minutes / 60);
        if (hours < 24)
            return hours === 1 ? App.tr("an hour ago") : App.tr("%1 hours ago", hours);
        const days = Math.round(hours / 24);
        return days === 1 ? App.tr("yesterday") : App.tr("%1 days ago", days);
    }

    function refuse(folder) {
        const name = folder.split("/").filter(part => part !== "").slice(-1).join("");
        applicationWindow().showPassiveNotification({
            "text": App.tr("“%1” is not a project. A project's folder holds a file manifest.json and, for a scene, its scene.json: choose the folder that holds them.", name),
            "tone": "coral"
        }, 12000, App.tr("Choose another"), () => facts.page && facts.page.pickFolder());
    }
}
