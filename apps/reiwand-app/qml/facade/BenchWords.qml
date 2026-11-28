pragma Singleton
import QtQuick
import io.github.reiwand.app

// The names and icons of the editor workbench: its panels and the layouts it comes with.
QtObject {
    function icon(unit) {
        return {
            "canvas": "frame-corners",
            "layers": "stack",
            "assets": "folder-simple",
            "options": "gear-six",
            "try": "cursor-click",
            "properties": "sliders-horizontal",
            "effects": "sparkle",
            "timeline": "clock",
            "curves": "bezier-curve",
            "problems": "warning",
            "viewas": "gauge",
            "inputs": "cursor",
            "screens": "monitor",
            "sheet": "identification-card",
            "texts": "translate",
            "cover": "image",
            "review": "seal-check",
            "particles": "snowflake",
            "cost": "chart-bar",
            "scripts": "file-code",
            "code": "brackets-curly",
            "reference": "books",
            "console": "list"
        }[unit] ?? "";
    }

    function layout(kit) {
        return {
            "scene": App.tr("Scene"),
            "particles": App.tr("Particles"),
            "scripts": App.tr("Scripts"),
            "simulation": App.tr("Simulation"),
            "animation": App.tr("Animation"),
            "wallpaper": App.tr("Wallpaper")
        }[kit] ?? kit;
    }
}
