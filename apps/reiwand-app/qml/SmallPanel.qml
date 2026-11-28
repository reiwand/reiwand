import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

QQC2.Popup {
    id: panel

    property Item opener
    property Item before: null
    property url cover
    property string title
    property string text
    property string quiet
    property string primary
    property string primaryIcon
    property bool destructive: false
    property bool padded: true
    readonly property real inset: padded ? 0 : Look.listGap + Look.e3
    property bool ready: true
    property Item first: primary !== "" ? main : quiet !== "" ? calm : null
    default property alias content: inside.data
    property alias foot: extra.data
    readonly property real footWidth: width - 2 * Look.e4 - (main.visible ? main.implicitWidth + Look.e2 : 0) - (calm.visible ? calm.implicitWidth + Look.e2 : 0)

    signal accepted
    signal rejected

    function pressEnter() {
        if (body.Window.activeFocusItem === calm) {
            rejected();
            close();
        } else if (main.visible && !destructive && ready) {
            accepted();
            close();
        }
    }

    function place() {
        if (!parent)
            return;
        if (!opener) {
            x = (parent.width - width) / 2;
            y = Math.max(Look.e4, (parent.height - implicitHeight) / 2);
            return;
        }
        const below = opener.mapToItem(parent, 0, opener.height + Look.e2);
        const above = opener.mapToItem(parent, 0, -Look.e2).y - implicitHeight;
        x = below.x;
        y = below.y + implicitHeight + Look.e4 > parent.height && above >= Look.e4 ? above : below.y;
    }

    parent: QQC2.Overlay.overlay
    width: Look.smallPanel
    padding: 0
    margins: Look.e4
    focus: true
    closePolicy: QQC2.Popup.CloseOnEscape | QQC2.Popup.CloseOnPressOutside
    onAboutToShow: {
        // Without an opener, the focus goes back to what had it before the panel opened.
        before = opener ? null : (parent ?? body).Window.activeFocusItem;
        place();
    }
    onOpened: (destructive ? (calm.visible ? calm : body) : (first ?? body)).forceActiveFocus(Qt.TabFocusReason)
    onClosed: {
        const back = opener ?? before;
        before = null;
        Qt.callLater(() => {
            if (!back || typeof back.forceActiveFocus !== "function")
                return;
            back.focus = false;
            back.forceActiveFocus(Qt.TabFocusReason);
        });
    }

    background: Rectangle {
        radius: Look.panelRadius
        color: Look.night
        border.width: Look.line
        border.color: Look.nightLine

        Shadow {
            corner: Look.panelRadius
            kind: "panel"
        }
    }

    contentItem: FocusScope {
        id: body

        implicitWidth: panel.width
        implicitHeight: column.implicitHeight
        Accessible.role: Accessible.Dialog
        Accessible.name: panel.title
        Keys.onReturnPressed: panel.pressEnter()
        Keys.onEnterPressed: panel.pressEnter()

        Column {
            id: column

            width: parent.width

            Photo {
                visible: panel.cover.toString() !== ""
                width: parent.width
                height: width * 7 / 16
                source: panel.cover
                corner: Look.panelRadius
                lowCorner: 0
                sourceSize.width: width

                Rectangle {
                    anchors.fill: parent
                    gradient: Gradient {
                        GradientStop {
                            position: 0.4
                            color: Qt.rgba(Look.night.r, Look.night.g, Look.night.b, 0)
                        }
                        GradientStop {
                            position: 1
                            color: Look.night
                        }
                    }
                }
            }

            Column {
                id: inside

                width: parent.width
                padding: panel.padded ? Look.e4 : 0
                spacing: Look.e3

                PanelWords {
                    width: parent.width - 2 * parent.padding
                    owner: panel
                }
            }

            Rectangle {
                visible: panel.quiet !== "" || panel.primary !== "" || extra.children.length > 0
                width: parent.width
                height: Look.line
                color: Look.nightLine
            }

            RowLayout {
                visible: panel.quiet !== "" || panel.primary !== "" || extra.children.length > 0
                width: parent.width - 2 * Look.e4
                x: Look.e4
                height: implicitHeight + 2 * Look.e3
                spacing: Look.e2

                Row {
                    id: extra

                    Layout.fillWidth: true
                    spacing: Look.e2
                }

                QQC2.Button {
                    id: calm

                    visible: panel.quiet !== ""
                    text: panel.quiet
                    onClicked: {
                        panel.rejected();
                        panel.close();
                    }
                }

                QQC2.Button {
                    id: main

                    visible: panel.primary !== ""
                    enabled: panel.ready
                    highlighted: true
                    palette.highlight: panel.destructive ? Look.coral : Look.moon
                    text: panel.primary
                    icon.name: panel.primaryIcon
                    onClicked: {
                        panel.accepted();
                        panel.close();
                    }
                }
            }
        }
    }
}
