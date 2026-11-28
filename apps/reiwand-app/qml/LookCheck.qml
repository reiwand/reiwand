import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

QtObject {
    id: check

    required property var host
    property Timer later: Timer {
        interval: 200
        onTriggered: check.changed()
    }

    function same(found, wanted, what) {
        if (!Qt.colorEqual(found, wanted))
            throw new Error("look: " + what + " is " + found + " instead of " + wanted);
    }

    function tones(what) {
        same(host.palette.window, Look.night, what + " window");
        same(host.palette.windowText, Look.frost, what + " text");
        same(host.palette.button, Look.nightHigh, what + " button");
        same(host.palette.highlight, Look.moon, what + " highlight");
        same(host.Kirigami.Theme.backgroundColor, Look.night, what + " Kirigami background");
        same(host.Kirigami.Theme.textColor, Look.frost, what + " Kirigami text");
        same(host.Kirigami.Theme.negativeTextColor, Look.coral, what + " Kirigami negative text");
    }

    function controls() {
        const made = Qt.createQmlObject("import QtQuick\nimport QtQuick.Controls\nItem { Button { text: \"a\" } Button { text: \"b\"; highlighted: true } Button { text: \"c\"; flat: true } Switch { text: \"d\" } }", host, "LookControls");
        const quiet = made.children[0];
        const primary = made.children[1];
        const word = made.children[2];
        const key = made.children[3];
        same(quiet.background.color, Look.nightHigh, "quiet button");
        same(primary.background.color, Look.moon, "primary button");
        if (primary.font.family !== Look.titleFont || word.background.color.a !== 0)
            throw new Error("look: the primary button is not in the title font or the word button has a surface");
        if (key.indicator.implicitWidth !== Look.switchWidth)
            throw new Error("look: the switch is not drawn by the own style");
        made.destroy();
    }

    function swapping() {
        const made = Qt.createQmlObject("import QtQuick\nimport QtQuick.Controls\nItem { Button { text: \"Install\" } Button { text: \"Install\" } }", host, "LookSwap");
        const button = made.children[0];
        if (button.settling)
            throw new Error("look: a button that was never swapped ignores presses");
        button.text = "Set as wallpaper";
        if (!button.settling)
            throw new Error("look: a button that changed its action in place still takes presses");
        const hidden = made.children[1];
        hidden.visible = false;
        hidden.text = "Set as wallpaper";
        if (hidden.settling)
            throw new Error("look: a hidden button waits for nobody");
        hidden.visible = true;
        if (!hidden.settling)
            throw new Error("look: a button swapped while hidden still takes presses when shown");
        made.destroy();
    }

    function styled() {
        const files = Look.bridge.styleFiles().split(",").filter(file => file !== "");
        if (files.length === 0)
            throw new Error("look: the own style has no files");
        const probe = Qt.createQmlObject("import QtQuick.Templates\nAbstractButton {}", host, "LookProbe");
        const needs = {
            "CheckText.qml": {
                "control": probe
            },
            "FocusRing.qml": {
                "corner": Look.radius
            }
        };
        for (const file of files) {
            const url = "qrc:/qt/qml/io/github/reiwand/style/" + file;
            const component = Qt.createComponent(url);
            if (component.status !== Component.Ready)
                throw new Error("look: the style file " + file + " does not load: " + component.errorString());
            const made = component.createObject(host, needs[file] ?? {});
            if (!made)
                throw new Error("look: the style file " + file + " does not create its control");
            made.destroy();
        }
        probe.destroy();
    }

    function exercise() {
        if (Look.bridge.style() !== "io.github.reiwand.style")
            throw new Error("look: the controls style is " + Look.bridge.style());
        if (Look.bridge.family() !== Look.textFont || !Look.bridge.hasFamily(Look.titleFont))
            throw new Error("look: the text font is " + Look.bridge.family());
        const missing = Look.bridge.iconNames().split(",").filter(name => !Look.bridge.hasIcon(name));
        if (missing.length > 0)
            throw new Error("look: icons the own set does not resolve: " + missing.join(" "));
        if (Look.bridge.hasIcon("go-home") || Look.bridge.hasIcon("user-trash"))
            throw new Error("look: an icon outside the own set resolved from the system");
        tones("start");
        controls();
        swapping();
        styled();
        Look.tone({
            "night": "#1c1003",
            "nightHigh": "#2a1b09",
            "moon": "#f7afa7"
        });
        later.start();
    }

    function changed() {
        tones("after a change");
        Look.tone({
            "night": "#0a1322",
            "nightHigh": "#121e33",
            "moon": "#9cc3f5"
        });
    }
}
