import QtQuick

TapHandler {
    id: tap

    required property Item gizmo

    function above(scene) {
        const all = Array.from(gizmo.parent.children).filter(other => other !== gizmo && other.tapped !== undefined && other.index > gizmo.index);
        const under = all.filter(other => other.contains(other.mapFromItem(null, scene.x, scene.y)));
        return gizmo.chosen ? under.reduce((top, other) => !top || other.index > top.index ? other : top, null) ?? gizmo : gizmo;
    }

    gesturePolicy: TapHandler.ReleaseWithinBounds
    onTapped: eventPoint => {
        const target = above(eventPoint.scenePosition);
        if (acceptedButtons !== Qt.RightButton)
            return target.tapped((point.modifiers & Qt.ControlModifier) !== 0);
        const at = target.mapFromItem(null, eventPoint.scenePosition.x, eventPoint.scenePosition.y);
        target.page.layers.menu.onLayer(target.modelData, target, at.x, at.y);
    }
}
