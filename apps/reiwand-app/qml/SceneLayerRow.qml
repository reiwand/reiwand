import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

QQC2.ItemDelegate {
    id: row

    required property var owner
    required property var modelData
    property bool adding: false

    width: ListView.view.width
    highlighted: modelData.pointer === row.owner.page.editor.selected || row.owner.page.picked.includes(modelData.id)
    readonly property bool lit: highlighted
    readonly property color ink: lit ? Look.noche : Look.escarcha

    leftPadding: Kirigami.Units.largeSpacing * (1 + modelData.depth * 2)
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
        height: 2
        visible: row.owner.target === row.modelData.pointer
        color: Kirigami.Theme.highlightColor
    }

    background: Rectangle {
        color: row.lit ? Look.escarcha : row.hovered ? Look.nocheLinea : "transparent"

        Rectangle {
            anchors.fill: parent
            anchors.margins: 3
            visible: row.ringed
            color: "transparent"
            border.color: Look.noche
            border.width: 2
            radius: Kirigami.Units.smallSpacing
        }
    }

    contentItem: RowLayout {
        IconImage {
            name: "handle-sort"
            sourceSize: Qt.size(Kirigami.Units.iconSizes.small, Kirigami.Units.iconSizes.small)
            Layout.preferredWidth: Kirigami.Units.iconSizes.small
            Layout.preferredHeight: Kirigami.Units.iconSizes.small
            color: row.ink
            opacity: grip.active ? 1 : 0.5
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

        IconImage {
            name: row.owner.icon(row.modelData.type)
            sourceSize: Qt.size(Kirigami.Units.iconSizes.small, Kirigami.Units.iconSizes.small)
            Layout.preferredWidth: Kirigami.Units.iconSizes.small
            Layout.preferredHeight: Kirigami.Units.iconSizes.small
            color: row.ink
        }

        QQC2.Label {
            Layout.fillWidth: true
            text: row.modelData.label ?? row.modelData.id
            textFormat: Text.PlainText
            elide: Text.ElideRight
            color: row.ink
            font.weight: row.lit ? Font.DemiBold : Font.Normal
            opacity: row.modelData.hidden ? 0.5 : 1
        }

        Repeater {
            model: [["hide", "hidden", row.modelData.hidden, "view-hidden", App.tr("Hide")], ["lock", "locked", row.modelData.locked, "object-locked", App.tr("Lock")], ["solo", "", row.modelData.solo, "favorite", App.tr("Solo")]]

            delegate: QQC2.ToolButton {
                required property var modelData

                icon.name: modelData[3]
                icon.color: row.ink
                text: modelData[4]
                display: QQC2.AbstractButton.IconOnly
                checkable: true
                checked: modelData[2]
                opacity: checked ? 1 : 0.4
                focusPolicy: row.ListView.isCurrentItem ? Qt.StrongFocus : Qt.NoFocus
                onClicked: {
                    const args = {
                        "id": row.modelData.id
                    };
                    if (modelData[1])
                        args[modelData[1]] = checked;
                    row.owner.page.run(modelData[0], modelData[0] === "solo" && !checked ? {} : args);
                }
            }
        }
    }
}
