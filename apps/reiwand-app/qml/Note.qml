import QtQuick
import QtQuick.Controls.impl
import io.github.reiwand.app

Row {
    id: note

    property string text
    property string tone
    property string icon: Tone.icon(tone)
    property bool lit: false
    property real fitWidth: 0
    property bool wrap: false
    property bool wraps: false

    spacing: Look.listGap
    Accessible.role: Accessible.StaticText
    Accessible.name: text

    IconImage {
        id: mark

        anchors.verticalCenter: parent.verticalCenter
        visible: note.icon !== ""
        name: note.icon
        sourceSize: Qt.size(Look.noteIcon, Look.noteIcon)
        color: Tone.of(note.tone, note.lit, note.lit ? Look.night : Look.mist)
    }

    // Wrapping, the words take the width the note is given: in whole pixels, so that on a
    // fractional scale the width the row reports back stays the one it got.
    Text {
        anchors.verticalCenter: parent.verticalCenter
        width: note.wraps ? Math.floor(note.width - (note.icon !== "" ? Look.noteIcon + note.spacing : 0)) : note.fitWidth > 0 ? Math.min(implicitWidth, note.fitWidth - (mark.visible ? mark.width + note.spacing : 0)) : implicitWidth
        text: note.text
        wrapMode: note.wrap || note.wraps ? Text.Wrap : Text.NoWrap
        elide: note.wrap || note.wraps ? Text.ElideNone : Text.ElideRight
        font.family: Look.textFont
        font.pixelSize: Look.tFoot
        color: note.lit ? Look.night : Look.mist
    }
}
