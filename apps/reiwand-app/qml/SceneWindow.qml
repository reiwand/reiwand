import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// A panel or a whole group detached into a window of its own: the same panels, as tabs when there
// are several, with "Return to the workbench". Closing the window returns them too; its size is
// kept for the next time the same panels are detached, also after the app is closed. On Wayland
// the desktop chooses where it appears.
Window {
    id: window

    required property var area
    required property var held
    readonly property string unit: held.shown
    readonly property var units: held.units
    readonly property alias body: body
    readonly property var size: area.layout.view.sizes[units.join("+")] ?? [Look.sideLeft * 1.6, Look.sideLeft * 2]

    property bool leaving: false

    // The panels leave the window before it closes, so they are not destroyed with it.
    function back() {
        if (leaving)
            return;
        leaving = true;
        area.serial++;
        area.layout.view.attachAll(unit, [width, height]);
    }

    // A window that a screen change left off every screen (a screen removed or made smaller) comes
    // back, whole, on the screen of the editor.
    function keepOn() {
        const meets = one => Math.min(x + width, one.virtualX + one.width) - Math.max(x, one.virtualX) >= Look.e7 && Math.min(y + height, one.virtualY + one.height) - Math.max(y, one.virtualY) >= Look.e7;
        const screens = Array.from(Qt.application.screens);
        if (!visible || screens.length === 0 || screens.some(meets))
            return;
        const home = area.Window.window?.screen ?? screens[0];
        width = Math.min(width, home.width);
        height = Math.min(height, home.height);
        x = home.virtualX + (home.width - width) / 2;
        y = home.virtualY + (home.height - height) / 2;
    }

    title: units.map(one => area.title(one)).join(", ") + " · " + (area.page.view.title ?? "")
    width: size[0]
    height: size[1]
    color: Look.night
    visible: true
    onClosing: back()
    onWidthChanged: keep.restart()
    onHeightChanged: keep.restart()

    // The size is kept a moment after it settles, so closing the app with the window open keeps it.
    Timer {
        id: keep

        interval: 500
        onTriggered: window.area.layout.view.remember(window.units, [window.width, window.height])
    }
    onHeldChanged: leaving = false

    Connections {
        target: Qt.application

        function onScreensChanged() {
            Qt.callLater(window.keepOn);
        }
    }

    Instantiator {
        model: Qt.application.screens

        delegate: Connections {
            required property var modelData

            target: modelData

            function onVirtualXChanged() {
                Qt.callLater(window.keepOn);
            }

            function onWidthChanged() {
                Qt.callLater(window.keepOn);
            }

            function onHeightChanged() {
                Qt.callLater(window.keepOn);
            }
        }
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        Rectangle {
            Layout.fillWidth: true
            implicitHeight: DeskLook.strip
            color: DeskLook.stripTone

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: Look.e2
                anchors.rightMargin: Look.e2
                spacing: Look.e1

                Repeater {
                    model: window.units.length > 1 ? window.units : []

                    delegate: QQC2.ToolButton {
                        required property string modelData

                        icon.name: BenchWords.icon(modelData)
                        text: window.area.title(modelData)
                        checked: window.unit === modelData
                        onClicked: {
                            window.area.serial++;
                            window.area.layout.view.showIn(modelData);
                        }
                    }
                }

                Item {
                    Layout.fillWidth: true
                }

                QQC2.ToolButton {
                    icon.name: "sign-in"
                    text: App.tr("Return to the workbench")
                    onClicked: window.back()
                }
            }
        }

        Item {
            id: body

            Layout.fillWidth: true
            Layout.fillHeight: true
        }
    }
}
