import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Rectangle {
    id: cap

    required property string key
    readonly property bool arrow: ["left", "right", "up", "down"].includes(key)
    readonly property string shown: key === "Intro" ? App.trc("key", "Enter") : key

    implicitWidth: Math.max(Look.keyWidth, word.implicitWidth + Look.e2 * 2)
    implicitHeight: Look.keyHeight
    radius: Look.keyRadius
    color: Look.nightHigh
    border.width: Look.line
    border.color: Look.nightLine
    Accessible.ignored: true

    Rectangle {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.leftMargin: cap.radius
        anchors.rightMargin: cap.radius
        height: Look.line
        color: Look.nightLine
    }

    Kirigami.Icon {
        anchors.centerIn: parent
        width: Look.keyIcon
        height: Look.keyIcon
        visible: cap.arrow
        source: cap.key === "left" ? "arrow-left" : cap.key === "right" ? "arrow-right" : cap.key === "up" ? "arrow-up" : cap.key === "down" ? "arrow-down" : ""
    }

    Text {
        id: word

        anchors.centerIn: parent
        visible: !cap.arrow
        text: cap.arrow ? "" : cap.shown
        color: Look.frost
        font.family: Look.textFont
        font.pixelSize: Look.tKey
        font.weight: Font.DemiBold
    }
}
