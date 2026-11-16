import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

ColumnLayout {
    id: tool

    required property var page
    readonly property var parallax: page.values.parallax ?? {}
    readonly property bool moving: parallax.enabled ?? false
    readonly property var sources: ["auto", "pointer", "accelerometer"]
    readonly property var steps: [1, 8, 16, 32]
    readonly property var fills: ["lama", "simple"]
    readonly property var depthLayers: [4, 5, 6]
    readonly property var lama: JSON.parse(maps.assets || "[]").find(asset => asset.inpaint)
    readonly property var missing: JSON.parse(maps.assets || "[]").filter(asset => !asset.installed && !asset.inpaint)
    readonly property string sizes: missing.map(asset => asset.size).join(" + ")

    function send(key, value) {
        const args = {};
        args[key] = value;
        tool.page.run("parallax", args);
    }

    DepthMaps {
        id: maps

        Component.onCompleted: open(tool.page.item.path ?? "")
        onMapChanged: tool.page.editor.refresh()
    }

    QQC2.Switch {
        text: App.tr("Move the picture in depth")
        checked: tool.moving
        onToggled: tool.page.run("parallax", {
            "off": !checked
        })
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        type: Kirigami.MessageType.Warning
        visible: tool.moving && maps.map === "" && !maps.busy
        text: maps.ready ? App.tr("The picture stays still: it has no depth map yet. Make one in Depth above.") : App.tr("The picture stays still: the depth model is not installed. It is downloaded only if you ask (%1), then the map is made on this computer.", tool.sizes)
        actions: [
            Kirigami.Action {
                text: maps.ready ? App.tr("Generate the map") : App.tr("Download the depth model…")
                icon.name: maps.ready ? "view-preview" : "download"
                onTriggered: maps.ready ? maps.generate(tool.page.values.depth?.input ?? 518) : consent.open()
            }
        ]
    }

    RowLayout {
        visible: maps.busy

        QQC2.ProgressBar {
            Layout.fillWidth: true
            value: Math.max(0, maps.progress)
            indeterminate: maps.progress < 0
        }

        QQC2.ToolButton {
            icon.name: "process-stop"
            text: App.tr("Cancel")
            onClicked: maps.cancel()
        }
    }

    QQC2.ComboBox {
        Layout.fillWidth: true
        enabled: tool.moving
        model: [App.tr("Slow orbit"), App.tr("Pointer"), App.tr("Accelerometer")]
        currentIndex: Math.max(0, tool.sources.indexOf(tool.parallax.source))
        onActivated: index => tool.send("source", tool.sources[index])
    }

    QQC2.ComboBox {
        Layout.fillWidth: true
        enabled: tool.moving
        model: tool.steps.map(n => n === 1 ? App.tr("Fastest: shift only") : App.tr("%1 steps", n))
        currentIndex: Math.max(0, tool.steps.indexOf(tool.parallax.steps))
        onActivated: index => tool.send("steps", tool.steps[index])
    }

    QQC2.ComboBox {
        id: fillChoice

        Layout.fillWidth: true
        enabled: tool.moving
        model: tool.fills.map(fill => MachineWords.fills[fill])
        currentIndex: Math.max(0, tool.fills.indexOf(tool.parallax.fill ?? "lama"))
        onActivated: index => tool.send("fill", tool.fills[index])
    }

    QQC2.Label {
        Layout.fillWidth: true
        text: App.tr("Smart fill paints what a near object hid when it moves; simple fill stretches the edges.")
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        type: Kirigami.MessageType.Information
        visible: tool.moving && (tool.parallax.fill ?? "lama") === "lama" && tool.lama !== undefined && !tool.lama.installed
        text: tool.lama ? App.tr("The smart fill (LaMa) is not installed, so pictures use the simple fill. It is downloaded once from %1 (%2, license %3), checked against its SHA-256 and runs on this computer; computing the layers of a picture takes a few minutes.", tool.lama.hosts, tool.lama.size, tool.lama.license) : ""
        actions: [
            Kirigami.Action {
                text: App.tr("Download the smart fill (LaMa)")
                icon.name: "download"
                enabled: maps.allowed && !maps.busy
                onTriggered: maps.fetchLama()
            }
        ]
    }

    QQC2.ComboBox {
        id: layersChoice

        Layout.fillWidth: true
        enabled: tool.moving
        model: tool.depthLayers.map(count => App.tr("%1 layers", count))
        currentIndex: Math.max(0, tool.depthLayers.indexOf(tool.parallax.layers ?? 5))
        onActivated: index => tool.send("layers", tool.depthLayers[index])
    }

    QQC2.Label {
        Layout.fillWidth: true
        text: App.tr("More layers separate near and far better and take longer to compute.")
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
    }

    Repeater {
        model: [["strength", App.tr("Intensity"), 0, 0.03, 0.001, 3], ["focus", App.tr("Focus"), 0, 1, 0.01, 2], ["smoothing_ms", App.tr("Smoothing (ms)"), 0, 2000, 10, 0], ["overscan", App.tr("Zoom to hide the edges"), 1, 1.1, 0.005, 3]]

        delegate: EditSlider {
            required property var modelData

            enabled: tool.moving
            text: modelData[1]
            from: modelData[2]
            to: modelData[3]
            stepSize: modelData[4]
            decimals: modelData[5]
            value: tool.parallax[modelData[0]] ?? 0
            onMoved: value => tool.send(modelData[0] === "smoothing_ms" ? "smoothing" : modelData[0], value)
        }
    }

    EditLayersProgress {
        Layout.fillWidth: true
        page: tool.page
    }

    EditRegions {
        Layout.fillWidth: true
        visible: maps.map !== ""
        page: tool.page
        map: maps.map
    }

    QQC2.Label {
        Layout.fillWidth: true
        text: App.tr("Turn on Show on the desktop to try it. The parallax needs a depth map, pauses like videos and stays still when the system asks for reduced motion.")
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
    }

    DepthDialog {
        id: consent

        maps: maps
    }
}
