import QtQuick
import io.github.reiwand.app

FocusScope {
    id: part

    required property var page
    readonly property var lines: String(App.licenses()).split("\n")

    implicitHeight: view.height

    ListView {
        id: view

        property string kind: "texto"
        property string explain
        property string text: App.tr("Licenses")

        width: Math.min(Look.filasAncho, parent.width)
        height: Math.max(240, part.page.height - Look.e6 * 3)
        clip: true
        model: part.lines
        activeFocusOnTab: true
        boundsBehavior: Flickable.StopAtBounds
        reuseItems: true
        Accessible.role: Accessible.StaticText
        Accessible.name: App.tr("Licenses")
        Keys.onUpPressed: event => contentY = Math.max(0, contentY - 48)
        Keys.onDownPressed: event => contentY = Math.min(Math.max(0, contentHeight - height), contentY + 48)
        Keys.onPressed: event => {
            const page = height - 48;
            if (event.key === Qt.Key_PageUp)
                contentY = Math.max(0, contentY - page);
            else if (event.key === Qt.Key_PageDown)
                contentY = Math.min(Math.max(0, contentHeight - height), contentY + page);
            else if (event.key === Qt.Key_Home)
                contentY = 0;
            else if (event.key === Qt.Key_End)
                contentY = Math.max(0, contentHeight - height);
            else
                return;
            event.accepted = true;
        }

        delegate: Text {
            required property string modelData

            width: view.width
            text: modelData === "" ? " " : modelData
            textFormat: Text.PlainText
            wrapMode: Text.WordWrap
            color: Look.bruma
            font.family: Look.letraTexto
            font.pixelSize: Look.tPie
        }

        Rectangle {
            anchors.fill: parent
            z: -1
            color: "transparent"
            border.width: view.activeFocus ? 2 : 0
            border.color: Look.escarcha
        }
    }
}
