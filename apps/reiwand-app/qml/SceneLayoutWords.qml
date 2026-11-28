import QtQuick
import io.github.reiwand.app

// The layouts in the top bar of the editor: the six it comes with and, when one of the author's is
// in use, that one. Ctrl+1 to Ctrl+6 choose the six, Ctrl+7 to Ctrl+9 the author's first three.
// When the bar is short of room only the layout in use keeps its name.
Row {
    id: words

    property var page: null
    readonly property var layout: page?.layout ?? null
    property bool tight: false
    readonly property var shown: layout ? layout.layouts.filter(entry => entry.own < 0 || entry === layout.current) : []
    // How much narrower the row is with only the layout in use named.
    readonly property real names: shown.filter(entry => entry !== layout.current).reduce((sum, entry) => sum + wide(entry.name) + DeskLook.gap, 0)

    function wide(name) {
        return metrics.advanceWidth(name);
    }

    spacing: 0

    FontMetrics {
        id: metrics

        font.family: Look.titleFont
        font.pixelSize: DeskLook.tLayout
        font.weight: Font.Medium
    }

    Repeater {
        model: words.shown

        delegate: SceneLayoutWord {
            required property var modelData

            entry: modelData
            layout: words.layout
            named: !words.tight || lit
        }
    }

    Instantiator {
        model: 9

        delegate: Shortcut {
            required property int index

            sequence: "Ctrl+" + (index + 1)
            enabled: words.page?.visible === true && words.layout !== null
            onActivated: words.layout.chooseKey(index + 1)
        }
    }
}
