import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Item {
    id: line

    readonly property alias button: off

    visible: App.cut
    implicitHeight: visible ? row.implicitHeight + 2 * Look.e3 : 0

    RowLayout {
        id: row

        anchors.fill: parent
        anchors.leftMargin: Look.e4
        anchors.rightMargin: Look.e4
        spacing: Look.e3

        QQC2.Label {
            Layout.fillWidth: true
            text: App.tr("Private mode: the network is cut.")
            color: Look.mist
            wrapMode: Text.Wrap
        }

        QQC2.Button {
            id: off

            text: App.tr("Turn off")
            activeFocusOnTab: true
            onClicked: App.act("SetOption", ["network.private", "false"], "Daemon1")
            Keys.onReturnPressed: clicked()
            Keys.onEnterPressed: clicked()
        }
    }
}
