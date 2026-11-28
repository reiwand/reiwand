import QtQuick
import QtQuick.Controls.impl
import io.github.reiwand.app

Item {
    id: tile

    property url picture
    property string icon
    property string name
    property string note
    property string noteIcon
    property string noteTone
    property bool lit
    property bool chosen
    property bool off
    property bool grows
    readonly property alias hovered: area.containsMouse
    readonly property bool big: grows && lit
    readonly property bool marked: lit || chosen
    readonly property alias square: square

    signal tapped(int modifiers)
    signal doubleTapped
    signal askedMenu

    implicitWidth: big ? Look.coverCreateFocus : Look.coverCreate
    implicitHeight: square.height + Look.e2 + label.height + (note !== "" ? words.height + Look.noteTop : 0)
    Accessible.role: Accessible.ListItem
    Accessible.name: name
    Accessible.description: note
    Accessible.selected: lit

    Behavior on implicitWidth  {
        NumberAnimation {
            duration: Look.pace
            easing.type: Easing.BezierSpline
            easing.bezierCurve: Look.paceCurve
        }
    }

    Rectangle {
        id: square

        width: parent.width
        height: Math.round(width * 9 / 16)
        radius: Look.radius
        color: Look.nightHigh

        Image {
            anchors.fill: parent
            source: tile.picture
            asynchronous: true
            cache: true
            fillMode: Image.PreserveAspectCrop
            sourceSize.width: Look.coverFocus
            sourceSize.height: Math.round(Look.coverFocus * 9 / 16)
            opacity: tile.off ? Look.coverDimmed : 1
        }

        TileGlyph {
            anchors.fill: parent
            visible: tile.picture == "" && tile.icon !== ""
            icon: tile.icon
        }

        Rectangle {
            anchors.fill: parent
            radius: Look.radius
            visible: !tile.marked
            color: Look.night
            opacity: tile.hovered ? Look.coverVeilHover : Look.coverVeil
        }

        Rectangle {
            visible: tile.lit
            anchors.fill: parent
            anchors.margins: -(Look.inset + Look.ring)
            radius: Look.radius + Look.inset + Look.ring
            color: "transparent"
            border.width: Look.ring
            border.color: Look.frost
        }
    }

    Text {
        id: label

        y: square.height + Look.e2
        width: parent.width
        text: tile.name
        elide: Text.ElideRight
        font.family: Look.titleFont
        font.pixelSize: Look.tName
        font.weight: tile.marked ? Font.Bold : Font.Medium
        color: tile.marked ? Look.frost : tile.off ? Look.mistLow : Look.mist
    }

    Row {
        id: words

        visible: tile.note !== ""
        y: label.y + label.height + Look.noteTop
        width: parent.width
        spacing: Look.inUseGap

        IconImage {
            visible: tile.noteIcon !== ""
            y: 1
            width: Look.icon
            height: Look.icon
            sourceSize: Qt.size(Look.icon, Look.icon)
            name: tile.noteIcon
            color: Tone.of(tile.noteTone, false, Look.mist)
        }

        Text {
            width: parent.width - (tile.noteIcon !== "" ? Look.icon + parent.spacing : 0)
            text: tile.note
            wrapMode: Text.WordWrap
            lineHeight: 1.4
            font.family: Look.textFont
            font.pixelSize: Look.tFoot
            color: tile.marked ? Look.frost : tile.noteTone !== "" ? Tone.of(tile.noteTone, false, Look.mist) : Look.mist
        }
    }

    MouseArea {
        id: area

        anchors.fill: square
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        acceptedButtons: Qt.LeftButton | Qt.RightButton
        onClicked: mouse => mouse.button === Qt.RightButton ? tile.askedMenu() : tile.tapped(mouse.modifiers)
        onDoubleClicked: mouse => mouse.button === Qt.LeftButton && tile.doubleTapped()
    }
}
