import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Item {
    id: recent

    required property var page
    readonly property alias row: row
    readonly property bool any: page.projects.length > 0
    readonly property bool lit: page.projects.some(entry => entry.recent !== false)

    implicitHeight: any ? follow.height : invite.height

    Column {
        id: follow

        visible: recent.any
        anchors.right: parent.right
        width: Math.min(parent.width, Math.max(row.contentWidth, top.implicitWidth + row.pad))
        spacing: Look.e3

        Row {
            id: top

            x: row.pad
            spacing: Look.e4

            Text {
                id: heading

                anchors.verticalCenter: parent.verticalCenter
                text: App.tr("Continue with yours")
                font.family: Look.titleFont
                font.pixelSize: Look.tRow
                font.weight: Font.Bold
                font.letterSpacing: Look.tRow * Look.tight
                color: Look.frost
            }

            QQC2.Button {
                anchors.verticalCenter: parent.verticalCenter
                visible: recent.lit
                flat: true
                text: App.trc("recent projects", "Clear")
                QQC2.ToolTip.text: App.tr("Empty this row. The projects stay in “All my projects” and their files stay where they are.")
                QQC2.ToolTip.visible: hovered
                onClicked: recent.page.facts.clear()
            }
        }

        CreateRow {
            id: row

            wheels: !recent.page.shared
            width: parent.width
            height: implicitHeight
            grows: true
            label: App.tr("Continue with yours")
            model: recent.page.facts.recentModel
            active: recent.page.focused === row
            onTaken: recent.page.focused = row
            onActivated: recent.page.act(row)
            onAsked: (index, at) => recent.page.menu.ask(row, at)
        }
    }

    Column {
        id: invite

        visible: !recent.any
        anchors.right: parent.right
        width: Math.min(parent.width, Look.toastWidth + Look.e7 * 2)
        spacing: Look.e3

        Text {
            width: parent.width
            horizontalAlignment: Text.AlignRight
            wrapMode: Text.WordWrap
            text: App.tr("Your first wallpaper starts here")
            font.family: Look.titleFont
            font.pixelSize: Look.tLarge
            font.weight: Font.Bold
            font.letterSpacing: Look.tLarge * Look.tight
            color: Look.frost
        }

        Text {
            width: parent.width
            horizontalAlignment: Text.AlignRight
            wrapMode: Text.WordWrap
            text: App.tr("The easiest way is to edit a copy of an example scene and change it to your liking. To start from scratch, choose “Start a new one”.")
            font.family: Look.textFont
            font.pixelSize: Look.tExplain
            color: Look.mist
        }
    }
}
