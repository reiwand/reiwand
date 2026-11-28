import QtQuick
import QtQuick.Controls as QQC2

// The volume of a screen whose wallpaper has sound: it sends what is chosen a moment after the pointer
// stops, and the state coming back does not move it while it is held or while what it sent is on its way.
QQC2.Slider {
    id: loudness

    required property var host

    visible: host.sound !== ""
    from: 0
    to: 1
    onMoved: debounce.restart()

    Binding on value {
        value: loudness.host.daemon.state?.volume
        when: !loudness.pressed && !debounce.running && !settle.running
        restoreMode: Binding.RestoreNone
    }

    Timer {
        id: debounce

        interval: 200
        onTriggered: {
            loudness.host.daemon.act("SetVolume", [loudness.value]);
            loudness.host.daemon.act("SetMuted", [false]);
            settle.restart();
        }
    }

    Timer {
        id: settle

        interval: 600
    }
}
