import QtQuick
import org.kde.kirigami as Kirigami

Item {
    id: gizmo

    required property var page
    required property int index
    required property real ratio
    property var modelData: page.held.blank
    readonly property bool chosen: page.editor.selected === modelData.pointer
    readonly property bool movable: chosen && !modelData.locked
    readonly property bool brushing: ["/effects/", "/lighting"].some(part => (page.brush?.pointer ?? "").startsWith(modelData.pointer + part))
    readonly property var center: [modelData.offset[0] + modelData.position[0], modelData.offset[1] + modelData.position[1]]
    property var grabbed: null

    function pick() {
        const next = page.gizmos[index] ?? page.held.blank;
        if (next !== modelData)
            modelData = next;
    }

    function moved(point) {
        const dx = (point.scenePosition.x - point.scenePressPosition.x) / ratio;
        const dy = (point.scenePosition.y - point.scenePressPosition.y) / ratio;
        const angle = -modelData.angle * Math.PI / 180;
        return [dx, dy, dx * Math.cos(angle) - dy * Math.sin(angle), dx * Math.sin(angle) + dy * Math.cos(angle)];
    }

    function commit(field, value) {
        const rounded = Array.isArray(value) ? value.map(v => Math.round(v * 100) / 100) : Math.round(value * 100) / 100;
        page.run("layer", {
                "transform": {
                    "layer": modelData.id,
                    [field]: rounded,
                    "at": page.editor.time,
                    "replace": page.overwrite
                }
            }, true);
    }

    function grip(kind, to, snap) {
        knobs.grip(kind, to, snap);
    }

    function tapped(add) {
        parent.forceActiveFocus();
        const chosen = (page.gizmos ?? []).find(g => g.pointer === page.editor.selected);
        if (page.picking !== "parent" || !chosen || chosen.id === modelData.id) {
            page.layers.pick(modelData, add);
            parent.forceActiveFocus();
            return;
        }
        page.picking = "";
        page.run("layer", {
                "parent": {
                    "layer": chosen.id,
                    "parent": modelData.id,
                    "keep": true,
                    "at": page.editor.time,
                    "replace": page.overwrite
                }
            });
    }

    x: center[0] * ratio - width / 2
    y: center[1] * ratio - height / 2
    width: Math.max(8, modelData.extent[0] * ratio)
    height: Math.max(8, modelData.extent[1] * ratio)
    rotation: modelData.angle
    z: chosen ? 2 : 1
    containmentMask: SceneGizmoShape {
        gizmo: gizmo
    }

    Component.onCompleted: pick()

    Connections {
        target: gizmo.page.held

        function onListChanged() {
            gizmo.pick();
        }
    }

    SceneOutline {
        parent: gizmo.parent
        anchors.fill: parent
        z: gizmo.z
        corners: gizmo.modelData.outline
        ratio: gizmo.ratio
        chosen: gizmo.chosen
        dashed: false
        opacity: (gizmo.chosen ? 1 : 0.5) * (gizmo.modelData.corners ? 0.4 : 1)
    }

    HoverHandler {
        id: hover

        cursorShape: gizmo.page.picking === "parent" ? Qt.PointingHandCursor : Qt.ArrowCursor
    }

    Rectangle {
        anchors.fill: parent
        color: "transparent"
        border.color: Kirigami.Theme.highlightColor
        opacity: hover.hovered && !gizmo.chosen ? 0.5 : 0
    }

    SceneGizmoTap {
        gizmo: gizmo
    }

    SceneGizmoTap {
        gizmo: gizmo
        acceptedButtons: Qt.RightButton
    }

    SceneMapHeat {
        id: heat

        anchors.fill: parent
        page: gizmo.page
        visible: gizmo.brushing && gizmo.page.brush?.action === "paint_map"
    }

    DragHandler {
        property var points: []

        target: null
        enabled: gizmo.brushing
        onActiveChanged: {
            if (!active && points.length > 0)
                gizmo.page.run(gizmo.page.brush.action ?? "paint_mask", heat.stroke(gizmo.page.brush, points));
            points = [];
        }
        onCentroidChanged: {
            if (active)
                points = points.concat([[centroid.position.x / gizmo.width, centroid.position.y / gizmo.height]]);
        }
    }

    DragHandler {
        target: null
        enabled: !gizmo.modelData.locked && !gizmo.brushing && gizmo.page.picking === "" && gizmo.page.mesh.rigging !== gizmo.modelData.id && !gizmo.page.mesh.posing(gizmo.modelData.id)
        acceptedModifiers: Qt.NoModifier | Qt.AltModifier
        onActiveChanged: {
            gizmo.page.moving = active;
            if (!active)
                gizmo.page.editor.hold(false);
            if (active && !gizmo.chosen)
                gizmo.page.editor.select(gizmo.modelData.pointer);
            if (active)
                gizmo.parent.forceActiveFocus();
            gizmo.grabbed = gizmo.modelData.handles?.pivot ?? gizmo.modelData.pivot;
        }
        onCentroidChanged: {
            const delta = gizmo.moved(centroid);
            if (active && gizmo.grabbed)
                gizmo.grip("move", [gizmo.grabbed[0] + delta[0], gizmo.grabbed[1] + delta[1]], !(centroid.modifiers & Qt.AltModifier));
        }
    }

    SceneGizmoKnobs {
        id: knobs

        parent: gizmo.parent
        anchors.fill: parent
        z: gizmo.z + 4
        gizmo: gizmo
    }

    SceneGizmoLean {
        gizmo: gizmo
    }

    SceneEmitters {
        anchors.fill: parent
        z: 3
        visible: gizmo.chosen
        page: gizmo.page
        pointer: gizmo.chosen ? gizmo.modelData.pointer : ""
    }

    SceneBones {
        gizmo: gizmo
        visible: gizmo.page.mesh.rigging === gizmo.modelData.id || gizmo.page.mesh.posing(gizmo.modelData.id)
    }
}
