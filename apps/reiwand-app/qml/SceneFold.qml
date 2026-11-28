import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import QtCore as Core
import io.github.reiwand.app

ColumnLayout {
    id: fold

    required property string key
    property string title
    property bool open: true
    property bool reached: false
    property var saved: null
    default property alias content: inside.data
    readonly property Component keeper: Component {
        Core.Settings {
            location: Core.StandardPaths.writableLocation(Core.StandardPaths.ConfigLocation) + "/reiwand/editor.conf"
            category: "folds"
        }
    }

    onOpenChanged: reached = reached || open
    Component.onCompleted: {
        reached = open;
        if (App.checking)
            return;
        saved = keeper.createObject(fold);
        open = saved.value(key, open ? "open" : "closed") !== "closed";
    }

    spacing: 0

    // The header of a section: a caret and the title, which open or fold it with a click, Space or
    // Enter. It is a plain header, not a lit button, open or folded.
    QQC2.ToolButton {
        id: head

        Layout.fillWidth: true
        text: fold.title
        icon.name: fold.open ? "caret-down" : "caret-right"
        Accessible.role: Accessible.Button
        Accessible.name: fold.title
        Accessible.checkable: true
        Accessible.checked: fold.open
        onClicked: {
            fold.open = !fold.open;
            if (fold.saved)
                fold.saved.setValue(fold.key, fold.open ? "open" : "closed");
        }

        contentItem: Row {
            spacing: Look.e2

            IconImage {
                anchors.verticalCenter: parent.verticalCenter
                name: head.icon.name
                sourceSize: Qt.size(DeskLook.tabIcon, DeskLook.tabIcon)
                color: Look.mist
            }

            Text {
                anchors.verticalCenter: parent.verticalCenter
                width: head.availableWidth - DeskLook.tabIcon - Look.e2
                text: head.text
                elide: Text.ElideRight
                font.family: Look.textFont
                font.pixelSize: DeskLook.tRow
                font.weight: Font.DemiBold
                color: Look.frost
            }
        }
    }

    ColumnLayout {
        id: inside

        Layout.fillWidth: true
        visible: fold.open
    }
}
