import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

// A tab of a workbench group: the panel's icon and name, dragged to another place, with its menu
// on the menu key, Shift+F10 or a right click, and the accent mark when its group has the keyboard.
QQC2.TabButton {
    id: tab

    required property var home

    required property string modelData

    visible: !home.hidden.includes(modelData)
    onImplicitWidthChanged: Qt.callLater(home.measure)
    readonly property bool lit: home.current === modelData
    readonly property alias menu: menu

    text: home.area.title(modelData)
    implicitHeight: DeskLook.strip
    leftPadding: DeskLook.tabSide
    rightPadding: DeskLook.tabSide
    icon.name: BenchWords.icon(modelData)
    font.family: Look.titleFont
    font.pixelSize: DeskLook.tTab
    font.weight: Font.DemiBold
    checked: lit
    Accessible.role: Accessible.PageTab
    Accessible.description: home.bare ? App.tr("Press the menu key to move the canvas or detach it into a window") : App.tr("Drag the tab onto another group, or press the menu key, to move this panel")
    onClicked: {
        home.layout.reveal(modelData);
        Qt.callLater(() => home.area.keys?.enter(home, modelData));
    }
    function menuKey(event) {
        return event.key === Qt.Key_Menu || (event.key === Qt.Key_F10 && (event.modifiers & Qt.ShiftModifier) !== 0);
    }

    // The tab claims its menu keys before the window-wide ones of a panel (Layers) can take them.
    Keys.onShortcutOverride: event => event.accepted = menuKey(event)
    Keys.onPressed: event => {
        if (menuKey(event)) {
            menu.popup(tab, 0, tab.height);
            event.accepted = true;
        }
    }

    contentItem: Row {
        spacing: DeskLook.gap

        IconImage {
            anchors.verticalCenter: parent.verticalCenter
            name: tab.icon.name
            sourceSize: Qt.size(DeskLook.tabIcon, DeskLook.tabIcon)
            color: tab.lit || tab.hovered ? Look.frost : Look.mist
        }

        Text {
            anchors.verticalCenter: parent.verticalCenter
            text: tab.text
            font: tab.font
            color: tab.lit || tab.hovered ? Look.frost : Look.mist
        }
    }

    background: Rectangle {
        color: tab.lit ? Look.night : "transparent"

        Rectangle {
            x: parent.width - width
            width: Look.line
            height: parent.height
            color: Look.nightLine
        }

        Ring {
            anchors.margins: Look.ring
            visible: tab.visualFocus
        }
    }

    DragHandler {
        target: null
        enabled: !home.bare
        cursorShape: active ? Qt.ClosedHandCursor : Qt.ArrowCursor
        onActiveChanged: active ? home.area.drop.carried = tab.modelData : home.area.drop.land()
        onCentroidChanged: {
            if (active)
                home.area.drop.spot = tab.mapToItem(home.area, centroid.position.x, centroid.position.y);
        }
    }

    TapHandler {
        acceptedButtons: Qt.RightButton
        onTapped: menu.popup()
    }

    SceneTabMenu {
        id: menu

        area: home.area
        unit: tab.modelData
    }

    Rectangle {
        x: tab.mirrored ? tab.width - width - tab.rightPadding : tab.leftPadding
        y: tab.height - height - DeskLook.markBottom
        width: Look.markWidth
        height: DeskLook.markHeight
        radius: Look.line
        visible: tab.lit
        color: home.focused ? Look.moon : Look.mistLow
    }
}
