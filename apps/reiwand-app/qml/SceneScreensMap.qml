import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The map of the chosen screens in the Screens panel: the scene's thumbnail cut into the part each
// screen shows, placed as the screens sit, each with its name. It shows the same parts the canvas
// frames (the query `spread`); without a thumbnail each part is a plain tile.
Item {
    id: map

    required property var page
    property string layout
    readonly property real bezel: App.state.options?.["layout.bezel_mm"] ?? 0
    readonly property var mine: Object.keys(App.state.outputs ?? {}).filter(name => App.state.outputs[name].mm).map(name => ({
                "id": name,
                "mm": App.state.outputs[name].mm
            }))
    readonly property var size: page.view.canvas?.size ?? App.fill("Offer.canvas").size
    readonly property var parts: {
        map.page.view;
        if (layout === "")
            return [];
        const query = {
            "spread": {
                "layout": layout,
                "bezel": bezel,
                "monitors": mine
            }
        };
        return (JSON.parse(page.editor.rig("", JSON.stringify(query)) || "null") ?? {}).parts ?? [];
    }
    readonly property string thumbnail: {
        map.page.view;
        const found = Publishing.thumbnail(JSON.parse(page.editor.value("/manifest") || "{}") ?? {});
        return found ? "file://" + page.editor.folder + "/" + found : "";
    }
    // The box around every screen, in pixels of the canvas: the map fits it to its width.
    readonly property var bounds: parts.length === 0 ? [0, 0, 1, 1] : (left => [left[0], left[1], Math.max(...parts.map(part => part.rect[0] + part.rect[2])) - left[0], Math.max(...parts.map(part => part.rect[1] + part.rect[3])) - left[1]])([Math.min(...parts.map(part => part.rect[0])), Math.min(...parts.map(part => part.rect[1]))])
    readonly property real scale: width / Math.max(1, bounds[2])
    readonly property real seam: Look.line * 2

    implicitHeight: parts.length > 0 ? bounds[3] * scale : 0
    visible: parts.length > 0
    Accessible.role: Accessible.Graphic
    Accessible.name: App.tr("Map of the screens: %1", parts.map(part => part.id).join(", "))

    Repeater {
        model: map.parts

        delegate: Rectangle {
            id: part

            required property var modelData

            x: (modelData.rect[0] - map.bounds[0]) * map.scale + map.seam / 2
            y: (modelData.rect[1] - map.bounds[1]) * map.scale + map.seam / 2
            width: modelData.rect[2] * map.scale - map.seam
            height: modelData.rect[3] * map.scale - map.seam
            radius: Look.radius / 2
            color: Look.nightHigh
            border.color: Look.nightLine
            border.width: Look.line
            clip: true

            Image {
                x: -part.x - map.bounds[0] * map.scale
                y: -part.y - map.bounds[1] * map.scale
                width: map.size[0] * map.scale
                height: map.size[1] * map.scale
                visible: map.thumbnail !== ""
                source: map.thumbnail
                fillMode: Image.PreserveAspectCrop
                asynchronous: true
                sourceSize.width: width
            }

            QQC2.Label {
                x: Look.e1
                y: Look.e1
                text: part.modelData.id
                textFormat: Text.PlainText
                color: Look.frost
                style: Text.Outline
                styleColor: Look.night
                font.pixelSize: DeskLook.tCap
            }
        }
    }
}
