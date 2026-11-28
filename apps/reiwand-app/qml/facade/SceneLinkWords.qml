pragma Singleton
import QtQuick

QtObject {
    readonly property var curves: ({
            "back": App.tr("overshoot"),
            "bounce": App.tr("bouncing"),
            "circ": App.tr("circular"),
            "cubic": App.tr("cubic"),
            "elastic": App.tr("elastic"),
            "expo": App.tr("exponential"),
            "quad": App.tr("quadratic"),
            "quart": App.tr("quartic"),
            "quint": App.tr("quintic"),
            "sine": App.tr("sine")
        })
    readonly property var sources: ({
            "audio": App.tr("the sound"),
            "clock": App.tr("the clock"),
            "cursor": App.tr("the pointer"),
            "date": App.tr("the date"),
            "keys": App.tr("the keyframes"),
            "media": App.tr("the media player"),
            "prop": App.tr("a user property"),
            "script": App.tr("a script")
        })
}
