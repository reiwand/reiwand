import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.reiwand.app

QQC2.ItemDelegate {
    id: row

    required property var modelData
    required property var owner
    property var uses: []
    readonly property int tabbed: ListView.isCurrentItem ? Qt.StrongFocus : Qt.NoFocus
    readonly property string file: owner.page.editor.folder + "/" + modelData.path
    readonly property var still: (owner.frames[file] ?? [])[0]?.file ?? ""

    function texture(t) {
        if (!t)
            return [];
        return [t.hdr ? App.tr("HDR") : "", t.cube ? App.tr("cube map, shown as a panorama") : "", t.frames > 1 ? App.tr("%1 frames", t.frames) : ""].filter(w => w);
    }

    function ask() {
        const found = JSON.parse(owner.page.editor.rig("", JSON.stringify({
            "asset_uses": {
                "asset": modelData.id
            }
        })) || "null");
        uses = found?.uses ?? [];
        if (uses.length === 0)
            drop();
        else
            warning.open();
    }

    function drop() {
        owner.page.run("remove", {
            "pointer": modelData.pointer
        });
    }

    function opensMenu(x, y) {
        owner.page.editor.select(modelData.pointer);
        const image = modelData.kind === "image" && !modelData.path.toLowerCase().endsWith(".ktx2");
        owner.menu.show(row, x ?? row.width / 3, y ?? row.height, [
            {
                "text": App.tr("Rename…"),
                "icon": "edit-rename",
                "act": () => renaming.ask("asset", row.modelData.id)
            }
        ].concat(image ? [
            {
                "text": App.tr("Cut sprites…"),
                "icon": "view-grid",
                "act": () => row.owner.cutter.show(row.modelData.id)
            },
            {
                "text": App.tr("Convert to KTX2…"),
                "icon": "application-x-archive",
                "act": () => row.owner.packing.show(row.modelData.id, row.modelData.pointer)
            }
        ] : [], [
            {
                "separator": true
            },
            {
                "text": App.tr("Remove"),
                "icon": "edit-delete",
                "tone": "coral",
                "act": () => row.ask()
            }
        ]));
    }

    function place(use) {
        const last = use.pointer.split("/").pop();
        const where = SceneWords.label(last, use.pointer);
        return use.layer ? App.tr("%1 (%2): %3", use.layer.id, use.layer.type, where) : where;
    }

    width: ListView.view.width
    text: [modelData.id, SceneWords.kind(modelData.kind), App.bytes(modelData.size)].concat(texture(modelData.texture)).join(" · ") + (modelData.media ? " · " + (modelData.media === "current" ? App.tr("cover of what plays") : App.tr("previous cover")) : "")
    implicitHeight: DeskLook.row
    topPadding: 0
    bottomPadding: 0
    leftPadding: DeskLook.thumb + Look.e2 * 2
    rightPadding: remove.width * (cutting.visible ? 4 : 2) + Look.e1
    font.pixelSize: DeskLook.tRow
    marked: owner.page.editor.selected === modelData.pointer
    highlighted: marked && SceneFocus.lit(row)
    focusPolicy: Qt.NoFocus
    onClicked: owner.page.editor.select(modelData.pointer)
    Component.onCompleted: {
        if (modelData.kind === "video")
            owner.strips.ask(file, 1);
    }

    TapHandler {
        acceptedButtons: Qt.RightButton
        onTapped: eventPoint => row.opensMenu(eventPoint.position.x, eventPoint.position.y)
    }

    Rectangle {
        anchors.fill: parent
        anchors.margins: Look.ring
        visible: row.ListView.isCurrentItem && row.ListView.view.activeFocus
        color: "transparent"
        border.color: row.highlighted ? Look.night : Look.frost
        border.width: Look.ring
        radius: Look.radiusSmall
    }

    SceneAssetThumb {
        anchors.left: parent.left
        anchors.leftMargin: Look.e2
        anchors.verticalCenter: parent.verticalCenter
        size: DeskLook.thumb
        kind: row.modelData.kind
        file: row.file
        still: row.still
    }

    QQC2.ToolButton {
        anchors.right: cutting.left
        anchors.verticalCenter: parent.verticalCenter
        icon.name: "edit-rename"
        text: App.tr("Rename…")
        display: QQC2.AbstractButton.IconOnly
        focusPolicy: row.tabbed
        onClicked: renaming.ask("asset", row.modelData.id)
    }

    QQC2.ToolButton {
        id: cutting

        anchors.verticalCenter: parent.verticalCenter
        anchors.right: packed.left
        visible: row.modelData.kind === "image" && !row.modelData.path.toLowerCase().endsWith(".ktx2")
        icon.name: "view-grid"
        text: App.tr("Cut sprites…")
        display: QQC2.AbstractButton.IconOnly
        focusPolicy: row.tabbed
        QQC2.ToolTip.text: text
        QQC2.ToolTip.visible: hovered
        onClicked: row.owner.cutter.show(row.modelData.id)
    }

    QQC2.ToolButton {
        id: packed

        anchors.verticalCenter: parent.verticalCenter
        anchors.right: remove.left
        visible: row.modelData.kind === "image" && !row.modelData.path.toLowerCase().endsWith(".ktx2")
        icon.name: "application-x-archive"
        text: App.tr("Convert to KTX2…")
        display: QQC2.AbstractButton.IconOnly
        focusPolicy: row.tabbed
        QQC2.ToolTip.text: text
        QQC2.ToolTip.visible: hovered
        onClicked: row.owner.packing.show(row.modelData.id, row.modelData.pointer)
    }

    SceneRename {
        id: renaming

        page: row.owner.page
    }

    QQC2.ToolButton {
        id: remove

        anchors.verticalCenter: parent.verticalCenter
        anchors.right: parent.right
        icon.name: "edit-delete"
        text: App.tr("Remove")
        display: QQC2.AbstractButton.IconOnly
        focusPolicy: row.tabbed
        onClicked: row.ask()
    }

    Kirigami.PromptDialog {
        id: warning

        title: App.tr("%1 is still in use", row.modelData.id)
        subtitle: row.uses.map(use => "• " + row.place(use)).join("\n") + "\n\n" + App.tr("Removing it leaves these places without a picture, video or file. Remove it anyway?")
        standardButtons: Kirigami.Dialog.Cancel
        customFooterActions: [
            Kirigami.Action {
                text: App.tr("Remove anyway")
                icon.name: "edit-delete"
                onTriggered: {
                    row.drop();
                    warning.close();
                }
            }
        ]
    }
}
