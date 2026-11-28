import QtQuick
import io.github.reiwand.app

QtObject {
    required property var page

    function exercise() {
        const files = Qt.createComponent("SceneLayerFiles.qml").createObject(page, {
                "owner": {
                    "page": page,
                    "current": null
                }
            });
        if (files.count !== 5)
            throw new Error("files: the menu offers " + files.count + " kinds of file, not 5");
        for (const kind of ["image", "video", "sound", "mesh"]) {
            files.kind = kind;
            if (!/\(\*\.[a-z0-9]+/.test(files.dialog.nameFilters[0] ?? ""))
                throw new Error("files: no file filter for " + kind);
        }
        files.destroy();
        const calls = [];
        const importer = Qt.createComponent("SceneModelImport.qml").createObject(page, {
                "page": {
                    "view": ({}),
                    "editor": {
                        "rig": () => "null"
                    },
                    "run": (action, args) => calls.push([action, args])
                },
                "into": ""
            });
        importer.file = "/somewhere/Crate Box.glb";
        importer.accepted();
        if (calls.length !== 1 || calls[0][0] !== "add_layer" || calls[0][1].kind !== "model" || calls[0][1].file !== "/somewhere/Crate Box.glb" || calls[0][1].name !== "crate_box")
            throw new Error("files: accepting a 3D model file does not add its layer in one step: " + JSON.stringify(calls));
        importer.destroy();
        dropping();
    }

    // Many files dropped at once: one step, the sounds stop at the scene's limit, the rest is told.
    function dropping() {
        const calls = [];
        const host = {
            "view": {
                "layers": Array.from({
                    "length": 7
                }, () => ({
                            "type": "sound"
                        }))
            },
            "editor": {
                "error": ""
            },
            "notice": "",
            "run": (action, args) => calls.push([action, args])
        };
        const drop = Qt.createComponent("SceneDropFiles.qml").createObject(page, {
                "page": host,
                "layers": true
            });
        drop.take(["/d/a.ogg", "/d/b.ogg", "/d/c.ogg", "/d/one.png", "/d/two.png", "/d/note.txt"]);
        // The page given to the drop is a copy of the host, so its notice is read back from the drop.
        const notice = drop.page.notice ?? "";
        drop.destroy();
        const added = calls[0]?.[1] ?? [];
        if (calls.length !== 1 || calls[0][0] !== "steps" || added.length !== 3 || added.filter(a => a.add_layer?.kind === "sound").length !== 1)
            throw new Error("files: a drop of many files is not one step within the sound limit: " + JSON.stringify(calls));
        if (!notice.includes("2") || !notice.includes("1 "))
            throw new Error("files: the drop does not tell what it left out: " + notice);
    }
}
