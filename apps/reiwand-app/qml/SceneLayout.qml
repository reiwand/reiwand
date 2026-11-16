import QtQuick
import QtCore as Core
import io.github.reiwand.app

QtObject {
    id: layout

    property real width: 1600
    property real height: 900
    readonly property var standard: ({
            "left": ["layers", "assets", "options", "try"],
            "right": ["properties"],
            "bottom": ["timeline", "problems"]
        })
    readonly property var units: [].concat(standard.left, standard.right, standard.bottom)
    property var places: standard
    property var shown: ({})
    property var kept: ({})
    property real leftNeed: 0
    property real rightNeed: 0
    readonly property real leftFloor: floorFor("left", leftNeed)
    readonly property real rightFloor: floorFor("right", rightNeed)
    readonly property real canvasFloor: Look.lateralDer
    readonly property real footFloor: Look.barraLienzoAlto * 2
    readonly property real leftWant: places.left.length ? Math.max(kept.left ?? Math.min(Math.max(width * 0.22, Look.lateralIzq), Look.lateralIzq * 1.5), leftFloor) : 0
    readonly property real rightWant: places.right.length ? Math.max(kept.right ?? Math.min(Math.max(width * 0.25, Look.lateralDer), Look.lateralDer * 1.6), rightFloor) : 0
    readonly property real over: Math.max(0, leftWant + rightWant - (width - canvasFloor))
    readonly property real leftWide: leftWant ? Math.round(Math.max(leftFloor, leftWant - over * leftWant / (leftWant + rightWant))) : 0
    readonly property real rightWide: rightWant ? Math.round(Math.max(rightFloor, rightWant - over * rightWant / (leftWant + rightWant))) : 0
    readonly property real footHigh: places.bottom.length ? Math.round(Math.min(Math.max(kept.bottom ?? height * 0.24, footFloor), ceiling("bottom"))) : 0
    readonly property Core.Settings saved: Core.Settings {
        property string arrangement: ""

        location: Core.StandardPaths.writableLocation(Core.StandardPaths.ConfigLocation) + "/reiwand/window.conf"
        category: "editor"
    }

    function ceiling(place) {
        if (place === "bottom")
            return Math.max(footFloor, height - Look.barraLienzoAlto * 2 - canvasFloor * 0.5);
        return Math.max(floorOf(place), width - canvasFloor - (place === "left" ? rightWide : leftWide));
    }

    function floorFor(place, need) {
        const base = places[place].includes("properties") ? Look.lateralDer : Look.lateralIzq;
        return Math.max(base, Math.min(need, base * 1.5));
    }

    function floorOf(place) {
        return place === "bottom" ? footFloor : place === "left" ? leftFloor : rightFloor;
    }

    function size(place, value) {
        const next = Object.assign({}, kept);
        if (value === undefined)
            delete next[place];
        else
            next[place] = Math.round(Math.min(Math.max(value, floorOf(place)), ceiling(place)));
        kept = next;
        store();
    }

    function placeOf(unit) {
        return ["left", "right", "bottom"].find(place => places[place].includes(unit)) ?? "";
    }

    function current(place) {
        const list = places[place];
        return list.includes(shown[place]) ? shown[place] : list[0] ?? "";
    }

    function reveal(unit) {
        const next = Object.assign({}, shown);
        next[placeOf(unit)] = unit;
        shown = next;
    }

    function move(unit, place) {
        if (!units.includes(unit) || !places[place] || placeOf(unit) === place)
            return;
        const next = {};
        for (const at of ["left", "right", "bottom"])
            next[at] = places[at].filter(item => item !== unit);
        next[place] = next[place].concat([unit]);
        places = next;
        reveal(unit);
        store();
    }

    function reset() {
        places = standard;
        shown = ({});
        kept = ({});
        store();
    }

    function sane(read) {
        const next = {};
        const seen = [];
        for (const at of ["left", "right", "bottom"]) {
            next[at] = (Array.isArray(read?.[at]) ? read[at] : []).filter((unit, at, list) => units.includes(unit) && !seen.includes(unit) && list.indexOf(unit) === at);
            seen.push(...next[at]);
        }
        for (const at of ["left", "right", "bottom"])
            next[at] = next[at].concat(standard[at].filter(unit => !seen.includes(unit)));
        return next;
    }

    function restore(text) {
        let read = {};
        try {
            read = JSON.parse(text || "{}");
        } catch (error) {
            read = {};
        }
        places = sane(read.places);
        const sizes = {};
        for (const at of ["left", "right", "bottom"])
            if (typeof read.sizes?.[at] === "number")
                sizes[at] = read.sizes[at];
        kept = sizes;
    }

    function store() {
        if (!App.checking)
            saved.arrangement = JSON.stringify({
                "places": places,
                "sizes": kept
            });
    }

    Component.onCompleted: restore(App.checking ? "" : saved.arrangement)
}
