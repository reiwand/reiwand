import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

// The kinds of asset the list of assets can be narrowed to, as filter words: "All" and each kind
// the scene has, the one in use with the mark under it.
Flow {
    id: kinds

    property var assets: []
    property string current: ""
    readonly property var order: ["image", "video", "sound", "model", "font", "lut"]
    readonly property var present: order.filter(kind => assets.some(asset => asset.kind === kind))

    function rank(kind) {
        const at = order.indexOf(kind);
        return at < 0 ? order.length : at;
    }

    function title(kind) {
        return {
            "": App.tr("All"),
            "image": App.tr("Images"),
            "video": App.tr("Videos"),
            "sound": App.tr("Sounds"),
            "model": App.tr("Meshes and models"),
            "font": App.tr("Fonts"),
            "lut": App.tr("Color tables")
        }[kind] ?? kind;
    }

    spacing: 0
    Accessible.role: Accessible.PageTabList
    Accessible.name: App.tr("Kinds of asset")

    Repeater {
        model: [""].concat(kinds.present)

        delegate: QQC2.ToolButton {
            id: word

            required property string modelData
            readonly property bool lit: kinds.current === modelData

            implicitHeight: DeskLook.word
            leftPadding: Look.e2
            rightPadding: Look.e2
            text: kinds.title(modelData)
            font.family: Look.textFont
            font.pixelSize: DeskLook.tBar
            font.weight: lit ? Font.DemiBold : Font.Normal
            Accessible.role: Accessible.PageTab
            Accessible.selected: lit
            onClicked: kinds.current = modelData

            contentItem: Text {
                text: word.text
                font: word.font
                color: word.lit || word.hovered || word.visualFocus ? Look.frost : Look.mist
                verticalAlignment: Text.AlignVCenter
            }

            background: Item {
                Rectangle {
                    x: word.leftPadding
                    y: parent.height - height - Look.line
                    width: Math.min(Look.markWidth, word.width - 2 * word.leftPadding)
                    height: DeskLook.markHeight
                    radius: Look.line
                    visible: word.lit
                    color: Look.moon
                }

                Ring {
                    anchors.margins: Look.ring
                    visible: word.visualFocus
                }
            }
        }
    }
}
