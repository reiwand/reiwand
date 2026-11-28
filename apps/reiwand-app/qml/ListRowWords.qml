import QtQuick
import QtQuick.Controls.impl
import io.github.reiwand.app

Row {
    id: wordList

    required property Item row
    required property bool lit

    visible: row.words.length > 0
    spacing: Look.e3

    Repeater {
        model: wordList.row.words

        delegate: Row {
            id: word

            required property var modelData
            required property int index
            readonly property bool on: wordList.row.lit && wordList.row.words.length > 1 && index === wordList.row.wordAt

            spacing: Look.e1

            IconImage {
                visible: word.modelData.icon !== undefined
                anchors.verticalCenter: parent.verticalCenter
                name: word.modelData.icon ?? ""
                sourceSize: Qt.size(Look.icon, Look.icon)
                color: wordList.lit ? Look.night : Look.frost
            }

            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: word.modelData.text
                font.family: Look.textFont
                font.pixelSize: Look.tValue
                font.weight: Font.Bold
                font.underline: word.on
                color: wordList.lit ? Look.night : Look.frost
            }

            TapHandler {
                onTapped: {
                    wordList.row.forceActiveFocus(Qt.MouseFocusReason);
                    wordList.row.wordAt = word.index;
                    wordList.row.worded(word.index);
                }
            }
        }
    }
}
