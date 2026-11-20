import QtQuick
import io.github.reiwand.app

Item {
    id: tapa

    property url picture
    property var pictures: []
    property url moving
    property string name
    property string icon
    property string note
    property string noteIcon
    property string noteTone
    property bool lit
    property bool off
    property bool inUse
    property string usedOn
    property string listed
    property bool choosing
    property bool choosable
    property bool chosen
    property bool even
    property real progress: -1
    readonly property alias hovered: area.containsMouse
    readonly property alias bar: bar
    readonly property alias mark: mark
    default property alias below: under.data

    signal tapped(int modifiers)
    signal doubleTapped
    signal askedMenu

    readonly property bool grown: lit && !even
    readonly property real room: label.height + Look.notaArriba + caption.height
    property string liveUid
    property bool live

    implicitWidth: grown ? Look.tapaFoco : Look.tapa
    implicitHeight: square.height + (grown ? under.height + Look.e4 : label.height + Look.e3 + (note ? caption.height + Look.notaArriba : 0))
    Accessible.role: Accessible.ListItem
    Accessible.name: name
    Accessible.description: note
    Accessible.selected: lit
    Accessible.checkable: choosing
    Accessible.checked: chosen

    Behavior on implicitWidth  {
        NumberAnimation {
            duration: Look.paso
            easing.type: Easing.BezierSpline
            easing.bezierCurve: Look.pasoCurva
        }
    }

    Rectangle {
        id: square

        width: parent.width
        height: Math.round(width * 9 / 16)
        radius: Look.radio
        color: Look.nocheAlta

        Sombra {
            kind: tapa.lit ? "foco" : "tapa"
        }

        TapaMosaico {
            anchors.fill: parent
            pictures: tapa.pictures
            off: tapa.off
        }

        Foto {
            readonly property bool tiny: status === Image.Ready && (implicitWidth * 5 < width * 3 || implicitHeight * 5 < height * 3)

            anchors.fill: parent
            visible: tapa.pictures.length < 2
            source: tapa.pictures.length === 1 ? tapa.pictures[0] : tapa.picture
            sourceSize.width: Look.tapaFoco
            sourceSize.height: Math.round(Look.tapaFoco * 9 / 16)
            fillMode: tiny ? Image.Pad : Image.PreserveAspectCrop
            clip: tiny
            opacity: tapa.off ? Look.tapaApagada : 1
        }

        PuntoDibujo {
            anchors.fill: parent
            visible: tapa.picture == "" && tapa.pictures.length === 0 && tapa.icon !== ""
            icon: tapa.icon
        }

        AnimatedImage {
            anchors.fill: parent
            visible: tapa.moving.toString() !== "" && status === AnimatedImage.Ready
            source: tapa.moving
            playing: visible && tapa.Window.visibility !== Window.Hidden
            cache: false
            fillMode: Image.PreserveAspectCrop
            opacity: tapa.off ? Look.tapaApagada : 1
        }

        TapaViva {
            anchors.fill: parent
            uid: tapa.liveUid
            wanted: tapa.live && tapa.hovered && tapa.visible && tapa.Window.active
        }

        Rectangle {
            anchors.fill: parent
            radius: Look.radio
            visible: !tapa.lit
            color: Look.noche
            opacity: tapa.chosen ? Look.veloTapaElegida : tapa.hovered ? Look.veloTapaSobre : Look.veloTapa
        }

        Barra {
            id: bar

            x: Look.radio
            y: parent.height - height - Look.linea * 2
            width: parent.width - Look.radio * 2
            value: tapa.progress
        }

        Anillo {
            visible: tapa.lit
            gap: Look.filete
        }

        Uso {
            id: mark

            visible: tapa.inUse || tapa.listed !== ""
            screen: tapa.usedOn
            list: tapa.inUse ? "" : tapa.listed
            pointed: tapa.hovered && contains(Qt.point(area.mouseX - x, area.mouseY - y))
            x: Look.marcaTapa
            y: parent.height - height - Look.marcaTapa
        }

        Elegir {
            id: circle

            visible: tapa.choosing || tapa.choosable && tapa.hovered
            x: Look.marcaTapa
            y: Look.marcaTapa
            chosen: tapa.chosen
        }
    }

    Text {
        id: label

        visible: !tapa.grown
        y: square.height + Look.e3
        width: parent.width
        text: tapa.name
        elide: Text.ElideRight
        font.family: Look.letraTitulo
        font.pixelSize: Look.tNombre
        font.weight: tapa.lit ? Font.Bold : Font.Medium
        color: tapa.lit ? Look.escarcha : Look.bruma
    }

    Nota {
        id: caption

        visible: !tapa.grown && tapa.note !== ""
        y: label.y + label.height + Look.notaArriba
        fitWidth: parent.width
        text: tapa.note
        tone: tapa.noteTone
        icon: tapa.noteIcon
    }

    Item {
        id: under

        visible: tapa.grown
        y: square.height + Look.e4
        width: childrenRect.width
        height: childrenRect.height
    }

    MouseArea {
        id: area

        anchors.fill: square
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        acceptedButtons: Qt.LeftButton | Qt.RightButton
        onClicked: mouse => mouse.button === Qt.RightButton ? tapa.askedMenu() : tapa.tapped(circle.visible && circle.contains(circle.mapFromItem(area, mouse.x, mouse.y)) ? mouse.modifiers | Qt.ControlModifier : mouse.modifiers)
        onDoubleClicked: mouse => mouse.button === Qt.LeftButton && tapa.doubleTapped()
    }
}
