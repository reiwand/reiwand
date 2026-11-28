import QtQuick
import QtQuick.Templates as T
import io.github.reiwand.app

T.Label {
    id: control

    readonly property bool heading: "level" in control

    font.family: heading ? Look.titleFont : Look.textFont
    color: enabled ? Look.frost : Look.mistLow
    linkColor: Look.moon
}
