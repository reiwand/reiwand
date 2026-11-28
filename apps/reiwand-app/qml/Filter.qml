import QtQuick
import io.github.reiwand.app

FocusScope {
    id: filter

    property var words: []
    property int current: 0
    property bool sections: false
    property string label
    property int keyed: current

    signal picked(int index)

    implicitWidth: row.implicitWidth
    implicitHeight: row.implicitHeight
    activeFocusOnTab: true
    Accessible.role: Accessible.PageTabList
    Accessible.name: label

    Row {
        id: row

        spacing: Look.e1

        Repeater {
            id: repeater

            model: filter.words

            Item {
                id: word

                required property var modelData
                required property int index
                readonly property bool first: index === 0
                readonly property bool lit: index === filter.current
                readonly property int side: filter.sections ? Look.sectionsSide : Look.filterSide
                readonly property bool keyed: filter.activeFocus && index === filter.keyed

                implicitWidth: line.implicitWidth + (first ? 0 : side) + side
                implicitHeight: filter.sections ? Look.menuHeight : Look.controlHeight
                Accessible.role: Accessible.PageTab
                Accessible.name: modelData.count === undefined ? modelData.text : modelData.text + " " + modelData.count
                Accessible.selected: lit
                Accessible.onPressAction: filter.pick(index)

                Row {
                    id: line

                    x: word.first ? 0 : word.side
                    anchors.verticalCenter: parent.verticalCenter
                    spacing: Look.filterGap

                    Text {
                        text: word.modelData.text
                        font.family: Look.titleFont
                        font.pixelSize: filter.sections ? Look.tRow : Look.tFilter
                        font.weight: word.lit ? Font.Bold : Font.Medium
                        color: word.lit || hover.hovered ? Look.frost : Look.mist
                    }

                    Text {
                        visible: word.modelData.count !== undefined
                        anchors.baseline: parent.children[0].baseline
                        text: word.modelData.count ?? ""
                        font.family: Look.textFont
                        font.pixelSize: Look.tCount
                        font.weight: Font.Medium
                        color: word.modelData.tone === "coral" ? Look.coral : Look.mist
                    }
                }

                Rectangle {
                    visible: word.lit
                    x: line.x
                    anchors.bottom: parent.bottom
                    anchors.bottomMargin: Look.markBottom
                    width: Look.markWidth
                    height: Look.markHeight
                    radius: Look.line
                    color: Look.moon
                }

                FocusHalo {
                    visible: word.keyed
                    corner: 0
                }

                HoverHandler {
                    id: hover

                    cursorShape: Qt.PointingHandCursor
                }

                TapHandler {
                    onTapped: filter.pick(word.index)
                }
            }
        }
    }

    function pick(index) {
        keyed = index;
        if (index !== current)
            picked(index);
    }

    onCurrentChanged: keyed = current
    Keys.onLeftPressed: keyed = Math.max(0, keyed - 1)
    Keys.onRightPressed: keyed = Math.min(repeater.count - 1, keyed + 1)
    Keys.onReturnPressed: pick(keyed)
    Keys.onEnterPressed: pick(keyed)
    Keys.onSpacePressed: pick(keyed)
}
