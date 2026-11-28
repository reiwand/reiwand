import QtQuick
import QtQuick.Layouts
import QtQuick.Controls.impl
import io.github.reiwand.app

RowLayout {
    id: note

    property string text
    property string tone
    property string icon: Tone.icon(tone)

    spacing: Look.listGap
    Accessible.role: Accessible.StaticText
    Accessible.name: text

    IconImage {
        Layout.alignment: Qt.AlignTop
        Layout.topMargin: (Look.tFoot * 1.3 - Look.noteIcon) / 2
        visible: note.icon !== ""
        name: note.icon
        sourceSize: Qt.size(Look.noteIcon, Look.noteIcon)
        color: Tone.of(note.tone, false, Look.mist)
    }

    Text {
        Layout.fillWidth: true
        text: note.text
        wrapMode: Text.WordWrap
        font.family: Look.textFont
        font.pixelSize: Look.tFoot
        color: note.tone === "" ? Look.mist : Tone.of(note.tone, false, Look.mist)
    }
}
