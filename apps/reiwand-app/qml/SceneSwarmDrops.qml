import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

ColumnLayout {
    id: drop

    required property var drops

    Repeater {
        model: drop.drops

        delegate: Item {
            id: one

            required property var modelData
            required property int index

            Layout.fillWidth: true
            implicitHeight: Kirigami.Units.gridUnit * 2

            Rectangle {
                objectName: "stretchDrop" + one.index
                readonly property real unit: one.width * 0.8 / Math.max(drop.drops[0].long, drop.drops[1].long, 1)

                width: one.modelData.long * unit
                height: Math.max(one.modelData.side * unit, 2)
                x: one.width * 0.9 - width
                anchors.verticalCenter: parent.verticalCenter
                radius: height / 2
                color: Kirigami.Theme.highlightColor
                opacity: 0.8
            }

            QQC2.Label {
                anchors.left: parent.left
                anchors.verticalCenter: parent.verticalCenter
                text: one.index === 0 ? App.tr("Nearest drop: %1 px", Math.round(one.modelData.long)) : App.tr("Farthest drop: %1 px", Math.round(one.modelData.long))
            }
        }
    }
}
