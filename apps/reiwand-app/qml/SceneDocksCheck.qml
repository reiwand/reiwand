import QtQuick
import io.github.reiwand.app

QtObject {
    required property var page

    function fail(what) {
        throw new Error("docks: " + what);
    }

    function whole(layout, what) {
        const all = [].concat(layout.places.left, layout.places.right, layout.places.bottom);
        if (all.length !== layout.units.length || layout.units.some(unit => all.indexOf(unit) !== all.lastIndexOf(unit)))
            fail(what + " lost or doubled a panel: " + JSON.stringify(layout.places));
        for (const place of ["left", "right", "bottom"])
            if (layout.places[place].length && !layout.places[place].includes(layout.current(place)))
                fail(what + " shows no panel on the " + place);
        if (layout.width - layout.leftWide - layout.rightWide < Math.min(layout.canvasFloor, layout.width - layout.leftFloor - layout.rightFloor))
            fail(what + " squeezed the canvas below its floor");
    }

    function sized(layout, place) {
        layout.size(place, 0);
        const low = place === "bottom" ? layout.footHigh : place === "left" ? layout.leftWide : layout.rightWide;
        if (low !== layout.floorOf(place))
            fail("the " + place + " side went below its floor: " + low);
        layout.size(place, 100000);
        whole(layout, "the widest " + place + " side");
        if (layout.footHigh > layout.ceiling("bottom"))
            fail("the bottom went past its ceiling");
        layout.size(place);
        if (layout.kept[place] !== undefined)
            fail("a double click did not return the " + place + " side to its size");
    }

    function exercise() {
        const layout = page.layout;
        const layers = page.layers;
        for (const unit of layout.units)
            for (const place of ["left", "right", "bottom"]) {
                const from = layout.placeOf(unit);
                layout.move(unit, place);
                whole(layout, unit + " to " + place);
                if (layout.placeOf(unit) !== place || (from !== place && layout.current(place) !== unit))
                    fail(unit + " did not land on the " + place);
                if (unit === "layers" && (layers.parent.parent.parent.place !== place || layers.width !== layers.parent.parent.width))
                    fail("the layers list did not follow its tab to the " + place);
                layout.move(unit, from);
                whole(layout, unit + " back to " + from);
            }
        layout.move("properties", "bottom");
        if (layout.places.right.length || layout.rightWide !== 0)
            fail("an empty side stayed open");
        layout.reset();
        if (JSON.stringify(layout.places) !== JSON.stringify(layout.standard))
            fail("resetting the view did not put the panels back");
        for (const place of ["left", "right", "bottom"])
            sized(layout, place);
        const small = Qt.createComponent("SceneLayout.qml").createObject(page, {
            "width": 1024,
            "height": 640
        });
        small.restore(JSON.stringify({
            "places": {
                "left": ["properties", "gone", "layers", "layers"],
                "right": "nothing",
                "bottom": ["try"]
            },
            "sizes": {
                "left": 5000,
                "right": 5000,
                "bottom": 5000
            }
        }));
        whole(small, "a kept arrangement that no longer fits");
        if (small.placeOf("properties") !== "left" || small.placeOf("assets") !== "left" || small.placeOf("timeline") !== "bottom")
            fail("a kept arrangement did not keep or restore its panels: " + JSON.stringify(small.places));
        if (small.footHigh > small.ceiling("bottom"))
            fail("a kept height went past the ceiling");
        small.restore("{not json");
        whole(small, "a broken kept arrangement");
        small.destroy();
        layout.reveal("layers");
    }
}
