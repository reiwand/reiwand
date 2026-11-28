import QtQuick
import io.github.reiwand.app

QtObject {
    required property var page

    function gather(item, out) {
        out.push(item);
        for (const child of item.children)
            gather(child, out);
        return out;
    }

    function words(item) {
        return gather(item, []).filter(at => at.checkable !== undefined && at.visible).map(at => at.text);
    }

    function exercise() {
        const bar = page.topBar.createObject(applicationWindow().contentItem);
        const shown = words(bar);
        const wanted = [App.tr("Undo"), App.tr("Redo"), App.tr("Set as wallpaper"), App.tr("Publish…")].concat(page.viewable ? [App.tr("View as wallpaper")] : []);
        const missing = wanted.filter(word => !shown.includes(word));
        if (missing.length > 0 || page.viewable !== shown.includes(App.tr("View as wallpaper")))
            throw new Error(page.kind + ": the top bar shows " + shown.join(", "));
        const menus = page.menus?.names ?? [];
        if (menus.join() !== "file,edit,view,window" || !(page.status?.height > 0))
            throw new Error(page.kind + ": the package top bar has the menus " + menus.join(", ") + (page.status?.height > 0 ? "" : " and no status bar"));
        const kits = gather(bar, []).filter(at => at.entry !== undefined).map(at => at.entry.name);
        if (kits.length < 6 || !kits.includes(BenchWords.layout("particles")))
            throw new Error(page.kind + ": the package top bar offers the layouts " + kits.join(", "));
        const publish = gather(bar, []).find(at => at.text === App.tr("Publish…"));
        publish.clicked();
        if (page.builder.visible || !page.builder.inline || !page.layout.isShown("review") || page.layout.parsed.chosen.factory !== "wallpaper")
            throw new Error(page.kind + ": Publish… does not bring the Review panel of the Wallpaper layout");
        let at = page.builder.review;
        while (at && at !== page.builder.host)
            at = at.parent;
        if (!at)
            throw new Error(page.kind + ": the steps of publishing are not in the Review panel");
        page.builder.close();
        bar.destroy();
        const named = gather(page, []).filter(at => at.checkable !== undefined).map(at => at.text);
        const tabs = [App.tr("See as"), App.tr("Wallpaper sheet"), App.tr("How it will look"), App.tr("Review"), App.tr("Try")].concat(page.kind === "web" ? [App.tr("Options")] : page.kind === "video" ? [App.tr("Presets")] : []);
        const lost = tabs.concat([App.tr("Wallpaper details…")]).filter(word => !named.includes(word));
        if (lost.length > 0 || named.includes(App.tr("Preview")) || named.includes(App.tr("Properties")))
            throw new Error(page.kind + ": the package page lacks " + lost.join(", ") + " among " + named.join(", "));
        const layout = page.layout;
        if (layout.parsed.chosen.factory !== "wallpaper" || !layout.groupOf("canvas") || !layout.groupOf("sheet"))
            throw new Error(page.kind + ": the package does not open in the Wallpaper layout: " + layout.root);
        for (const scene of ["layers", "effects", "timeline", "curves"])
            if (layout.groupOf(scene))
                throw new Error(page.kind + ": the package workbench shows " + scene);
        layout.reveal("try");
        if (layout.groupOf("try").shown !== "try")
            throw new Error(page.kind + ": Try does not come into view");
        Qt.createComponent("PackageKeepCheck.qml").createObject(page, {
            "page": page
        }).exercise();
        Qt.createComponent("SceneTonesCheck.qml").createObject(page, {
            "page": page
        }).exercise();
        Qt.createComponent("SceneTypeCheck.qml").createObject(page, {
            "page": page
        }).exercise();
        layout.flush();
    }
}
