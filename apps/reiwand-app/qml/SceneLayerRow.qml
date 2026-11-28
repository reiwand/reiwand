import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

// One layer in the list of layers: a row of the workbench's density with the handle to drag it, its
// thumbnail (the picture of an image layer, the icon of its kind otherwise), its name, what it is
// and the buttons that hide and lock it (and show it alone, while that is on or the row is in use).
QQC2.ItemDelegate {
    id: row

    required property var owner
    required property var modelData
    property bool adding: false

    width: ListView.view.width
    implicitHeight: DeskLook.row
    verticalPadding: 0
    rightPadding: Look.ringRoom
    marked: modelData.pointer === row.owner.page.editor.selected || row.owner.page.picked.includes(modelData.id)
    highlighted: marked && SceneFocus.lit(row)
    readonly property bool lit: highlighted
    readonly property color ink: lit ? Look.night : Look.frost
    readonly property color soft: lit ? Look.night : Look.mist
    readonly property string detail: modelData.effects === 1 ? App.tr("1 effect") : modelData.effects > 1 ? App.tr("%1 effects", modelData.effects) : SceneWords.kind(modelData.type)

    leftPadding: Look.e1 + modelData.depth * Look.e4
    readonly property bool ringed: ListView.isCurrentItem && ListView.view.activeFocus
    focusPolicy: Qt.NoFocus
    onClicked: row.owner.pick(modelData, adding)

    function opensMenu() {
        row.owner.menu.onLayer(row.modelData, row, row.width / 3, row.height);
    }

    TapHandler {
        acceptedButtons: Qt.LeftButton | Qt.RightButton
        onPressedChanged: pressed && (row.adding = (point.modifiers & Qt.ControlModifier) !== 0)
        onTapped: (eventPoint, button) => button === Qt.RightButton && row.owner.menu.onLayer(row.modelData, row, eventPoint.position.x, eventPoint.position.y)
    }

    function under(point) {
        const view = row.ListView.view;
        const at = view.contentItem.mapFromItem(null, point.x, point.y);
        return row.owner.shown[view.indexAt(at.x, at.y)] ?? null;
    }

    Rectangle {
        anchors.left: parent.left
        anchors.right: parent.right
        height: Look.ring
        visible: row.owner.target === row.modelData.pointer
        color: Look.moon
    }

    background: Rectangle {
        color: row.lit ? Look.frost : row.hovered ? Look.nightLine : row.marked ? Look.nightHigh : "transparent"
        border.width: row.marked && !row.lit ? Look.line : 0
        border.color: Look.nightLine

        Rectangle {
            anchors.fill: parent
            anchors.margins: Look.ring
            visible: row.ringed
            color: "transparent"
            border.color: row.lit ? Look.night : Look.frost
            border.width: Look.ring
            radius: Look.radiusSmall
        }
    }

    contentItem: RowLayout {
        spacing: Look.e2

        IconImage {
            name: "dots-six-vertical"
            sourceSize: Qt.size(DeskLook.tabIcon, DeskLook.tabIcon)
            Layout.preferredWidth: DeskLook.tabIcon / 2 + Look.e1
            Layout.preferredHeight: DeskLook.tabIcon
            fillMode: Image.Pad
            color: row.soft
            opacity: grip.active || row.hovered ? 1 : 0.35
            Accessible.name: App.tr("Drag to move the layer")

            HoverHandler {
                cursorShape: Qt.SizeVerCursor
            }

            DragHandler {
                id: grip

                target: null
                xAxis.enabled: false
                onCentroidChanged: {
                    if (active)
                        row.owner.target = row.under(centroid.scenePosition)?.pointer ?? "";
                }
                onActiveChanged: {
                    if (!active)
                        row.owner.drop(row.modelData, row.owner.shown.find(layer => layer.pointer === row.owner.target));
                }
            }
        }

        Rectangle {
            Layout.preferredWidth: DeskLook.thumbWide
            Layout.preferredHeight: DeskLook.thumb
            radius: Look.radiusSmall
            color: row.lit ? Qt.darker(Look.frost, 1.12) : Look.nightHigh
            border.width: Look.line
            border.color: row.lit ? Qt.darker(Look.frost, 1.3) : Look.nightLine
            clip: true

            Image {
                id: picture

                anchors.fill: parent
                anchors.margins: Look.line
                visible: status === Image.Ready
                source: row.modelData.picture ? App.url(row.owner.page.editor.folder + "/" + row.modelData.picture) : ""
                sourceSize.width: DeskLook.thumbWide * 2
                fillMode: Image.PreserveAspectCrop
                asynchronous: true
            }

            IconImage {
                anchors.centerIn: parent
                visible: !picture.visible
                name: row.owner.icon(row.modelData.type)
                sourceSize: Qt.size(DeskLook.tabIcon, DeskLook.tabIcon)
                color: row.soft
            }
        }

        Text {
            text: row.modelData.label ?? row.modelData.id
            textFormat: Text.PlainText
            elide: Text.ElideRight
            Layout.fillWidth: !kind.visible
            Layout.maximumWidth: row.availableWidth / 2
            color: row.ink
            font.family: Look.textFont
            font.pixelSize: DeskLook.tRow
            font.weight: row.lit ? Font.DemiBold : Font.Normal
            opacity: row.modelData.hidden ? 0.5 : 1
        }

        Text {
            id: kind

            Layout.fillWidth: true
            visible: row.width > DeskLook.rowRoomy
            text: row.detail
            textFormat: Text.PlainText
            elide: Text.ElideRight
            color: row.soft
            font.family: Look.textFont
            font.pixelSize: DeskLook.tKind
        }

        Repeater {
            model: [["hide", "hidden", row.modelData.hidden, "eye-slash", "eye", App.tr("Hide")], ["lock", "locked", row.modelData.locked, "lock-simple", "lock-simple-open", App.tr("Lock")], ["solo", "", row.modelData.solo, "favorite", "favorite", App.tr("Solo")]]

            delegate: QQC2.ToolButton {
                required property var modelData
                required property int index
                readonly property bool on: modelData[2] === true
                // Solo keeps its place while it is out of sight, so the eyes and locks of every
                // row stay in one column.
                readonly property bool shown: index < 2 || on || row.hovered || row.ListView.isCurrentItem

                implicitWidth: DeskLook.rowButton
                implicitHeight: DeskLook.rowButton
                padding: (DeskLook.rowButton - DeskLook.tabIcon) / 2
                enabled: shown
                icon.name: on ? modelData[3] : modelData[4]
                icon.width: DeskLook.tabIcon
                icon.height: DeskLook.tabIcon
                icon.color: row.ink
                text: modelData[5]
                display: QQC2.AbstractButton.IconOnly
                opacity: !shown ? 0 : on || hovered || visualFocus ? 1 : 0.55
                focusPolicy: row.ListView.isCurrentItem ? Qt.StrongFocus : Qt.NoFocus
                Accessible.checkable: true
                Accessible.checked: on
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: {
                    const args = {
                        "id": row.modelData.id
                    };
                    if (modelData[1])
                        args[modelData[1]] = !on;
                    row.owner.page.run(modelData[0], modelData[0] === "solo" && on ? {} : args);
                }
            }
        }
    }
}
