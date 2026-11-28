import QtQuick
import io.github.reiwand.app

// How it will look sets the cover next to another one at rest and offers Record 6 s; the Screens
// panel maps the thumbnail onto each chosen screen, two for two side by side.
QtObject {
    required property var page

    function fail(what) {
        throw new Error("cover and screens: " + what);
    }

    function gather(item, out) {
        out.push(item);
        for (const child of item.children)
            gather(child, out);
        return out;
    }

    function exercise() {
        const beside = Qt.createComponent("SceneCoverBeside.qml").createObject(page, {
            "page": page
        });
        if (beside.picture === "" || beside.other.title === page.view.title)
            fail("no other cover to sit beside: " + JSON.stringify(beside.other?.title));
        beside.destroy();
        const cover = Qt.createComponent("SceneCoverPanel.qml").createObject(page, {
            "page": page,
            "width": 640,
            "height": 600
        });
        const record = gather(cover, []).find(item => item.text === App.tr("Record 6 s"));
        if (!record || !record.visible)
            fail("How it will look has no Record 6 s");
        cover.destroy();
        const map = Qt.createComponent("SceneScreensMap.qml").createObject(page, {
            "page": page,
            "width": 320,
            "layout": "dual"
        });
        if (map.parts.length !== 2 || !(map.implicitHeight > 0) || Math.abs(map.bounds[2] * map.scale - map.width) > 0.5)
            fail("the map of two screens has " + JSON.stringify(map.parts));
        map.layout = "";
        if (map.visible)
            fail("the map shows with one screen");
        map.destroy();
    }
}
