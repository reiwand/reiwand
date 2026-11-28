import QtQuick
import io.github.reiwand.app

Row {
    id: strip

    property string current
    readonly property var screens: App.state.outputs ?? {}

    signal picked(string name)

    spacing: Look.e4
    Accessible.role: Accessible.RadioButton
    Accessible.name: App.tr("Which screen")

    Repeater {
        model: App.outputs

        delegate: Column {
            id: screen

            required property string modelData
            readonly property var info: strip.screens[modelData] ?? {}
            readonly property string item: info.item ?? ""
            readonly property bool chosen: modelData === strip.current
            readonly property real wide: info.width / Math.max(1, info.height)
            property string thumb

            function load() {
                const uid = item;
                thumb = "";
                if (uid !== "")
                    App.query("Thumbnail", [uid, 256], "Library1", reply => screen && screen.item === uid && (screen.thumb = reply[0] ?? ""));
            }

            anchors.bottom: parent ? parent.bottom : undefined
            spacing: Look.e2
            activeFocusOnTab: true
            Accessible.role: Accessible.RadioButton
            Accessible.name: modelData
            Accessible.checked: chosen
            onItemChanged: load()
            Component.onCompleted: load()
            Keys.onReturnPressed: strip.picked(modelData)
            Keys.onSpacePressed: strip.picked(modelData)

            Rectangle {
                id: frame

                anchors.horizontalCenter: parent.horizontalCenter
                width: screen.wide >= 16 / 9 ? Look.screenTileWide : Look.screenTile
                height: Math.round(width / Math.max(1, screen.wide))
                radius: Look.radiusSmall
                color: Look.nightHigh
                border.width: Look.ring
                border.color: Look.nightLine

                Photo {
                    anchors.fill: parent
                    anchors.margins: Look.ring
                    source: screen.thumb !== "" ? App.url(screen.thumb) : ""
                }

                Rectangle {
                    anchors.fill: parent
                    anchors.margins: Look.ring
                    color: Look.night
                    opacity: screen.chosen ? Look.coverVeilChosen : Look.coverVeil
                }

                FocusHalo {
                    visible: screen.activeFocus
                    corner: Look.radiusSmall
                }

                TapHandler {
                    onTapped: strip.picked(screen.modelData)
                }
            }

            Text {
                anchors.horizontalCenter: parent.horizontalCenter
                text: App.screen(screen.modelData)
                font.family: Look.textFont
                font.pixelSize: Look.tFoot
                font.weight: screen.chosen ? Font.DemiBold : Font.Normal
                color: screen.chosen ? Look.frost : Look.mist
            }

            Rectangle {
                anchors.horizontalCenter: parent.horizontalCenter
                visible: screen.chosen
                width: Look.e4
                height: Look.screenMark
                radius: Look.radiusSmall / 2
                color: Look.moon
            }
        }
    }
}
