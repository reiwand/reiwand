import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.reiwand.app

Rectangle {
    id: group

    required property var area
    required property var layout
    required property int index
    readonly property var held: layout?.groups[index] ?? null
    readonly property var units: held?.panels ?? []
    readonly property string current: held?.shown ?? ""
    readonly property bool bare: units.includes("canvas")
    // A folded side: icons on a strip; the one unfolded is drawn over the workbench on its own.
    readonly property bool rail: held?.rail === true
    readonly property alias body: body
    readonly property real strip: DeskLook.strip + Look.line
    property var hidden: []
    readonly property bool focused: {
        for (let item = Window.activeFocusItem; item; item = item.parent)
            if (item === group)
                return true;
        return false;
    }

    function menuOf(unit) {
        return Array.from(tabs.children).find(item => item.modelData === unit)?.menu ?? null;
    }

    // The tabs that do not fit on the strip go to its "⋯" menu, in their order; the tab shown
    // always keeps its place on the strip.
    function measure() {
        const widths = units.map((unit, at) => [unit, row.itemAt(at)?.implicitWidth ?? 0]);
        let room = tabs.width - (widths.find(pair => pair[0] === current)?.[1] ?? 0);
        const out = [];
        for (const [unit, wide] of widths) {
            if (unit === current)
                continue;
            if (out.length === 0 && wide <= room)
                room -= wide;
            else
                out.push(unit);
        }
        if (JSON.stringify(out) !== JSON.stringify(hidden))
            hidden = out;
    }

    function more(button) {
        if (hidden.length === 0)
            return menuOf(current)?.popup(button, 0, button.height);
        const entries = hidden.map(unit => ({
                    "text": area.title(unit),
                    "icon": BenchWords.icon(unit),
                    "act": () => {
                        layout.reveal(unit);
                        Qt.callLater(group.focusTab, unit);
                    }
                }));
        overflow.show(button, 0, button.height, entries.concat([
            {
                "separator": true
            },
            {
                "text": App.tr("Move or close %1…", area.title(current)),
                "icon": "dots-three",
                "act": () => menuOf(current)?.popup(button, 0, button.height)
            }
        ]));
    }

    function focusTab(unit) {
        const tab = Array.from(tabs.children).find(item => item.modelData === unit);
        tab?.forceActiveFocus(Qt.TabFocusReason);
    }

    x: held?.x ?? 0
    y: held?.y ?? 0
    width: held?.width ?? 0
    height: held?.height ?? 0
    color: rail ? DeskLook.stripTone : Look.night
    clip: true
    z: held?.overlay ? 5 : 0
    border.width: held?.overlay ? Look.line : 0
    border.color: Look.nightLine

    Flow {
        readonly property bool flat: ["top", "bottom"].includes(group.held?.side) // icons in a row
        visible: group.rail
        readonly property real knob: DeskLook.strip + Look.e1 - 2 * Look.ringRoom // rings fit the clip
        flow: flat ? Flow.LeftToRight : Flow.TopToBottom
        x: flat ? Look.ringRoom : (parent.width - knob) / 2
        y: Look.ringRoom // a flat rail made taller keeps its icons at the top
        width: flat ? parent.width - 2 * Look.ringRoom : knob
        height: flat ? knob : parent.height - 2 * Look.ringRoom
        spacing: Look.e1

        Repeater {
            model: group.rail ? group.units : []

            delegate: QQC2.ToolButton {
                required property string modelData

                width: parent.knob
                height: width
                padding: (width - DeskLook.tabIcon) / 2
                icon.name: BenchWords.icon(modelData)
                icon.width: DeskLook.tabIcon
                icon.height: DeskLook.tabIcon
                display: QQC2.AbstractButton.IconOnly
                checkable: true
                checked: group.current === modelData
                text: group.area.title(modelData)
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: group.layout.view.unfold(modelData)
            }
        }
    }

    Rectangle {
        visible: !group.rail
        width: parent.width
        height: group.strip
        color: DeskLook.stripTone

        Rectangle {
            y: parent.height - height
            width: parent.width
            height: Look.line
            color: Look.nightLine
        }

        QQC2.ToolButton {
            x: parent.width - width - Look.ringRoom
            y: (parent.height - Look.line - height) / 2
            width: DeskLook.strip - 2 * Look.ringRoom
            height: width
            padding: (width - DeskLook.tabIcon) / 2
            icon.name: "dots-three"
            icon.width: DeskLook.tabIcon
            icon.height: DeskLook.tabIcon
            display: QQC2.AbstractButton.IconOnly
            text: group.hidden.length > 0 ? App.tr("More tabs: %1", group.hidden.map(unit => group.area.title(unit)).join(", ")) : App.tr("Move or close %1", group.area.title(group.current))
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            onClicked: group.more(this)
        }
    }

    SceneMenu {
        id: overflow
    }

    onWidthChanged: Qt.callLater(measure)
    onUnitsChanged: Qt.callLater(measure)
    onCurrentChanged: Qt.callLater(measure)

    Row {
        id: tabs

        visible: !group.rail
        width: parent.width - DeskLook.strip
        height: DeskLook.strip
        spacing: 0
        clip: true

        Repeater {
            id: row

            model: group.units

            delegate: SceneGroupTab {
                home: group
            }
        }
    }

    Item {
        id: pane

        visible: !group.rail
        y: group.strip
        width: parent.width
        height: parent.height - y
        clip: true

        Item {
            id: body

            readonly property real room: group.bare ? 0 : Look.ringRoom

            x: room
            y: room
            width: parent.width - 2 * room
            height: parent.height - 2 * room
        }
    }
}
