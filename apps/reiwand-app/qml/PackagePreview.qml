import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

ColumnLayout {
    id: preview

    required property var page
    property var stream: null
    property bool missing: false
    readonly property var rates: [0, 15, 30, 60]
    readonly property bool web: page.kind === "web"

    function connect() {
        stream?.destroy();
        stream = null;
        if (App.checking || page.editor.node === 0)
            return;
        try {
            stream = Qt.createQmlObject("import QtQuick; import org.kde.pipewire as PipeWire; PipeWire.PipeWireSourceItem { anchors.fill: parent }", screen);
            if (page.editor.serial > 0 && stream.objectSerial !== undefined)
                stream.objectSerial = page.editor.serial;
            else
                stream.nodeId = page.editor.node;
        } catch (error) {
            missing = true;
        }
    }

    function rate(fps) {
        const look = JSON.parse(page.editor.look || "{}");
        if (fps > 0)
            look.fps = fps;
        else
            delete look.fps;
        page.editor.look = JSON.stringify(look);
        SceneLook.preview(page);
    }

    function show(on) {
        if (on) {
            const look = JSON.parse(page.editor.look || "{}");
            look.show = true;
            page.editor.look = JSON.stringify(look);
            page.editor.attach(0);
            SceneLook.preview(page);
        }
        page.editor.cover(!on);
    }

    spacing: 0
    onVisibleChanged: show(visible)

    Connections {
        target: preview.page.editor

        function onSerialChanged() {
            preview.connect();
        }
    }

    // One compact row, as the scene canvas bar: the quality the preview is drawn at, its frame rate
    // and Reload (what it does is in its tooltip).
    Rectangle {
        Layout.fillWidth: true
        implicitHeight: Look.canvasBarHeight
        color: Look.nightHigh

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: Look.e2
            anchors.rightMargin: Look.e2
            spacing: Look.e2

            QQC2.ComboBox {
                Layout.fillWidth: true
                Layout.minimumWidth: Look.e8
                Layout.maximumWidth: Look.smallPanel * 0.7
                Accessible.name: App.tr("Preview quality")
                model: SceneLook.names.map((name, i) => [name, "Q" + i].join(" · "))
                currentIndex: SceneLook.levels.indexOf(SceneLook.quality)
                onActivated: index => {
                    SceneLook.quality = SceneLook.levels[index];
                    SceneLook.preview(preview.page);
                }
                QQC2.ToolTip.text: App.tr("Preview quality")
                QQC2.ToolTip.visible: hovered
            }

            QQC2.ComboBox {
                Layout.fillWidth: true
                Layout.minimumWidth: Look.e8
                Layout.maximumWidth: Look.smallPanel * 0.5
                Accessible.name: App.tr("Frame rate")
                model: preview.rates.map(r => r === 0 ? App.tr("Not capped") : App.tr("%1 frames a second", r))
                onActivated: index => preview.rate(preview.rates[index])
                QQC2.ToolTip.text: App.tr("Frame rate")
                QQC2.ToolTip.visible: hovered
            }

            Item {
                Layout.fillWidth: true
            }

            QQC2.ToolButton {
                icon.name: "arrows-clockwise"
                text: App.tr("Reload")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.text: preview.web ? App.tr("Reload builds the page again from its files, so what you changed in another editor shows; what the page writes with console.log and its errors appear in the console on the right.") : App.tr("Reload builds the preview again from its files, so what you changed in another editor shows.")
                QQC2.ToolTip.visible: hovered
                onClicked: SceneLook.again(preview.page)
            }
        }
    }

    Rectangle {
        id: area

        Layout.fillWidth: true
        Layout.fillHeight: true
        color: Look.night
        clip: true

        Rectangle {
            id: screen

            readonly property real fitted: Math.min(area.width / 16, area.height / 9)

            anchors.centerIn: parent
            width: fitted * 16
            height: fitted * 9
            color: "black"
            clip: true

            QQC2.Label {
                anchors.fill: parent
                anchors.margins: Look.e2
                visible: preview.stream === null
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
                wrapMode: Text.Wrap
                elide: Text.ElideRight
                fontSizeMode: Text.Fit
                minimumPixelSize: Look.tFoot * 0.8
                color: "white"
                text: preview.missing ? App.tr("Install the KPipeWire QML module to see the preview here.") : (preview.web ? App.tr("The page appears here while reiwandd runs.") : App.tr("The preview appears here while reiwandd runs."))
            }
        }
    }

    SceneFold {
        Layout.fillWidth: true
        Layout.leftMargin: Look.e2
        Layout.rightMargin: Look.e2
        key: "lights-of-the-package"
        title: preview.web ? App.tr("The page and its lights") : App.tr("The preview and its lights")
        open: false

        QQC2.Switch {
            Layout.fillWidth: true
            text: preview.web ? App.tr("Allow the page to send colors to the lights") : App.tr("Allow the package to send colors to the lights")
            checked: preview.page.manifest.permissions?.rgb === true
            onToggled: checked ? preview.page.change("/manifest/permissions/rgb", true) : preview.page.run("remove", {
                "pointer": "/manifest/permissions/rgb"
            })
        }

        LightsStrip {
            Layout.fillWidth: true
            output: preview.page.editor.output
            active: preview.visible
        }
    }
}
