import QtQuick
import QtQuick.Window
import org.kde.plasma.plasmoid

WallpaperItem {
    id: root

    Rectangle {
        anchors.fill: parent
        color: "black"
    }

    Stream {
        id: stream

        anchors.fill: parent
        output: lock.stream
        covered: coverage.covered
    }

    Lock {
        id: lock

        screen: Screen.name
    }

    Text {
        anchors.centerIn: parent
        width: (parent?.width ?? 0) * 0.8
        visible: stream.error !== "" && !stream.ready
        horizontalAlignment: Text.AlignHCenter
        wrapMode: Text.Wrap
        color: "#a0a0a0"
        text: i18nd("plasma_wallpaper_io.github.reiwand", "Reiwand: %1", stream.error)
    }

    Coverage {
        id: coverage
    }

    Taps {
        anchors.fill: parent
    }

    Accent {
        wallpaper: root
    }
}
