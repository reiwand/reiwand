import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Rectangle {
    id: dock

    required property var area
    required property var page
    required property var layout
    required property string place
    readonly property var units: layout.places[place]
    readonly property string current: layout.current(place)
    readonly property alias body: body
    readonly property real strip: tabs.height + Look.e2 + Look.linea
    readonly property real need: Array.from(tabs.children).reduce((sum, item) => sum + item.implicitWidth, Look.e2 * 2)

    function title(unit) {
        const count = (page.view.problems ?? []).length;
        return {
            "layers": App.tr("Layers"),
            "assets": App.tr("Assets"),
            "options": App.tr("Options"),
            "try": App.tr("Try"),
            "properties": App.tr("Properties"),
            "problems": count ? App.tr("Problems (%1)", count) : App.tr("Problems"),
            "timeline": App.tr("Timeline")
        }[unit] ?? unit;
    }

    visible: units.length > 0
    color: place === "bottom" ? Look.noche : Look.nocheAlta
    clip: true

    Rectangle {
        y: dock.strip - Look.linea
        width: parent.width
        height: Look.linea
        color: Look.nocheLinea
    }

    Flow {
        id: tabs

        x: Look.e2
        y: Look.e2
        width: parent.width - Look.e2 * 2
        spacing: 0

        Repeater {
            model: dock.units

            delegate: QQC2.TabButton {
                id: tab

                required property string modelData
                readonly property bool lit: dock.current === modelData

                text: dock.title(modelData)
                leftPadding: Look.e2
                rightPadding: Look.e2
                font.pixelSize: Look.tCuerpo
                font.weight: lit ? Font.DemiBold : Font.Normal
                checked: lit
                Accessible.role: Accessible.PageTab
                Accessible.description: App.tr("Drag the tab, or press the menu key, to move this panel to another side")
                onClicked: dock.layout.reveal(modelData)
                Keys.onPressed: event => {
                    if (event.key === Qt.Key_Menu || (event.key === Qt.Key_F10 && event.modifiers & Qt.ShiftModifier)) {
                        menu.popup(tab, 0, tab.height);
                        event.accepted = true;
                    }
                }

                DragHandler {
                    target: null
                    cursorShape: active ? Qt.ClosedHandCursor : Qt.ArrowCursor
                    onActiveChanged: active ? dock.area.drop.carried = tab.modelData : dock.area.drop.land()
                    onCentroidChanged: {
                        if (active)
                            dock.area.drop.spot = tab.mapToItem(dock.area, centroid.position.x, centroid.position.y);
                    }
                }

                TapHandler {
                    acceptedButtons: Qt.RightButton
                    onTapped: menu.popup()
                }

                QQC2.Menu {
                    id: menu

                    QQC2.MenuItem {
                        text: App.tr("Move to the left")
                        enabled: dock.place !== "left"
                        onTriggered: Qt.callLater(dock.layout.move, tab.modelData, "left")
                    }

                    QQC2.MenuItem {
                        text: App.tr("Move to the right")
                        enabled: dock.place !== "right"
                        onTriggered: Qt.callLater(dock.layout.move, tab.modelData, "right")
                    }

                    QQC2.MenuItem {
                        text: App.tr("Move to the bottom")
                        enabled: dock.place !== "bottom"
                        onTriggered: Qt.callLater(dock.layout.move, tab.modelData, "bottom")
                    }
                }

                Rectangle {
                    anchors.bottom: parent.bottom
                    width: parent.width
                    height: Look.linea * 2
                    visible: tab.lit
                    color: Look.luna
                }
            }
        }
    }

    Item {
        id: body

        x: Look.anilloLugar
        y: dock.strip + Look.anilloLugar
        width: parent.width - 2 * Look.anilloLugar
        height: parent.height - y - Look.anilloLugar
    }
}
