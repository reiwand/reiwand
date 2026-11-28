import QtQuick
import io.github.reiwand.app

QtObject {
    required property var page

    function exercise() {
        const log = Qt.createComponent("SceneConsolePanel.qml").createObject(page, {
                "page": page,
                "scriptError": "scripts/orbits.js:24:5 the layer moon does not exist",
                "lines": ["log scripts/orbits.js: loaded", "warn scripts/orbits.js: slow frame", "error scripts/orbits.js: no moon"]
            });
        if (log.errorFile !== "scripts/orbits.js" || log.errorLine !== 24)
            throw new Error("console: the error does not lead to its file and line: " + log.errorFile + ":" + log.errorLine);
        if (log.shown.length !== 3)
            throw new Error("console: all the lines are not shown: " + log.shown.length);
        log.kept = "warnings";
        if (log.shown.length !== 2)
            throw new Error("console: Warnings does not keep the warnings and the errors: " + log.shown.length);
        log.kept = "errors";
        if (log.shown.length !== 1 || log.fileOf(log.shown[0]) !== "scripts/orbits.js")
            throw new Error("console: Errors keeps " + JSON.stringify(log.shown));
        log.destroy();
        const problems = Qt.createComponent("SceneProblems.qml").createObject(page, {
                "page": page
            });
        if (problems.wanted <= 0)
            throw new Error("problems: the panel asks for no room");
        problems.destroy();
        // A script that stopped with an error is a problem of its own, with its line, and counts.
        const failed = Qt.createComponent("SceneProblems.qml").createObject(page, {
                "page": page,
                "scriptError": "scripts/orbits.js:24:5 the layer moon does not exist"
            });
        const row = failed.children.find(item => item.objectName === "scriptError");
        if (failed.count !== (page.view.problems ?? []).length + 1 || !row?.visible || row.title !== App.tr("Error in the script, line %1", 24) || failed.children.some(item => item.text === App.tr("No problems") && item.visible))
            throw new Error("problems: a script error is not among the problems: " + failed.count + " " + row?.title);
        if (SceneScriptFault.file(failed.scriptError) !== "scripts/orbits.js")
            throw new Error("problems: the script error does not lead to its file");
        failed.destroy();
    }
}
