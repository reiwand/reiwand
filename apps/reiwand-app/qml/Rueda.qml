import QtQuick

// The mouse wheel over a row of covers: one notch (120) moves to the next or the previous cover.
// Fine wheels and touchpads send smaller steps; they add up until they make a notch.
// Under Wayland a plain mouse wheel can arrive marked as a touchpad: the handler takes both.
WheelHandler {
    id: wheel

    property Item fila
    property real wheeled: 0
    readonly property bool turning: still.running
    readonly property Timer still: Timer {
        interval: 300
        onTriggered: wheel.wheeled = 0
    }

    signal moved

    function turn(dx, dy) {
        const delta = dy !== 0 ? dy : dx;
        if (delta === 0 || !fila || fila.count === 0)
            return;
        if (wheeled * delta < 0)
            wheeled = 0;
        wheeled += delta;
        still.restart();
        let moves = false;
        while (Math.abs(wheeled) >= 120) {
            const step = wheeled > 0 ? -1 : 1;
            wheeled += step * 120;
            fila.currentIndex = Math.max(0, Math.min(fila.count - 1, fila.currentIndex + step));
            moves = true;
        }
        if (moves)
            moved();
    }

    target: null
    acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
    onWheel: event => turn(event.angleDelta.x, event.angleDelta.y)
}
