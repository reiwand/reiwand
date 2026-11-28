import QtQuick
import io.github.reiwand.app

// The editor's tones: the light theme puts ink on paper and swaps the deep tones, and turning it
// off gives the dark set back.
QtObject {
    required property var page

    function fail(what) {
        throw new Error("editor tones: " + what);
    }

    function exercise() {
        const tones = page.status.tones;
        const was = tones.saved.light;
        if (was)
            tones.toggle();
        if (Look.light || String(Look.frost) !== "#f4efe6")
            fail("the dark editor is not dark: " + Look.frost);
        tones.toggle();
        if (!Look.light || String(Look.frost) !== "#1c1a17" || Look.night.hslLightness < 0.8 || String(Look.amber) !== "#7a5200")
            fail("the light editor is not ink on paper: " + Look.frost + " on " + Look.night);
        tones.toggle();
        if (Look.light || Look.night.hslLightness > 0.4)
            fail("the dark editor did not come back: " + Look.night);
        if (was)
            tones.toggle();
    }
}
