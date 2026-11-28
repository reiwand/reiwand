import QtQuick
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

Item {
    id: card

    required property string uid
    required property string json
    required property int offset
    required property bool gone
    readonly property var item: JSON.parse(json)
    property real aspect: 16 / 9
    property real unit: 1
    property bool calm: false
    property bool current: false
    property string animated: ""
    property string playUid: ""
    property string found: ""
    property bool shown: false
    readonly property bool middle: offset === 0
    readonly property int reach: Math.min(Math.abs(offset), 3)
    readonly property int sign: offset < 0 ? -1 : 1
    readonly property real span: Math.round(unit * (reach === 0 ? PreviewLook.focusWidth : reach === 1 ? PreviewLook.nearWidth : PreviewLook.farWidth))
    readonly property real centre: reach === 0 ? 0 : sign * Math.round(unit * (PreviewLook.focusWidth / 2 + PreviewLook.gap + (reach === 1 ? PreviewLook.nearWidth / 2 : PreviewLook.nearWidth + PreviewLook.gap + PreviewLook.farWidth / 2 + (reach === 3 ? PreviewLook.farWidth / 2 : 0))))
    readonly property int edge: PreviewLook.edge(Math.min(span, PreviewLook.focusWidth * unit) * Screen.devicePixelRatio)
    readonly property bool hovered: grip.containsMouse
    readonly property bool staying: animated === "" && playUid === "" || calm
    property real home: 0
    signal clicked(int offset)

    function fetch() {
        const wanted = uid;
        App.query("Thumbnail", [wanted, edge], "Library1", reply => card && card.uid === wanted && (card.found = reply[0] ?? ""), () => {});
    }

    x: home + centre - width / 2
    y: Math.round((parent.height - height) / 2)
    width: span
    height: Math.round(span / aspect)
    opacity: !shown || gone || reach === 3 ? 0 : middle ? 1 : hovered ? 1 : reach === 1 ? PreviewLook.nearOpacity : PreviewLook.farOpacity
    scale: grip.pressed ? 0.98 : 1
    z: middle ? 2 : 1
    Accessible.role: Accessible.Button
    Accessible.name: item.title ?? ""
    Accessible.onPressAction: card.clicked(offset)
    onEdgeChanged: fetch()
    Component.onCompleted: {
        fetch();
        shown = true;
    }

    Behavior on x {
        enabled: card.shown
        NumberAnimation {
            duration: card.calm ? 0 : PreviewLook.pace
            easing.type: Easing.OutCubic
        }
    }

    Behavior on width {
        enabled: card.shown
        NumberAnimation {
            duration: card.calm ? 0 : PreviewLook.pace
            easing.type: Easing.OutCubic
        }
    }

    Behavior on height {
        enabled: card.shown
        NumberAnimation {
            duration: card.calm ? 0 : PreviewLook.pace
            easing.type: Easing.OutCubic
        }
    }

    Behavior on opacity {
        NumberAnimation {
            duration: card.calm ? 0 : PreviewLook.pace
            easing.type: Easing.OutCubic
        }
    }

    Behavior on scale {
        NumberAnimation {
            duration: card.calm ? 0 : 90
            easing.type: Easing.OutCubic
        }
    }

    Rectangle {
        anchors.fill: parent
        radius: PreviewLook.cardRadius
        color: Kirigami.Theme.alternateBackgroundColor
    }

    Photo {
        id: picture

        anchors.fill: parent
        corner: PreviewLook.cardRadius
        source: App.url(card.found !== "" ? card.found : String(card.item.thumb ?? ""))
        sourceSize.width: Math.round(card.width * Screen.devicePixelRatio)
        mipmap: true
        smooth: true
        opacity: status === Image.Ready ? 1 : 0

        Behavior on opacity {
            NumberAnimation {
                duration: card.calm ? 0 : PreviewLook.appear
            }
        }
    }

    AnimatedImage {
        anchors.fill: parent
        visible: card.middle && !card.staying && status === AnimatedImage.Ready
        source: card.middle && !card.staying ? card.animated : ""
        playing: visible && card.Window.visibility !== Window.Hidden
        fillMode: Image.PreserveAspectCrop
        cache: false
    }

    LiveCover {
        anchors.fill: parent
        uid: card.playUid
        wanted: card.middle && !card.calm && card.playUid !== "" && card.shown && card.Window.active
        corner: PreviewLook.cardRadius
    }

    Kirigami.Icon {
        anchors.centerIn: parent
        width: Kirigami.Units.iconSizes.large
        height: width
        visible: picture.status !== Image.Ready && (card.item.missing === true || !!card.item.error)
        source: "image-x-generic"
        color: Kirigami.Theme.textColor
    }

    Rectangle {
        anchors.fill: parent
        anchors.margins: -(PreviewLook.ringGap + PreviewLook.ring)
        radius: PreviewLook.cardRadius + PreviewLook.ringGap + PreviewLook.ring
        color: "transparent"
        border.width: card.middle ? PreviewLook.ring : card.hovered ? 2 : 0
        border.color: card.middle ? Kirigami.Theme.highlightColor : Qt.alpha(Kirigami.Theme.textColor, 0.55)
        visible: border.width > 0
    }

    PreviewBadge {
        x: Math.round(8 * card.unit)
        y: x
        visible: card.current && card.reach < 3
        text: App.tr("In use")
        icon: "dialog-ok-apply"
    }

    MouseArea {
        id: grip

        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: card.clicked(card.offset)
    }
}
