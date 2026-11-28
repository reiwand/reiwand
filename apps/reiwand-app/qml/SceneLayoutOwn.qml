import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

// The layouts the author saved, each with its blueprint, "Rename…" and "Remove". Removing one asks
// once more in its row, since it cannot be undone.
Kirigami.Dialog {
    id: dialog

    required property var layout
    required property var naming
    property int asking: -1

    title: App.tr("Your layouts")
    standardButtons: QQC2.Dialog.Close
    padding: Look.e3
    onOpened: asking = -1

    ColumnLayout {
        spacing: Look.e1

        QQC2.Label {
            visible: dialog.layout.parsed.own.length === 0
            text: App.tr("You have not saved a layout yet. Window › Save as new layout… keeps the one in use under a name.")
            wrapMode: Text.Wrap
            Layout.fillWidth: true
            color: Look.mist
        }

        Repeater {
            model: dialog.layout.parsed.own

            delegate: RowLayout {
                id: line

                required property var modelData
                required property int index
                readonly property bool asked: dialog.asking === index

                Layout.fillWidth: true
                Layout.minimumHeight: DeskLook.menuRow
                spacing: Look.e2

                SceneBlueprint {
                    implicitWidth: DeskLook.planMenuWidth
                    implicitHeight: DeskLook.planMenuHeight
                    root: JSON.stringify(line.modelData.view.root)
                }

                QQC2.Label {
                    Layout.fillWidth: true
                    Layout.minimumWidth: Look.e8 * 2
                    text: line.asked ? App.tr("Remove %1? It cannot be undone.", line.modelData.name) : line.modelData.name
                    textFormat: Text.PlainText
                    elide: Text.ElideRight
                }

                QQC2.Button {
                    visible: !line.asked
                    flat: true
                    text: App.tr("Rename…")
                    onClicked: {
                        dialog.close();
                        dialog.naming.ask(line.index);
                    }
                }

                QQC2.Button {
                    flat: !line.asked
                    highlighted: line.asked
                    palette.highlight: Look.coral
                    text: line.asked ? App.tr("Remove it") : App.tr("Remove")
                    onClicked: {
                        if (!line.asked)
                            return dialog.asking = line.index;
                        dialog.asking = -1;
                        dialog.layout.remove(line.index);
                    }
                }

                QQC2.Button {
                    visible: line.asked
                    flat: true
                    text: App.tr("Cancel")
                    onClicked: dialog.asking = -1
                }
            }
        }
    }
}
