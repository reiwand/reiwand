pragma Singleton

import QtQuick

QtObject {
    readonly property int focusWidth: 520
    readonly property int nearWidth: 264
    readonly property int farWidth: 150
    readonly property int gap: 18
    readonly property int panelSide: 36
    readonly property int panelTop: 24
    readonly property int panelBottom: 22
    readonly property int panelRadius: 14
    readonly property int cardRadius: 9
    readonly property int ringGap: 4
    readonly property int ring: 3
    readonly property int pace: 190
    readonly property int appear: 170
    readonly property int leave: 150
    readonly property int wait: 5000
    readonly property int rest: 5000
    readonly property int warm: 300000
    readonly property real veil: 0.55
    readonly property real nearOpacity: 0.78
    readonly property real farOpacity: 0.36
    readonly property real wheelStep: 120
    readonly property real tTitle: 15
    readonly property real tName: 24
    readonly property real tSub: 15
    readonly property real tKey: 13
    readonly property real tBadge: 12

    function edge(pixels) {
        return [256, 512, 1024, 2048, 4096].find(size => size >= pixels) ?? 4096;
    }

    function unit(width) {
        return Math.max(0.75, Math.min(1.6, width / 1920));
    }
}
