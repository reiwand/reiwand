import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2

// A check box whose words wrap in the panel's width, the first line on the middle of the box: a
// long sentence never sets the width of a narrow panel. A click on the words checks it as well.
RowLayout {
    id: line

    property alias text: words.text
    property alias checked: box.checked

    signal toggled

    Layout.fillWidth: true

    QQC2.CheckBox {
        id: box

        Layout.alignment: Qt.AlignTop
        Accessible.name: words.text
        onToggled: line.toggled()
    }

    QQC2.Label {
        id: words

        Layout.fillWidth: true
        Layout.alignment: Qt.AlignTop
        Layout.topMargin: Math.max(0, (box.implicitHeight - contentHeight / Math.max(1, lineCount)) / 2)
        wrapMode: Text.Wrap

        TapHandler {
            onTapped: {
                box.toggle();
                line.toggled();
            }
        }
    }
}
