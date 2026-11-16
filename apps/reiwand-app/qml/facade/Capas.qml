pragma Singleton

import QtQuick
import io.github.reiwand.app

QtObject {
    id: capas

    readonly property var screens: App.state.outputs ?? {}
    readonly property var jobs: (App.jobs ?? []).filter(job => job.kind === "layers" && (job.state === "queued" || job.state === "running"))

    function onScreen(screen, list) {
        const file = String(screen?.image ?? "");
        if (file === "")
            return null;
        const job = (list ?? capas.jobs).find(job => job.kind === "layers" && (job.state === "queued" || job.state === "running") && job.target === file);
        if (job)
            return {
                "state": job.state,
                "done": job.state === "running" && job.total > 0 ? job.done / job.total : job.state === "running" ? 0 : -1,
                "file": file
            };
        const still = String(screen.still ?? "");
        if (still === "building" || still === "building_simple")
            return {
                "state": "queued",
                "done": -1,
                "file": file
            };
        if (still === "failed" || still === "cancelled")
            return {
                "state": still,
                "done": -1,
                "file": file
            };
        return null;
    }

    function of(uid) {
        if (!uid)
            return null;
        for (const name in capas.screens) {
            const screen = capas.screens[name];
            if (screen.item === uid) {
                const found = capas.onScreen(screen);
                if (found)
                    return found;
            }
        }
        return null;
    }

    function note(found) {
        if (!found || found.state === "cancelled")
            return "";
        if (found.state === "failed")
            return App.tr("Could not generate the layers");
        if (found.state === "running")
            return App.tr("Generating layers · %1 %", Math.round(found.done * 100));
        return App.tr("Waiting to generate layers");
    }

    function icon(found) {
        return !found ? "" : found.state === "failed" ? "warning-circle" : found.state === "running" ? "stack" : "hourglass-medium";
    }

    function tone(found) {
        return found && found.state === "failed" ? "coral" : "";
    }

    function bar(found) {
        return found && found.state === "running" ? found.done : found && found.state === "queued" ? 0 : -1;
    }
}
