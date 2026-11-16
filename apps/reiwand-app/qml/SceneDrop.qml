import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Item {
    id: drop

    required property var area
    readonly property var layout: area.layout
    property string carried: ""
    property point spot
    readonly property string from: layout.placeOf(carried)
    readonly property string aim: carried ? at(spot) : ""

    function zone(place) {
        const left = layout.leftWide || Look.lateralIzq;
        const right = layout.rightWide || Look.lateralIzq;
        if (place === "left")
            return Qt.rect(0, 0, left, height);
        if (place === "right")
            return Qt.rect(width - right, 0, right, height);
        const high = area.low || layout.footFloor * 2;
        return Qt.rect(left, height - high, width - left - right, high);
    }

    function at(point) {
        return ["left", "right", "bottom"].find(place => {
            const box = zone(place);
            return place !== from && point.x >= box.x && point.x < box.x + box.width && point.y >= box.y && point.y < box.y + box.height;
        }) ?? "";
    }

    function land() {
        if (aim)
            Qt.callLater(layout.move, carried, aim);
        carried = "";
    }

    function name(place) {
        const held = layout.places[place];
        if (held.length)
            return App.tr("With %1", area.dock(place).title(layout.current(place)));
        return place === "left" ? App.tr("Left side") : place === "right" ? App.tr("Right side") : App.tr("Bottom");
    }

    visible: carried !== ""

    Repeater {
        model: ["left", "right", "bottom"]

        delegate: Rectangle {
            id: place

            required property string modelData
            readonly property rect box: drop.zone(modelData)
            readonly property bool lit: drop.aim === modelData

            x: box.x + Look.e2
            y: box.y + Look.e2
            width: box.width - Look.e2 * 2
            height: box.height - Look.e2 * 2
            visible: modelData !== drop.from
            radius: Look.radio
            color: Qt.rgba(Look.luna.r, Look.luna.g, Look.luna.b, lit ? Look.veloTapaSobre : Look.veloTapaElegida)
            border.color: Look.luna
            border.width: lit ? Look.anillo : Look.linea

            Rectangle {
                anchors.centerIn: parent
                width: said.width + Look.e4
                height: said.implicitHeight + Look.e2
                radius: Look.radioChico
                color: Look.nocheAlta
                border.color: Look.luna
                border.width: Look.linea

                QQC2.Label {
                    id: said

                    anchors.centerIn: parent
                    width: Math.min(implicitWidth, place.width - Look.e4)
                    horizontalAlignment: Text.AlignHCenter
                    wrapMode: Text.Wrap
                    text: drop.name(place.modelData)
                    color: Look.luna
                    font.pixelSize: Look.tCuerpo
                }
            }
        }
    }

    Rectangle {
        x: drop.spot.x + Look.e3
        y: drop.spot.y + Look.e3
        width: chip.implicitWidth + Look.e4
        height: chip.implicitHeight + Look.e2
        radius: Look.radioChico
        color: Look.nocheAlta
        border.color: Look.luna
        border.width: Look.linea

        QQC2.Label {
            id: chip

            anchors.centerIn: parent
            text: drop.carried ? drop.area.dock(drop.from).title(drop.carried) : ""
            font.pixelSize: Look.tCuerpo
        }
    }
}
