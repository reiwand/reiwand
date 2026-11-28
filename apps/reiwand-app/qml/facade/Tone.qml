pragma Singleton

import QtQuick
import io.github.reiwand.app

QtObject {
    function of(tone, lit, rest) {
        if (tone === "amber")
            return lit ? Look.amberDeep : Look.amber;
        if (tone === "coral")
            return lit ? Look.coralDeep : Look.coral;
        if (tone === "sprout")
            return lit ? Look.sproutDeep : Look.sprout;
        return rest;
    }

    function icon(tone) {
        return tone === "sprout" ? "check-circle" : tone === "amber" ? "warning" : tone === "coral" ? "warning-circle" : "info";
    }
}
