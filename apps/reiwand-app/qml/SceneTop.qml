import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.reiwand.app

RowLayout {
    id: top

    required property var page
    readonly property var history: page.view.history ?? ({})
    readonly property bool dirty: page.editor.dirty
    readonly property bool filled: (page.view.layers ?? []).length > 0
    property real roomy: 0
    // The room Save now takes while it is shown; the words are measured without it.
    readonly property real saving: saveNow.visible ? saveNow.implicitWidth + spacing : 0
    // Short of room the bar gives up, in turn, the words of Undo and Redo, the names of the layouts
    // not in use and the words of the buttons on the right.
    // Undo and Redo are not on a folded bar, so their words free no room there.
    readonly property real undoWords: folded ? 0 : undoText.advanceWidth + redoText.advanceWidth + 2 * DeskLook.gap
    readonly property real layoutWords: layouts.item?.names ?? 0
    readonly property real free: width - saving
    readonly property int stage: width <= 0 || free >= roomy ? 0 : free >= roomy - undoWords ? 1 : free >= roomy - undoWords - layoutWords ? 2 : 3
    readonly property bool tight: stage >= 3
    readonly property bool bench: page.dense === true
    readonly property bool folded: words.item?.folded ?? false

    spacing: Look.e1
    onImplicitWidthChanged: {
        if (stage === 0 || implicitWidth - saving > roomy)
            roomy = implicitWidth - saving;
    }

    TextMetrics {
        id: undoText

        font.pixelSize: top.bench ? DeskLook.tWord : Look.tBody
        text: App.tr("Undo")
    }

    TextMetrics {
        id: redoText

        font.pixelSize: top.bench ? DeskLook.tWord : Look.tBody
        text: App.tr("Redo")
    }

    StatusLine {
        text: top.dirty ? App.tr("Unsaved ·") : App.tr("Saved")
        tone: top.dirty ? "" : "sprout"
        icon: top.dirty ? "circle-half" : top.bench ? "check-circle-fill" : "check-circle"
        size: top.bench ? DeskLook.statusIcon : Look.statusIcon
        pixels: top.bench ? DeskLook.tStatus : Look.tBody
        ink: top.bench ? Look.mist : Look.frost
    }

    QQC2.Button {
        id: saveNow

        visible: top.dirty
        flat: true
        implicitHeight: top.bench ? DeskLook.button : Look.topButtonHeight
        font.pixelSize: top.bench ? DeskLook.tWord : Look.tBody
        text: App.tr("Save now")
        enabled: !top.page.editor.busy
        onClicked: top.page.editor.save()
    }

    Loader {
        id: words

        Layout.leftMargin: Look.e2
        active: top.bench
        visible: active

        sourceComponent: SceneMenus {}

        onLoaded: {
            item.page = Qt.binding(() => top.page);
            top.page.menus = item;
        }
    }

    Item {
        Layout.fillWidth: true
    }

    Loader {
        id: layouts

        active: top.bench
        visible: active

        sourceComponent: SceneLayoutWords {}

        onLoaded: {
            item.page = Qt.binding(() => top.page);
            item.tight = Qt.binding(() => top.stage >= 2);
        }
    }

    Item {
        Layout.fillWidth: true
    }

    QQC2.Button {
        flat: true
        visible: !top.folded
        icon.name: "arrow-counter-clockwise"
        implicitHeight: top.bench ? DeskLook.button : Look.topButtonHeight
        font.pixelSize: top.bench ? DeskLook.tWord : Look.tBody
        text: App.tr("Undo")
        display: top.stage >= 1 ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
        enabled: (top.history.done ?? []).length > 0
        QQC2.ToolTip.text: top.history.undo ? App.tr("Undo: %1", top.history.undo) : ""
        QQC2.ToolTip.visible: hovered && QQC2.ToolTip.text !== ""
        onClicked: top.page.run("undo")
    }

    QQC2.Button {
        flat: true
        visible: !top.folded
        icon.name: "arrow-clockwise"
        implicitHeight: top.bench ? DeskLook.button : Look.topButtonHeight
        font.pixelSize: top.bench ? DeskLook.tWord : Look.tBody
        text: App.tr("Redo")
        display: top.stage >= 1 ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
        enabled: (top.history.undone ?? []).length > 0
        QQC2.ToolTip.text: top.history.redo ? App.tr("Redo: %1", top.history.redo) : ""
        QQC2.ToolTip.visible: hovered && QQC2.ToolTip.text !== ""
        onClicked: top.page.run("redo")
    }

    Rectangle {
        Layout.leftMargin: Look.e1
        Layout.rightMargin: Look.e1
        visible: !top.folded
        implicitWidth: Look.line
        implicitHeight: top.bench ? DeskLook.sepHeight : Look.betweenHeight
        color: Look.nightLine
    }

    QQC2.Button {
        id: view

        leftPadding: top.bench ? Look.e3 : Look.e4
        rightPadding: top.bench ? Look.e3 : Look.e4

        implicitHeight: top.bench ? DeskLook.button : Look.topButtonHeight
        font.pixelSize: top.bench ? DeskLook.tWord : Look.tBody
        icon.name: "monitor-play"
        text: App.tr("View as wallpaper")
        display: top.tight ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
        QQC2.ToolTip.text: text
        QQC2.ToolTip.visible: top.tight && hovered
        visible: top.page.viewable !== false
        enabled: top.page.editor.ready
        onClicked: top.page.use.view(view)
    }

    QQC2.Button {
        id: place

        leftPadding: top.bench ? Look.e3 : Look.e4
        rightPadding: top.bench ? Look.e3 : Look.e4

        implicitHeight: top.bench ? DeskLook.button : Look.topButtonHeight
        font.pixelSize: top.bench ? DeskLook.tWord : Look.tBody
        icon.name: "image"
        text: App.tr("Set as wallpaper")
        display: top.tight ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
        QQC2.ToolTip.text: text
        QQC2.ToolTip.visible: top.tight && hovered
        enabled: top.page.editor.ready
        onClicked: top.page.use.place(place)
    }

    QQC2.Button {
        implicitHeight: top.bench ? DeskLook.button : Look.topButtonHeight
        font.pixelSize: top.bench ? DeskLook.tWord : Look.tBody
        highlighted: top.filled
        palette.highlight: Look.moon
        icon.name: "paper-plane-tilt"
        text: App.tr("Publish…")
        enabled: top.page.editor.ready
        onClicked: top.page.use.publish()
    }

    SceneTopKeys {
        visible: false
        bar: top
    }
}
