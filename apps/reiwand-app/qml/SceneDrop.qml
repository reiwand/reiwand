import QtQuick
import QtQuick.Controls as QQC2
import io.github.reiwand.app

Item {
    id: drop

    required property var area
    readonly property var layout: area.layout
    readonly property real size: Look.e6
    readonly property real gap: Look.e1
    property string carried: ""
    property point spot
    property var under: null
    readonly property var edges: carried ? usable(["left", "right", "up", "down"].map(side => border(side))) : []
    readonly property var cross: under ? usable(crossOf(under)) : []
    readonly property var targets: cross.concat(edges)
    readonly property var hit: targets.find(target => within(target, spot)) ?? null
    readonly property var choice: hit ?? cross.find(target => target.side === "") ?? null
    readonly property string next: choice ? layout.placing(carried, choice) : ""
    readonly property var zone: next ? layout.zone(next, carried) : null

    function aim() {
        if (!carried)
            under = null;
        else if (!cross.some(target => within(target, spot)))
            under = layout.groups.find(group => within(group, spot)) ?? null;
    }

    function within(box, at) {
        return at.x >= box.x && at.x < box.x + box.width && at.y >= box.y && at.y < box.y + box.height;
    }

    function target(kind, side, beside, x, y) {
        return {
            "key": kind + side,
            "kind": kind,
            "side": side,
            "beside": beside,
            "x": x,
            "y": y,
            "width": size,
            "height": size
        };
    }

    function border(side) {
        const x = {
            "left": Look.e2,
            "right": width - Look.e2 - size
        }[side] ?? (width - size) / 2;
        const y = {
            "up": Look.e2,
            "down": height - Look.e2 - size
        }[side] ?? (height - size) / 2;
        return target("edge", side, "", x, y);
    }

    function crossOf(group) {
        const step = size + gap;
        const band = Look.e2 + step + step;
        const fit = (at, length) => Math.max(band, Math.min(length - band - size, at));
        const middle = Qt.point(fit(group.x + (group.width - size) / 2, width), fit(group.y + (group.height - size) / 2, height));
        const out = [target("stack", "", group.shown, middle.x, middle.y)];
        for (const [side, dx, dy] of [["left", -step, 0], ["right", step, 0], ["up", 0, -step], ["down", 0, step]])
            out.push(target("side", side, group.shown, middle.x + dx, middle.y + dy));
        return out;
    }

    function usable(list) {
        return list.filter(one => {
            const next = layout.placing(carried, one);
            return next !== "" && next !== layout.root;
        });
    }

    function land() {
        const unit = carried, next = drop.next;
        carried = "";
        if (next)
            Qt.callLater(settle, unit, next);
    }

    function settle(unit, next) {
        if (layout.change(next))
            Qt.callLater(area.focusTab, unit);
    }

    function said(choice) {
        if (!choice)
            return "";
        const name = area.title(choice.beside);
        if (choice.kind === "stack")
            return App.tr("With %1", name);
        if (choice.kind === "edge")
            return App.tr("Along this edge of the window");
        return {
            "left": App.tr("Left of %1", name),
            "right": App.tr("Right of %1", name),
            "up": App.tr("Above %1", name),
            "down": App.tr("Below %1", name)
        }[choice.side];
    }

    visible: carried !== ""
    onSpotChanged: aim()
    onCarriedChanged: aim()

    Rectangle {
        id: zone

        visible: !!drop.zone
        x: (drop.zone?.x ?? 0) + Look.e1
        y: (drop.zone?.y ?? 0) + Look.e1
        width: Math.max(0, (drop.zone?.width ?? 0) - Look.e2)
        height: Math.max(0, (drop.zone?.height ?? 0) - Look.e2)
        radius: Look.radius
        color: Qt.rgba(Look.moon.r, Look.moon.g, Look.moon.b, Look.coverVeilHover)
        border.color: Look.moon
        border.width: Look.ring

        Rectangle {
            anchors.horizontalCenter: parent.horizontalCenter
            y: Look.e3
            width: told.width + Look.e4
            height: told.implicitHeight + Look.e2
            radius: Look.radiusSmall
            color: Look.nightHigh
            border.color: Look.moon
            border.width: Look.line

            QQC2.Label {
                id: told

                anchors.centerIn: parent
                width: Math.min(implicitWidth, zone.width - Look.e4)
                horizontalAlignment: Text.AlignHCenter
                wrapMode: Text.Wrap
                text: drop.said(drop.choice)
                color: Look.moon
                font.pixelSize: Look.tBody
            }
        }
    }

    Repeater {
        model: drop.targets

        delegate: SceneDropTarget {
            drop: drop
        }
    }

    Rectangle {
        x: drop.spot.x + Look.e3
        y: drop.spot.y + Look.e3
        width: chip.implicitWidth + Look.e4
        height: chip.implicitHeight + Look.e2
        radius: Look.radiusSmall
        color: Look.nightHigh
        border.color: Look.moon
        border.width: Look.line

        QQC2.Label {
            id: chip

            anchors.centerIn: parent
            text: drop.carried ? drop.area.title(drop.carried) : ""
            font.pixelSize: Look.tBody
        }
    }
}
