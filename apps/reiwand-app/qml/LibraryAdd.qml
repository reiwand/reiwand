import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

PanelChico {
    id: add

    required property var owner
    property var files: []
    property bool copy: false
    property bool typing: false
    property var jobs: []
    property var ended: []
    readonly property var typed: area.text.split("\n").map(line => line.trim()).filter(line => line !== "").map(line => line.startsWith("file://") ? App.path(line) : line)
    readonly property var paths: typing ? typed : files
    readonly property int inner: Look.panelChico - 2 * Look.e4
    readonly property alias report: report

    function start(from) {
        opener = from;
        files = [];
        typing = false;
        area.text = "";
        open();
        chooser.open();
    }

    function pick() {
        packages.open();
    }

    function bring(paths, copy) {
        const wpak = paths.filter(path => path.toLowerCase().endsWith(".wpak"));
        const rest = paths.filter(path => !wpak.includes(path));
        if (wpak.length > 0)
            App.act("ImportPackages", [wpak, false], "Library1", reply => add && add.follow(Number(Array.isArray(reply) ? reply[0] : reply)));
        if (rest.length > 0)
            App.act("Import", [rest, copy], "Library1", reply => add && add.follow(Number(Array.isArray(reply) ? reply[0] : reply)));
    }

    function follow(job) {
        if (ended.includes(job))
            return done(job);
        jobs = jobs.concat([job]);
    }

    function done(job) {
        App.query("GetJob", [job], "Daemon1", reply => {
                if (!add)
                    return;
                const output = reply.job.output ?? {};
                const one = (output.imported ?? []).length === 1 ? output.imported[0] : (output.imported ?? []).length === 0 && (output.kept ?? []).length === 1 ? output.kept[0] : null;
                if (!one)
                    return add.tell(output, "");
                App.query("GetItem", [one.uid], "Library1", item => add && add.tell(output, item.item ? Names.of(item.item) : ""));
            });
    }

    function tell(output, name) {
        const imported = output.imported ?? [];
        const kept = output.kept ?? [];
        const failed = output.failed ?? [];
        const newer = kept.filter(one => one.newer);
        if (imported.length + kept.length + failed.length === 0)
            return;
        const window = applicationWindow();
        const library = page => !!page?.covers && !!page?.tools?.dialogs?.importer;
        const reach = act => {
            if (!library(window.pageStack.currentItem))
                App.navigate(0, "");
            const page = window.pageStack.currentItem;
            if (library(page))
                act(page);
        };
        const uids = imported.length > 0 ? imported.map(one => one.uid) : kept.map(one => one.uid);
        const see = () => reach(page => uids.length === 1 ? page.open(uids[0]) : page.covers.newest());
        if (failed.length === 0 && imported.length === 0 && newer.length === 1 && kept.length === 1)
            return window.showPassiveNotification(App.tr("A newer version of this wallpaper is already in your library."), "long", App.tr("Use the older one"), () => reach(page => page.tools.dialogs.importer.older([newer[0].path])));
        const said = [];
        if (imported.length === 1 && name !== "")
            said.push(App.tr("“%1” was added.", name));
        else if (imported.length > 0)
            said.push(imported.length === 1 ? App.tr("1 wallpaper was added.") : App.tr("%1 wallpapers were added.", imported.length));
        if (imported.length === 0 && kept.length === 1 && name !== "")
            said.push(App.tr("“%1” was already in your library.", name));
        else if (kept.length > 0)
            said.push(kept.length === 1 ? App.tr("1 was already in your library.") : App.tr("%1 were already in your library.", kept.length));
        if (failed.length > 0)
            said.push(App.tr("%1 could not be added.", failed.length));
        if (failed.length > 0 || newer.length > 0)
            return window.showPassiveNotification(said.join(" "), "long", App.tr("Details"), () => reach(page => page.tools.dialogs.importer.details(output)));
        window.showPassiveNotification(said.join(" "), "long", App.tr("See"), see);
    }

    function older(paths) {
        App.act("ImportPackages", [paths, true], "Library1", reply => add && add.follow(Number(Array.isArray(reply) ? reply[0] : reply)));
    }

    function details(output) {
        report.tell(output);
    }

    title: paths.length > 1 ? App.tr("Add %1 wallpapers", paths.length) : paths.length === 1 ? App.tr("Add 1 wallpaper") : App.tr("Add wallpapers")
    text: App.tr("Copy them to your library or leave them where they are?")
    quiet: App.tr("Cancel")
    principal: App.tr("Add")
    first: copy ? copying : leaving
    onAccepted: paths.length > 0 ? bring(paths, copy) : typing || chooser.open()
    foot: [
        QQC2.Button {
            flat: true
            text: add.typing ? App.tr("Choose files…") : App.tr("Write the paths…")
            onClicked: {
                if (add.typing)
                    return chooser.open();
                area.text = add.files.join("\n");
                add.typing = true;
                area.forceActiveFocus();
            }
        }
    ]

    LibraryAddWay {
        id: copying

        width: add.inner
        text: App.tr("Copy them to your library")
        reason: App.tr("They stay here even if you move or delete the originals.")
        marked: add.copy
        onPicked: add.copy = true
    }

    LibraryAddWay {
        id: leaving

        width: add.inner
        text: App.tr("Leave them where they are")
        reason: App.tr("They take no extra space; if you move the folder, they stop showing.")
        marked: !add.copy
        onPicked: add.copy = false
    }

    Area {
        id: area

        visible: add.typing
        width: add.inner
        height: Look.e6 * 4
        placeholderText: App.tr("One path per line")
        Accessible.name: App.tr("Paths to import")
    }

    Chooser {
        id: packages

        mode: Chooser.Files
        title: App.tr("Import wallpaper")
        nameFilters: [App.tr("Wallpaper packages (*.wpak)")]
        onAccepted: add.bring(Array.from(selectedFiles, file => App.path(file)), true)
    }

    Chooser {
        id: chooser

        mode: Chooser.Files
        onAccepted: {
            add.files = Array.from(selectedFiles, file => App.path(file));
            add.typing = false;
            add.opened || add.open();
        }
    }

    LibraryAddReport {
        id: report

        parent: add.parent
        onOlder: paths => add.older(paths)
        onSee: uids => uids.length === 1 ? add.owner.page.open(uids[0]) : add.owner.page.covers.newest()
    }

    Connections {
        target: App.backend

        function onJobFinished(job, state) {
            add.ended = add.ended.slice(-15).concat([job]);
            if (!add.jobs.includes(job))
                return;
            add.jobs = add.jobs.filter(other => other !== job);
            if (state === "done")
                add.done(job);
        }
    }
}
