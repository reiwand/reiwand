import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

ColumnLayout {
    id: field

    property alias text: area.text
    property string label

    signal committed(string text)

    spacing: Kirigami.Units.smallSpacing
    baselineOffset: area.y + area.baselineOffset

    Area {
        id: area

        Layout.fillWidth: true
        wrapMode: TextEdit.Wrap
        Accessible.name: field.label
        onEditingFinished: field.committed(text)
    }

    QQC2.Label {
        text: App.tr("How it looks")
        color: Kirigami.Theme.disabledTextColor
    }

    MarkupView {
        Layout.fillWidth: true
        source: area.text
    }
}
