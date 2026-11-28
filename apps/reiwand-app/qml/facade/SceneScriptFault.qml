pragma Singleton

import QtQuick

// The error a script of the preview stopped with (from the daemon's state of the preview), shared by
// Problems, its count in the status bar and in the tab, and Console: where it happened and the
// problems it adds to those of the scene.
QtObject {
    function of(page) {
        return App.state.previews?.[page.editor.output]?.error ?? "";
    }

    function file(error) {
        return (error.match(/([\w\/.-]+\.(?:js|mjs|ts)):\d+/) ?? [])[1] ?? "";
    }

    function line(error) {
        return Number((error.match(/line (\d+)/) ?? error.match(/:(\d+)(?::\d+)?\b/) ?? [])[1] ?? 0);
    }

    // The problems of the scene and the script error, if there is one.
    function count(page) {
        return (page.view.problems ?? []).length + (of(page) !== "" ? 1 : 0);
    }

    // Opens the script at the line of the error in Code.
    function open(page, error) {
        const code = page.docks?.code;
        if (!code || file(error) === "")
            return;
        page.layout?.reveal("code");
        code.openScript(file(error));
        if (line(error) > 0)
            code.script.jump(line(error));
    }
}
