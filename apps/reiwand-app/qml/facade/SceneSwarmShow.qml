pragma Singleton

import QtQuick

// Which handles of the chosen particle system the canvas draws: the emitters, the forces (pulls,
// swirls and keeps), the control points and the colliders. All are drawn until the author hides
// some from the canvas bar; the choice lasts while the app runs and is not saved.
QtObject {
    readonly property var kinds: ["emitters", "forces", "points", "colliders"]
    property var off: []

    function shown(kind) {
        return !off.includes(kind);
    }

    function flip(kind) {
        off = shown(kind) ? off.concat([kind]) : off.filter(other => other !== kind);
    }
}
