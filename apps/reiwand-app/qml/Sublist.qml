import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

Column {
    id: sublist

    property var model: []
    property string addText
    property string removeText: App.tr("Remove")
    property string otherText
    property string otherIcon
    property bool movable: false
    property int keep: -1
    readonly property alias rows: repeater
    readonly property alias drag: drag

    signal removed(int index)
    signal added
    signal otherAdded
    signal reordered(int from, int to)

    function move(from, to) {
        if (!movable || to < 0 || to >= repeater.count || to === from)
            return;
        keep = to;
        reordered(from, to);
    }

    onModelChanged: keep >= 0 && Qt.callLater(() => {
        const item = repeater.itemAt(keep);
        keep = -1;
        if (item)
            item.forceActiveFocus(Qt.TabFocusReason);
    })

    bottomPadding: Look.e2
    Accessible.role: Accessible.List

    SublistDrag {
        id: drag

        list: sublist
        visible: false
        onDropped: (from, to) => sublist.move(from, Math.max(0, Math.min(repeater.count - 1, to)))
    }

    Repeater {
        id: repeater

        model: sublist.model

        FocusScope {
            id: item

            required property var modelData
            required property int index
            readonly property bool lit: activeFocus
            readonly property alias handle: handle

            x: Look.rowSide
            width: sublist.width - Look.rowSide
            implicitHeight: Look.sublistHeight
            activeFocusOnTab: true
            Accessible.role: Accessible.ListItem
            z: drag.held === index ? 1 : 0
            Accessible.name: [modelData.name, modelData.where ?? "", modelData.problem ?? ""].filter(part => part !== "").join(", ")
            Accessible.description: sublist.movable ? App.tr("Ctrl+Up and Ctrl+Down change its place") : ""
            Keys.onUpPressed: event => sublist.movable && event.modifiers & Qt.ControlModifier ? sublist.move(index, index - 1) : nextItemInFocusChain(false).forceActiveFocus(Qt.TabFocusReason)
            Keys.onDownPressed: event => sublist.movable && event.modifiers & Qt.ControlModifier ? sublist.move(index, index + 1) : nextItemInFocusChain(true).forceActiveFocus(Qt.TabFocusReason)
            Keys.onDeletePressed: sublist.removeText !== "" && sublist.removed(index)
            transform: Translate {
                y: drag.held === item.index ? drag.lift : 0
            }

            Rectangle {
                anchors.fill: parent
                topRightRadius: Look.radius
                bottomRightRadius: Look.radius
                color: item.lit ? Look.frost : "transparent"
            }

            Rectangle {
                width: Look.line
                height: parent.height
                color: item.lit ? Look.frost : Look.nightLine
            }

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: Look.e4
                anchors.rightMargin: Look.rowEnd
                spacing: Look.listGap

                IconImage {
                    id: handle

                    visible: sublist.movable
                    sourceSize.width: Look.icon
                    sourceSize.height: Look.icon
                    name: "handle-sort"
                    color: item.lit ? Look.night : Look.mist

                    DragHandler {
                        target: null
                        xAxis.enabled: false
                        cursorShape: Qt.SizeVerCursor
                        onActiveChanged: active ? drag.begin(item.index, centroid.scenePressPosition, item.height) : drag.end()
                        onCentroidChanged: active && drag.follow(centroid.scenePosition)
                    }
                }

                Photo {
                    visible: item.modelData.picture !== undefined
                    Layout.preferredWidth: Look.thumbnail
                    Layout.preferredHeight: Look.thumbnail * 9 / 16
                    corner: Look.radiusSmall
                    source: item.modelData.picture ?? ""
                    sourceSize.width: Look.thumbnail * 2
                }

                Text {
                    Layout.maximumWidth: implicitWidth
                    Layout.preferredWidth: implicitWidth
                    elide: Text.ElideRight
                    font.family: Look.textFont
                    font.pixelSize: Look.tBody
                    color: item.lit ? Look.night : Look.frost
                    textFormat: Text.PlainText
                    text: item.modelData.name
                }

                Text {
                    visible: (item.modelData.where ?? "") !== ""
                    font.family: Look.textFont
                    font.pixelSize: Look.tFoot
                    color: item.lit ? Look.night : Look.mist
                    text: "· " + (item.modelData.where ?? "")
                }

                Text {
                    visible: (item.modelData.problem ?? "") !== ""
                    font.family: Look.textFont
                    font.pixelSize: Look.tFoot
                    color: Tone.of("coral", item.lit, Look.coral)
                    text: "· " + (item.modelData.problem ?? "")
                }

                Item {
                    Layout.fillWidth: true
                }

                QQC2.Button {
                    id: remove

                    visible: sublist.removeText !== ""
                    flat: true
                    focusPolicy: Qt.NoFocus
                    text: sublist.removeText
                    icon.name: "x"
                    icon.color: item.lit ? Look.night : Look.mist
                    Accessible.name: sublist.removeText + " " + item.modelData.name
                    onClicked: sublist.removed(item.index)
                    contentItem: IconLabel {
                        spacing: Look.buttonGap
                        icon: remove.icon
                        text: remove.text
                        font: remove.font
                        color: item.lit ? Look.night : Look.frost
                    }
                }
            }
        }
    }

    SublistFoot {
        sublist: sublist
    }
}
