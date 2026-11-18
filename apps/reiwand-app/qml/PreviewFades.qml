import QtQuick
import io.github.reiwand.app

QtObject {
    id: fades

    required property var preview
    required property var dim
    property alias opening: opening
    property alias leaving: leaving
    property alias idle: idleTimer

    readonly property Timer idleTimer: Timer {
        id: idleTimer

        interval: fades.preview.quiet
        onTriggered: fades.preview.press(Qt.Key_Escape)
    }

    readonly property Timer waiting: Timer {
        id: waiting

        interval: PreviewLook.espera
        running: !fades.preview.began && !fades.preview.warm.resting
        onTriggered: fades.preview.fail({})
    }

    readonly property NumberAnimation inAnim: NumberAnimation {
        id: opening

        target: fades.dim
        property: "shade"
        from: 0
        to: 1
        duration: fades.preview.calm ? 0 : PreviewLook.aparece
        easing.type: Easing.OutCubic
    }

    readonly property NumberAnimation outAnim: NumberAnimation {
        id: leaving

        target: fades.dim
        property: "shade"
        to: 0
        duration: fades.preview.calm ? 0 : PreviewLook.sale
        easing.type: Easing.OutCubic
        onFinished: fades.preview.left === 0 ? fades.preview.warm.rest() : Qt.exit(fades.preview.left)
    }
}
