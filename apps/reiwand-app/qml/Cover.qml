import QtQuick
import io.github.reiwand.app

Item {
    id: cover

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
    readonly property alias ring: ring
    default property alias below: under.data

    signal tapped(int modifiers)
    signal doubleTapped
    signal askedMenu

    readonly property bool grown: lit && !even
    readonly property real room: label.height + Look.noteTop + caption.height
    property string liveUid
    property bool live

    implicitWidth: grown ? Look.coverFocus : Look.cover
    implicitHeight: square.height + (grown ? under.height + Look.e4 : label.height + Look.e3 + (note ? caption.height + Look.noteTop : 0))
    Accessible.role: Accessible.ListItem
    Accessible.name: name
    Accessible.description: note
    Accessible.selected: lit
    Accessible.checkable: choosing
    Accessible.checked: chosen

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

        Shadow {
            kind: cover.lit ? "focus" : "cover"
        }

        CoverMosaic {
            anchors.fill: parent
            pictures: cover.pictures
            off: cover.off
        }

        Photo {
            readonly property bool tiny: status === Image.Ready && (implicitWidth * 5 < width * 3 || implicitHeight * 5 < height * 3)

            anchors.fill: parent
            visible: cover.pictures.length < 2
            source: cover.pictures.length === 1 ? cover.pictures[0] : cover.picture
            sourceSize.width: Look.coverFocus
            sourceSize.height: Math.round(Look.coverFocus * 9 / 16)
            fillMode: tiny ? Image.Pad : Image.PreserveAspectCrop
            clip: tiny
            opacity: cover.off ? Look.coverDimmed : 1
        }

        TileGlyph {
            anchors.fill: parent
            visible: cover.picture == "" && cover.pictures.length === 0 && cover.icon !== ""
            icon: cover.icon
        }

        AnimatedImage {
            anchors.fill: parent
            visible: cover.moving.toString() !== "" && status === AnimatedImage.Ready
            source: cover.moving
            playing: visible && cover.Window.visibility !== Window.Hidden
            cache: false
            fillMode: Image.PreserveAspectCrop
            opacity: cover.off ? Look.coverDimmed : 1
        }

        LiveCover {
            anchors.fill: parent
            uid: cover.liveUid
            wanted: cover.live && cover.hovered && cover.visible && cover.Window.active
        }

        Rectangle {
            anchors.fill: parent
            radius: Look.radius
            visible: !cover.lit
            color: Look.night
            opacity: cover.chosen ? Look.coverVeilChosen : cover.hovered ? Look.coverVeilHover : Look.coverVeil
        }

        ThinProgress {
            id: bar

            x: Look.radius
            y: parent.height - height - Look.line * 2
            width: parent.width - Look.radius * 2
            value: cover.progress
        }

        FocusHalo {
            id: ring

            visible: cover.lit && Keyboard.used
            gap: Look.inset
        }

        InUse {
            id: mark

            visible: cover.inUse || cover.listed !== ""
            screen: cover.usedOn
            list: cover.inUse ? "" : cover.listed
            pointed: cover.hovered && contains(mapFromItem(area, area.mouseX, area.mouseY))
            x: Look.coverMark
            y: parent.height - height - Look.coverMark
        }

        ChooseMark {
            id: circle

            visible: cover.choosing || cover.choosable && cover.hovered
            x: Look.coverMark
            y: Look.coverMark
            chosen: cover.chosen
        }
    }

    Text {
        id: label

        visible: !cover.grown
        y: square.height + Look.e3
        width: parent.width
        text: cover.name
        elide: Text.ElideRight
        font.family: Look.titleFont
        font.pixelSize: Look.tName
        font.weight: cover.lit ? Font.Bold : Font.Medium
        color: cover.lit ? Look.frost : Look.mist
    }

    Note {
        id: caption

        visible: !cover.grown && cover.note !== ""
        y: label.y + label.height + Look.noteTop
        fitWidth: parent.width
        text: cover.note
        tone: cover.noteTone
        icon: cover.noteIcon
    }

    Item {
        id: under

        visible: cover.grown
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
        onClicked: mouse => mouse.button === Qt.RightButton ? cover.askedMenu() : cover.tapped(circle.visible && circle.contains(circle.mapFromItem(area, mouse.x, mouse.y)) ? mouse.modifiers | Qt.ControlModifier : mouse.modifiers)
        onDoubleClicked: mouse => mouse.button === Qt.LeftButton && cover.doubleTapped()
    }
}
