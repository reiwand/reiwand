import QtQuick
import io.github.reiwand.app

// Under the canvas, an effect the preview leaves out is said, with the reason the desktop gives for
// its screens, and nothing shows while the preview draws every effect.
QtObject {
    required property var page

    function fail(what) {
        throw new Error("effects left out: " + what);
    }

    function exercise() {
        const strip = Qt.createComponent("SceneLeftOut.qml").createObject(page, {
            "page": page,
            "width": 600
        });
        if (!strip)
            fail("the strip under the canvas does not build");
        strip.lines = [];
        if (strip.visible)
            fail("the strip shows with nothing left out");
        strip.lines = ["the effect haze was left out: it is too slow for this graphics card"];
        // The same words as the screens of the desktop: in English the line as the daemon says it.
        const said = Qt.createComponent("LeftOut.qml").createObject(strip, {
            "daemon": App
        }).text(strip.lines[0]);
        if (!strip.visible || strip.text !== said || !/haze/i.test(said) || !said.includes(App.tr("it is too slow for this graphics card")))
            fail("the strip says \"" + strip.text + "\" instead of \"" + said + "\"");
        strip.destroy();
    }
}
